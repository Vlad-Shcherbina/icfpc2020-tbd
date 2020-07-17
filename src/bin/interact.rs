use std::io::{self, Write};
use tbd::{API_KEY, squiggle::*};

const API_ENDPOINT: &str = "https://icfpc2020-api.testkontur.ru/aliens/send";

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().ok().expect("flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read from stdin");
        let input = input.trim();

        let data = match Data::from_str(input) {
            Some(d) => d,
            None => {
                println!("couldnt parse input");
                continue;
            }
        };

        let modulated = modulate(data);
        // convert to string
        let modulated = modulated.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join("");

        println!("sending {}", modulated);

        let response = ureq::post(API_ENDPOINT)
            .query("apiKey", API_KEY)
            .send_string(&modulated);

        if !response.ok() {
            println!("got status {} {}", response.status(), response.status_text());
            println!("is api key correct?");
            continue;
        }

        let response = response.into_string().expect("valid response");

        println!("got response {}", response);
        let response = bytes_to_squiggle(response.as_bytes()).expect("response is 01");
        let demodulated = demodulate(response.iter());
        let demodulated = demodulated.expect("valid demodulate").0;
        println!("{}", demodulated.to_string());
    }
}
