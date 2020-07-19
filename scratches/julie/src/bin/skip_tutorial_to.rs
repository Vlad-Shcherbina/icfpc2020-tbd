use::tbd::tutorial;

fn ff() {
    let args: Vec<String> = std::env::args().collect();
    let lesson: i32 = if args.len() > 1 { args[1].parse::<i32>().unwrap() } else { -1 };
    let protocol = tbd::ufolang::Protocol::load_galaxy();
    tutorial::fast_forward_training(lesson, &protocol);
}

 
fn main() {
    std::thread::Builder::new().stack_size(100 * 1024 * 1024)
        .spawn(ff).unwrap()
        .join().unwrap();
}
