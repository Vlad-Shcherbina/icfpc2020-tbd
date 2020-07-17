use std::convert::TryInto;

fn squiggle_from_string(s: &str) -> Vec<u8> {
    s.chars().map(|c| c.to_digit(10).unwrap().try_into().unwrap()).collect()
}