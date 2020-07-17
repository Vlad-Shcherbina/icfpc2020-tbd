use aleksejs::squiggle::*;

fn main() {
    println!("{:?}", demodulate((vec!{1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0}).iter()));
    println!("{:?}", modulate(Data::Cons(
        Box::new(Data::Number(Sign::Plus, 3)),
        Box::new(Data::Nil)
    )));
}
