use tbd::ufolang::Protocol;
use tbd::{squiggle::Data, ufolang::{eval_multidraw, Value::*}, project_path, png_files::matrices_to_png, webapi::Endpoint};
use std::rc::Rc;
use std::io::BufRead;
use std::collections::HashSet;


fn explore(protocol: &Protocol, seen: &mut HashSet<String>, state: Data) {
    let state_str = state.to_string();
    seen.insert(state_str.clone());

    println!("i am in {:#?}", state);
    for x in -10..10 {
        for y in -10..10 {
            let result = protocol.interact(state.clone(), Data::make_cons(x, y), &Endpoint::Proxy);
            let result_state_str = result.final_state.to_string();
            if !seen.contains(&result_state_str) {
                println!("{} -> {} {} -> {}", state_str, x, y, result_state_str);
                explore(protocol, seen, result.final_state);
            }
        }
    }
}

fn main1() {
    let protocol = Protocol::load_galaxy();
    let mut seen_states = HashSet::new();

    explore(&protocol, &mut seen_states, Data::Nil);
}

fn main() {
    std::thread::Builder::new().stack_size(100 * 1024 * 1024)
        .spawn(main1).unwrap()
        .join().unwrap();
}
