use tbd::project_path;
use tbd::ufolang::{Protocol, eval_multidraw, InteractResult};
use tbd::squiggle::Data;

use warp::Filter;
use serde::{Deserialize, Serialize};
use tokio::runtime::Builder;
use tokio::time::delay_for;
use std::time::Duration;
use tbd::{webapi::Endpoint, png_files::matrices_to_png};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize)]
struct ClickParams {
    x: i128,
    y: i128,
    state: String
}

fn data_try_to_coords(value: &Data) -> Option<(i128, i128)> {
    match value {
        Data::Cons(a, b) => Some((a.try_as_number()?, b.try_as_number()?)),
        _ => None
    }
}

#[derive(Serialize, Deserialize)]
struct RequestResponse {
    pretty_request: String,
    pretty_response: String,
}

#[derive(Deserialize, Serialize)]
struct ClickResponse {
    state: String,
    pretty_state: String,
    pixels: Vec<Vec<(i128, i128)>>,
    network_history: Vec<RequestResponse>,
}

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
    //     |x| x.into_vec().iter().map(|y| data_try_to_coords(y).unwrap()).collect()
    // ).collect();

    let pixels = result.data_out_to_multipledraw.into_vec();
    let pixels: Vec<Vec<_>> = pixels.into_iter().map(|x| x.into_vec()).collect();
    let pixels = pixels.iter().map(|x| x.iter().map(|y| data_try_to_coords(y).unwrap()).collect()).collect();

    ClickResponse {
        state: result.final_state.to_string(),
        pretty_state: result.final_state.to_pretty_string(),
        pixels: pixels,
        network_history: result.network_history.iter().map(|rr| {
            RequestResponse {
                pretty_request: rr.request.to_pretty_string(),
                pretty_response: rr.response.to_pretty_string(),
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
