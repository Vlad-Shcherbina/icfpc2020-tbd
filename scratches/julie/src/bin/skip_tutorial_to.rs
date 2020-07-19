use::tbd::tutorial;


fn main() {
    std::thread::Builder::new().stack_size(100 * 1024 * 1024)
        .spawn(tutorial::fast_forward_training).unwrap()
        .join().unwrap();
}
