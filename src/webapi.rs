// use tbd::{API_KEY, squiggle::*};
use crate::squiggle::*;
use crate::API_KEY;

const API_ENDPOINT: &str = "https://icfpc2020-api.testkontur.ru/aliens/send";

pub fn aliens_send(data: Data) -> Data {
    let modulated = modulate(data);
    // convert to string
    let modulated = modulated.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join("");

    println!("sending {}", modulated);

    let response = ureq::post(API_ENDPOINT)
        .query("apiKey", API_KEY)
        .send_string(&modulated);

    if !response.ok() {
        // TODO: if there is rate limiting, do something about it.
        println!("got status {} {}", response.status(), response.status_text());
        println!("is api key correct?");
        panic!();
    }

    let response = response.into_string().expect("valid response");

    println!("got response {}", response);
    let response = bytes_to_squiggle(response.as_bytes()).expect("response is 01");
    let demodulated = demodulate(response.iter());
    demodulated.expect("valid demodulate").0
}
