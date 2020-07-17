use tbd::squiggle::*;

fn main() {
    println!("{:?}", demodulate(bytes_to_squiggle(b"110110000100").unwrap().iter()));
    println!("{:?}", modulate(Data::Cons(
        Box::new(Data::Number(Sign::Plus, 3)),
        Box::new(Data::Nil)
    )));
}
