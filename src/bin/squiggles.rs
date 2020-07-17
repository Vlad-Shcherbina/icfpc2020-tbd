use std::io::{self, Write};
use tbd::squiggle::*;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().ok().expect("flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("read from stdin");
        let input = input.trim();

        if input.starts_with("0") || input.starts_with("1") {
            // modulated input given, demodulate
            let input = bytes_to_squiggle(input.as_bytes()).expect("input all 01");
            let demodulated = demodulate(input.iter());
            let demodulated = demodulated.expect("valid demodulate").0;
            println!("= {}", demodulated.to_string());
        } else {
            // demodulated input given, modulate
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

            println!("= {}", modulated);
        }
    }
}
