// THIS IS A LOW-LEVEL EXAMPLE.
// SEE example_player.rs INSTEAD.

use tbd::{webapi::Endpoint, squiggle::Data};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3);
    let server_url = args[1].clone();
    let player_key = &args[2];
    eprintln!("Server URL: {}", server_url);
    eprintln!("Player key: {}", player_key);
    let endpoint = Endpoint::SubmissionServer { server_url };
    let player_key: i128 = player_key.parse().unwrap();

    // JOIN
    let req = Data::make_list3(2, player_key, Data::Nil);
    let resp = endpoint.aliens_send(req);
    dbg!(resp);

    // let resp = ureq::post(&server_url).send_string(player_key);
    // assert!(resp.ok(), "{:?}", resp);
    // println!("Response: {:?}", resp);
    // println!("Headers:");
    // for h in resp.headers_names() {
    //     println!("  {}: {:?}", h, resp.header(&h).unwrap());
    // }
    // let body = resp.into_string().unwrap();
    // println!("Body: {:?}", body);
}
