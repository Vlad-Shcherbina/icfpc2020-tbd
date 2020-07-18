use std::io::{self, Write};
use tbd::{squiggle::*, webapi::aliens_send};

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

        let response = aliens_send(data);
        println!("{}", response.to_string());
    }
}
