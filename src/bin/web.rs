use tbd::project_path;
use tbd::ufolang::{Protocol, eval_multidraw, InteractResult};
use tbd::squiggle::Data;

use warp::Filter;
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;
use tokio::time::delay_for;
use std::time::Duration;
use tbd::{webapi::Endpoint, png_files::matrices_to_png, uforest::{Commands, GameResponse}};
use std::collections::hash_map::DefaultHasher;
use std::{convert::TryFrom, hash::{Hash, Hasher}};

#[derive(Deserialize, Serialize)]
struct ClickParams {
    x: i128,
    y: i128,
    state: String
}

#[derive(Serialize, Deserialize)]
struct RequestResponse {
    pretty_request: String,
    pretty_response: String,

    response_as_game_response: String,
    //  - empty string for non-game responses or
    //  - pretty multiline representation or
    //  - error message

    request_as_commands: String,  // same idea
}

#[derive(Deserialize, Serialize)]
struct ClickResponse {
    state: String,
    pretty_state: String,
    pixels: Vec<Vec<(i128, i128)>>,
    network_history: Vec<RequestResponse>,
}

#[allow(dead_code)]
fn save_pics(result: &InteractResult) {
    let matrices = eval_multidraw(result.data_out_to_multipledraw.clone());
    let mut hasher = DefaultHasher::new();
    result.final_state.hash(&mut hasher);
    let step_dir = format!("outputs/galaxy--{:x}", hasher.finish());
    matrices_to_png(&matrices, project_path(&step_dir));
    std::fs::write(project_path(&step_dir).join("state.txt"), result.final_state.to_string()).unwrap();
}

fn process_click(click: &ClickParams) -> ClickResponse {
    let protocol = Protocol::load_galaxy();
    let state = match Data::from_str(&click.state) {
        Some(v) => v,
        None => return ClickResponse{
            state: String::from("error"),
            pretty_state: String::from("error"),
            pixels: vec![],
            network_history: vec![],
        }
    };

    let result = protocol.interact(state, Data::make_cons(click.x, click.y), &Endpoint::Proxy);
    // save_pics(&result);

    // let pixels = result.data_out_to_multipledraw.into_vec().iter().map(
    //     |x| x.into_vec().iter().map(|y| y.try_to_coords().unwrap()).collect()
    // ).collect();

    let pixels = result.data_out_to_multipledraw.try_into_vec().unwrap();
    let pixels: Vec<Vec<_>> = pixels.into_iter().map(|x| x.try_into_vec().unwrap()).collect();
    let pixels = pixels.iter().map(|x| x.iter().map(|y| y.try_to_coords().unwrap()).collect()).collect();

    ClickResponse {
        state: result.final_state.to_string(),
        pretty_state: result.final_state.to_pretty_string(),
        pixels: pixels,
        network_history: result.network_history.iter().map(|rr| {
            let mut response_as_game_response = String::new();
            if let Some(parts) = rr.response.clone().try_into_vec() {
                if parts.len() == 4 && parts[0].try_as_number().is_some() {
                    response_as_game_response = match GameResponse::try_from(rr.response.clone()) {
                        Ok(gr) => format!("{:#?}", &gr),
                        Err(e) => format!("can't parse as GameResponse: {}", e),
                    };
                }
            }
            let mut request_as_commands = String::new();
            if let Some(parts) = rr.request.clone().try_into_vec() {
                if parts.len() == 3 &&
                   parts[0].try_as_number() == Some(4) &&
                   parts[1].try_as_number().is_some() {
                    request_as_commands = match Commands::try_from(parts[2].clone()) {
                        Ok(c) => {
                            let data = Data::from(c.clone());
                            if data == parts[2] {
                                format!("{:#?}", c)
                            } else {
                                format!("Commands parse-serialize round-trip failed\n{:?}\n{:?}\ngo to uforest.rs and fix it\n{}",
                                    data, parts[2], "!".repeat(256))
                            }
                        }
                        Err(e) => format!("can't parse as Commands: {}", e),
                    };
                }
            }
            RequestResponse {
                pretty_request: rr.request.to_pretty_string(),
                pretty_response: rr.response.to_pretty_string(),
                response_as_game_response,
                request_as_commands,
            }
        }).collect(),
    }
}

async fn server_main() {
    delay_for(Duration::from_millis(100)).await;

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(project_path("data/ui.html")));

    let click = warp::post()
        .and(warp::path("click"))
        .and(warp::body::content_length_limit(1024 * 1024))
        .and(warp::body::json())
        .map(|click: ClickParams| {
            warp::reply::json(&process_click(&click))
        });

    let routes = index.or(click);

    println!("serving at http://127.0.0.1:22009 ...");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 22009))
        .await;
}

fn main() {
    let mut rt = Builder::new()
        .threaded_scheduler()
        .enable_all()
        .thread_stack_size(100 * 1024 * 1024)
        .build().unwrap();
    rt.block_on(server_main());
}
