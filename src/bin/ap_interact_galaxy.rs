use tbd::ufolang::Protocol;
use tbd::{squiggle::Data, ufolang::{eval_multidraw, Value::*}, project_path, png_files::matrices_to_png};
use std::rc::Rc;

fn main1() {
    let protocol = Protocol::load_galaxy();

    let result = protocol.interact(Data::Nil, Data::make_cons(0, 0));
    dbg!(&result.final_state);
    let matrices = eval_multidraw(result.data_out_to_multipledraw);
    matrices_to_png(&matrices, project_path("outputs/galaxy00"));
}

fn main() {
    std::thread::Builder::new().stack_size(100 * 1024 * 1024)
        .spawn(main1).unwrap()
        .join().unwrap();
}
