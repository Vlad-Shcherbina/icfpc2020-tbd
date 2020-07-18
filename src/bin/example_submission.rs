use tbd::{webapi::Endpoint, ufolang::Protocol, squiggle::Data};

fn main() {
    let protocol = Protocol::load_galaxy();

    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3);
    let server_url = args[1].clone();
    let player_key = &args[2];
    println!("Server URL: {}", server_url);
    println!("Player key: {}", player_key);
    let endpoint = &Endpoint::SubmissionServer { url: server_url };
    let player_key: i128 = player_key.parse().unwrap();

    let mut state = Data::Nil;

    // JOIN
    let data_in = Data::make_list3(2, player_key, Data::Nil);
    let result = protocol.interact(state, data_in, endpoint);
    dbg!(&result);

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
