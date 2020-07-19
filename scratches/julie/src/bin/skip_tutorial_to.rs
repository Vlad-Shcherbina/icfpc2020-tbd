use tbd::ufolang::Protocol;
use tbd::{squiggle::Data, ufolang::{eval_multidraw }, project_path, png_files::matrices_to_png, webapi::Endpoint};

#[allow(unused)]
fn main_simple() {
    let protocol = Protocol::load_galaxy();
    let result = protocol.interact(Data::Nil, Data::make_cons(0, 0), &Endpoint::Proxy);
    dbg!(&result.final_state);
    dbg!(&result.data_out_to_multipledraw);
    let matrices = eval_multidraw(result.data_out_to_multipledraw);
    matrices_to_png(&matrices, project_path("outputs/galaxyXX"));
}

fn fast_forward_training() {
    let args: Vec<String> = std::env::args().collect();
    let n: i32 = if args.len() > 1 { args[1].parse::<i32>().unwrap() } else { 5 };
    let mut state = Data::from_str("(5, ((2, (0, (nil, (nil, (nil, (nil, (nil, (54179, nil)))))))), (9, (nil, nil))))").unwrap();

    let protocol = Protocol::load_galaxy();
    state = protocol.interact(state, Data::make_cons(20, 0), &Endpoint::Proxy).final_state;
    println!("0:\n{}\n", state.to_string());
    if n == 0 { 
        return;      
    }

    for i in 0..9 {
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    }
    println!("1:\n{}\n", state.to_string());
    if n == 1 { 
        return;      
    }

    for i in 0..7 {
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    }
    state = protocol.interact(state, Data::make_cons(22, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(16, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    println!("2:\n{}\n", state.to_string());
    if n == 2 { 
        return;      
    }

    state = protocol.interact(state, Data::make_cons(16, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(16, -7), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(11, -7), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    
    state = protocol.interact(state, Data::make_cons(20, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(20, -7), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(15, -12), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    
    state = protocol.interact(state, Data::make_cons(30, 5), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(23, 5), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    
    println!("3:\n{}\n", state.to_string());
    if n == 3 { 
        return;      
    }

    let mut v = 0;
    let mut x = 0;
    for _ in 0..3 {
        state = protocol.interact(state, Data::make_cons(16 + x, 0), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(16 + x, -7), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(11 + x, -7), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
        v += 1;
        x += v;
    }
    for _ in 0..4 {
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    }
    state = protocol.interact(state, Data::make_cons(34, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(27, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    println!("4:\n{}\n", state.to_string());
    if n == 4 { 
        return;      
    }

    state = protocol.interact(state, Data::make_cons(0, 16), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 10), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(-5, 4), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;

    state = protocol.interact(state, Data::make_cons(2, 18), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(2, 12), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(2, 6), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;

    state = protocol.interact(state, Data::make_cons(4, 22), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(4, 16), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(4, 10), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;

    state = protocol.interact(state, Data::make_cons(6, 28), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(6, 22), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(6, 16), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;

    state = protocol.interact(state, Data::make_cons(8, 36), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(8, 30), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(8, 24), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;

    state = protocol.interact(state, Data::make_cons(10, 46), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(10, 40), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(10, 34), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;


    state = protocol.interact(state, Data::make_cons(15, 76), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(18, 76), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    println!("5:\n{}\n", state.to_string());
    if n == 5 { 
        return;      
    }

}

fn main() {
    std::thread::Builder::new().stack_size(100 * 1024 * 1024)
        .spawn(fast_forward_training).unwrap()
        .join().unwrap();
}
