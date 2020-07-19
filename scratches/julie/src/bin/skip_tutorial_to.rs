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

fn fast_forward_training() -> Data {
    let args: Vec<String> = std::env::args().collect();
    let n: i32 = if args.len() > 1 { args[1].parse::<i32>().unwrap() } else { -1 };
    let mut state = Data::from_str("(5, ((2, (0, (nil, (nil, (nil, (nil, (nil, (54179, nil)))))))), (9, (nil, nil))))").unwrap();
    let mut buf = String::new();

    let protocol = Protocol::load_galaxy();
    
    println!("1:\n{}\n", state.to_string());
    if n == 1 { 
        return state;      
    }
    if n == 0 { 
        print!("Continue? ");
        std::io::stdin().read_line(&mut buf);
        println!("Working...");
    }

    state = protocol.interact(state, Data::make_cons(20, 0), &Endpoint::Proxy).final_state;
    for _ in 0..8 {
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    }
    println!("2:\n{}\n", state.to_string());
    if n == 3 { 
        return state;
    }
    if n == 0 { 
        print!("Continue? ");
        std::io::stdin().read_line(&mut buf);
        println!("Working...");
    }


    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    for _ in 0..7 {
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    }
    state = protocol.interact(state, Data::make_cons(22, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(16, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    println!("3:\n{}\n", state.to_string());
    if n == 3 { 
        return state;      
    }
    if n == 0 { 
        print!("Continue? ");
        std::io::stdin().read_line(&mut buf);
        println!("Working...");
    }


    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(16, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(16, -7), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(11, -7), &Endpoint::Proxy).final_state;
    for _ in 0..4 {
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    }
    
    state = protocol.interact(state, Data::make_cons(20, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(20, -7), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(15, -12), &Endpoint::Proxy).final_state;
    for _ in 0..5 {
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    }
    
    state = protocol.interact(state, Data::make_cons(30, 5), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(23, 5), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    
    println!("4:\n{}\n", state.to_string());
    if n == 4 { 
        return state;      
    }
    if n == 0 { 
        print!("Continue? ");
        std::io::stdin().read_line(&mut buf);
        println!("Working...");
    }
    
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
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
    println!("5:\n{}\n", state.to_string());
    if n == 5 { 
        return state;      
    }
    if n == 0 { 
        print!("Continue? ");
        std::io::stdin().read_line(&mut buf);
        println!("Working...");
    }


    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 16), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 10), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(-5, 4), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;

    // x = 8
    // v = 10
    // let mut y = 22
    // 3

    let mut y = 2;
    v = 2;
    x = 2;
    for _ in 1..6 {
        println!("{} {}", x, y);
        state = protocol.interact(state, Data::make_cons(x, 16 + y), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(x, 10 + y), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(x, 4 + y), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
        x += 2;
        v += 2;
        y += v;
    }

    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;


    state = protocol.interact(state, Data::make_cons(15, 76), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(18, 76), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    // state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    println!("6:\n{}\n", state.to_string());
    if n == 6 { 
        return state;
    }
        if n == 0 { 
        print!("Continue? ");
        std::io::stdin().read_line(&mut buf);
        println!("Working...");
    }

    state
}

fn main() {
    std::thread::Builder::new().stack_size(100 * 1024 * 1024)
        .spawn(fast_forward_training).unwrap()
        .join().unwrap();
}
