use tbd::ufolang::Protocol;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3);
    let server_url = &args[1];
    let player_key = &args[2];

    let _protocol = Protocol::load_galaxy();

    println!("Server URL: {}", server_url);
    println!("Player key: {}", player_key);
    let resp = ureq::post(&server_url).send_string(player_key);
    assert!(resp.ok(), "{:?}", resp);
    println!("Response: {:?}", resp);
    println!("Headers:");
    for h in resp.headers_names() {
        println!("  {}: {:?}", h, resp.header(&h).unwrap());
    }
    let body = resp.into_string().unwrap();
    println!("Body: {:?}", body);
}
