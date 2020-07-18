use tbd::ufolang::Protocol;
use tbd::{squiggle::Data, ufolang::{eval_multidraw}, project_path, png_files::matrices_to_png};
use std::io::BufRead;

#[allow(unused)]
fn main_simple() {
    let protocol = Protocol::load_galaxy();
    let result = protocol.interact(Data::Nil, Data::make_cons(0, 0));
    dbg!(&result.final_state);
    dbg!(&result.data_out_to_multipledraw);
    let matrices = eval_multidraw(result.data_out_to_multipledraw);
    matrices_to_png(&matrices, project_path("outputs/galaxyXX"));
}

fn main1() {
    let protocol = Protocol::load_galaxy();

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut state = Data::Nil;

    let mut step = 0;
    println!("enter space-separated coordinates, e.g. '10 20'");
    loop {
        println!();
        println!("Click where?");
        let line = lines.next().unwrap().unwrap();
        let mut it = line.split_whitespace();
        let x: i128 = 0; //it.next().unwrap().parse().unwrap();
        let y: i128 = 0; //it.next().unwrap().parse().unwrap();

        let result = protocol.interact(state, Data::make_cons(x, y));
        dbg!(&result.final_state);
        dbg!(&result.data_out_to_multipledraw);
        let matrices = eval_multidraw(result.data_out_to_multipledraw);
        let step_dir = format!("outputs/galaxy{}", step);
        matrices_to_png(&matrices, project_path(&step_dir));
        println!("see {}", step_dir);

        state = result.final_state;
        step += 1;
    }
}

fn main() {
    std::thread::Builder::new().stack_size(100 * 1024 * 1024)
        .spawn(main_simple).unwrap()
        .join().unwrap();
}
