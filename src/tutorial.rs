use crate::ufolang::Protocol;
use crate::{squiggle::Data, ufolang::{eval_multidraw }, project_path, png_files::matrices_to_png, webapi::Endpoint};


#[allow(unused)]
fn main_simple() {
    let protocol = Protocol::load_galaxy();
    let result = protocol.interact(Data::Nil, Data::make_cons(0, 0), &Endpoint::Proxy);
    dbg!(&result.final_state);
    dbg!(&result.data_out_to_multipledraw);
    let matrices = eval_multidraw(result.data_out_to_multipledraw);
    matrices_to_png(&matrices, project_path("outputs/galaxyXX"));
}

pub fn fast_forward_training(lesson: i32, protocol: &Protocol) -> Vec<Data> {
    let mut state = Data::from_str("(5, ((2, (0, (nil, (nil, (nil, (nil, (nil, (54179, nil)))))))), (9, (nil, nil))))").unwrap();
    let mut buf = String::new();
    let mut states: Vec<Data> = Vec::new();
    
    println!("1:\n{}\n", state.to_string());
    states.push(state.clone());
    if lesson == 1 { 
        return states;      
    }
    if lesson == 0 { 
        println!("Press Enter to continue");
        std::io::stdin().read_line(&mut buf).unwrap();
        println!("Working...");
    }

    state = protocol.interact(state, Data::make_cons(20, 0), &Endpoint::Proxy).final_state;
    for _ in 0..8 {
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    }
    println!("2:\n{}\n", state.to_string());
    states.push(state.clone());
    if lesson == 2 { 
        return states;
    }
    if lesson == 0 { 
        println!("Press Enter to continue");
        std::io::stdin().read_line(&mut buf).unwrap();
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
    states.push(state.clone());
    if lesson == 3 { 
        return states;
    }
    if lesson == 0 { 
        println!("Press Enter to continue");
        std::io::stdin().read_line(&mut buf).unwrap();
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
    states.push(state.clone());
    if lesson == 4 { 
        return states;      
    }
    if lesson == 0 { 
        println!("Press Enter to continue");
        std::io::stdin().read_line(&mut buf).unwrap();
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
    states.push(state.clone());
    if lesson == 5 { 
        return states;
    }
    if lesson == 0 { 
        println!("Press Enter to continue");
        std::io::stdin().read_line(&mut buf).unwrap();
        println!("Working...");
    }


    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 16), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 10), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(-5, 4), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;


    let mut y = 2;
    v = 2;
    x = 2;
    for _ in 1..6 {
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
    state = protocol.interact(state, Data::make_cons(8, 76), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    println!("6:\n{}\n", state.to_string());
    states.push(state.clone());
    if lesson == 6 { 
        return states;
    }
    if lesson == 0 { 
        println!("Press Enter to continue");
        std::io::stdin().read_line(&mut buf).unwrap();
        println!("Working...");
    }

    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    for i in 0..3 {
        state = protocol.interact(state, Data::make_cons(16, 0), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(16, 7), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(48, 0), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;    
        if i == 0 {
            state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;    
        }
    }
    println!("7:\n{}\n", state.to_string());
    states.push(state.clone());
    if lesson == 7 { 
        return states;
    }
    if lesson == 0 { 
        println!("Press Enter to continue");
        std::io::stdin().read_line(&mut buf).unwrap();
        println!("Working...");
    }
    

    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    for i in 0..4 {
        state = protocol.interact(state, Data::make_cons(16, 0), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(16, 7), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(48, 0), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;    
    }
    println!("8:\n{}\n", state.to_string());
    states.push(state.clone());
    if lesson == 8 { 
        return states;
    }
    if lesson == 0 { 
        println!("Press Enter to continue");
        std::io::stdin().read_line(&mut buf).unwrap();
        println!("Working...");
    }
    
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;

    for (x, y) in vec![(41, 0), (38, 0), (30, 14), (16, 15)] {
        state = protocol.interact(state, Data::make_cons(16, 0), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(16, 7), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(x, y), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;         
    }
    println!("9:\n{}\n", state.to_string());
    states.push(state.clone());
    if lesson == 9 { 
        return states;
    }
    if lesson == 0 { 
        println!("Press Enter to continue");
        std::io::stdin().read_line(&mut buf).unwrap();
        println!("Working...");
    }

    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(16, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(16, 7), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(48, 17), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(16, 0), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(16, -7), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(11, -2), &Endpoint::Proxy).final_state;
    state = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy).final_state;
    println!("10:\n{}\n", state.to_string());
    states.push(state.clone());
    if lesson == 10 { 
        return states;
    }
    if lesson == 0 { 
        println!("Press Enter to continue");
        std::io::stdin().read_line(&mut buf).unwrap();
        println!("Working...");
    }

    for (x, y) in vec![(0, 0), (0, 16), (7, 16), (13, 8), (13, 8), (13, 8), (13, 8), 
                        (25, 8), (25, 8), (25, 8), (25, 8), (25, 8), (40, 16), (0, 0), 
                        (0, 16), (0, 9), (5, 4), (0, 16), (0, 16), (0, 9), (-5, 4), (0, 0), 
                        (1, 17), (1, 10), (-4, 5), (0, 0), (0, 0), (0, 0), (-4, 20), 
                        (-11, 20), (7, 23), (0, 23), (0, 0)] {
        state = protocol.interact(state, Data::make_cons(x, y), &Endpoint::Proxy).final_state;
    }
    println!("11:\n{}\n", state.to_string());
    states.push(state.clone());
    if lesson == 11 { 
        return states;
    }
    if lesson == 0 { 
        println!("Press Enter to continue");
        std::io::stdin().read_line(&mut buf).unwrap();
        println!("Working...");
    }

    states
}