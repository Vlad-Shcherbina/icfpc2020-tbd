// use tbd::{API_KEY, squiggle::*};
use crate::squiggle::*;
use crate::API_KEY;

const API_PROXY: &str = "https://icfpc2020-api.testkontur.ru";

pub enum Endpoint {
    NoComms,
    Proxy,
    SubmissionServer {
        server_url: String,
    },
}

impl Endpoint {
    pub fn aliens_send(&self, data: Data) -> Data {
        eprintln!("sending {:?}", data);
        let modulated = modulate(data);
        // convert to string
        let modulated = modulated.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join("");

        let mut retry_delays = vec![0.5, 2.0, 8.0].into_iter();
        let response = loop {
            let response = match self {
                Endpoint::NoComms => panic!(),
                Endpoint::Proxy => {
                    ureq::post(&format!("{}/aliens/send", API_PROXY))
                        .query("apiKey", API_KEY)
                        .send_string(&modulated)
                }
                Endpoint::SubmissionServer { server_url } => {
                    ureq::post(&format!("{}/aliens/send", server_url))
                        .send_string(&modulated)
                }
            };

            if response.ok() {
                break response;
            } else {
                eprintln!("got status {} {}", response.status(), response.status_text());
                if let Some(delay) = retry_delays.next() {
                    eprintln!("retrying in {} s...", delay);
                    std::thread::sleep(std::time::Duration::from_secs_f64(delay));
                } else {
                    eprintln!("is api key correct?");
                    panic!();
                }
            }
        };

        let response = response.into_string().expect("valid response");

        let response = bytes_to_squiggle(response.as_bytes()).expect("response is 01");
        let demodulated = demodulate(response.iter());
        let result = demodulated.expect("valid demodulate").0;
        eprintln!("got response {:?}", result);
        result
    }
}