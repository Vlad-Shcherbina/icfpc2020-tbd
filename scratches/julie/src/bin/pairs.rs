use tbd::ufolang::Protocol;
use tbd::{squiggle::Data, ufolang::{eval_multidraw, InteractResult}, project_path, png_files::matrices_to_png, webapi::Endpoint};
use std::io::BufRead;

#[allow(unused)]
fn main_simple() {
    let protocol = Protocol::load_galaxy();
    let result = protocol.interact(Data::Nil, Data::make_cons(0, 0), &Endpoint::Proxy);
    dbg!(&result.final_state);
    dbg!(&result.data_out_to_multipledraw);
    let matrices = eval_multidraw(result.data_out_to_multipledraw);
    matrices_to_png(&matrices, project_path("outputs/galaxyXX"));
}

fn to_coord(n: i128) -> (i128, i128) {
    ((n / 8) * 6, (n % 8) * 6)
}

// готовые пары: (номера) значения
// (0, 2) 122 410    поворот на 90
// (1, 26) 203 203   тождественное 
// (7, 13) 77 329    отражение отн-но горизонтали 
// (10, 42) 456, 201 отражение отн-но побочной диаг 
// (20, 34) 384, 192 отражение отн-но вертикали
// (24, 28) 34, 160  ???
// (25, 43) 437, 497 отражение отн-но главной диаг
// (31, 33) 428, 107 поворот на 180
fn click_all_pairs() {
    let init_state = Data::from_str("(4, ((1, ((122, (203, (410, (164, (444, (484, (202, (77, (251, \
        (56, (456, (435, (28, (329, (257, (265, (501, (18, (190, (423, \
        (384, (434, (266, (69, (34, (437, (203, (152, (160, (425, (245, \
        (428, (99, (107, (192, (372, (346, (344, (169, (478, (393, (502, \
        (201, (497, (313, (32, (281, (510, (436, (22, (237, (80, (325, \
        (405, (184, (358, (57, (276, (359, (189, (284, (277, (198, (244, \
        nil)))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))), \
        (-1, (0, (nil, nil))))), (0, (nil, nil))))").unwrap();
    
    let mut state = init_state.clone();
    let protocol = Protocol::load_galaxy();
    for (i, j) in [(1, 26), (7, 13), (10, 42), (20, 34), (24, 28), (25, 43), (31, 33)].iter() {
        let (y1, x1) = to_coord(*i);
        let (y2, x2) = to_coord(*j);
        state = protocol.interact(state, Data::make_cons(x1, y1), &Endpoint::Proxy).final_state;
        state = protocol.interact(state, Data::make_cons(x2, y2), &Endpoint::Proxy).final_state;
    }
    state = protocol.interact(state, Data::make_cons(12, 0), &Endpoint::Proxy).final_state;
    println!("{}", state.to_string());
    // the only one not clicked is 0, 0
    // prints the state, that is one (0,0) click to finish
    
    let result = protocol.interact(state, Data::make_cons(0, 0), &Endpoint::Proxy);
    let matrices = eval_multidraw(result.data_out_to_multipledraw);
    matrices_to_png(&matrices, project_path("outputs/galaxyXX"));
}



#[allow(unused)]
fn find_all_pairs() {
    let init_state = Data::from_str("(4, ((1, ((122, (203, (410, (164, (444, (484, (202, (77, (251, \
                    (56, (456, (435, (28, (329, (257, (265, (501, (18, (190, (423, \
                    (384, (434, (266, (69, (34, (437, (203, (152, (160, (425, (245, \
                    (428, (99, (107, (192, (372, (346, (344, (169, (478, (393, (502, \
                    (201, (497, (313, (32, (281, (510, (436, (22, (237, (80, (325, \
                    (405, (184, (358, (57, (276, (359, (189, (284, (277, (198, (244, \
                    nil)))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))), \
                    (-1, (0, (nil, nil))))), (0, (nil, nil))))").unwrap();
    
    let mut state = init_state.clone();
    // let mut state_to_draw = Data::Nil;
    let mut paired: Vec<i128> = Vec::new();

    let protocol = Protocol::load_galaxy();
    for i in 0..64 {
        if paired.contains(&i) { continue; }
        for j in i + 1..64 {
            if paired.contains(&j) { continue; }
            let (y1, x1) = to_coord(i);
            let (y2, x2) = to_coord(j);
            
            let result = protocol.interact(state, Data::make_cons(x1, y1), &Endpoint::Proxy);
            let result = protocol.interact(result.final_state, Data::make_cons(x2, y2), &Endpoint::Proxy);
            state = result.final_state;
            // state_to_draw = result.data_out_to_multipledraw;

            let pair = state.clone().into_vec()[1].clone().into_vec()[4].clone().into_vec();
            println!("{}, {}, {:?}", i, j, &pair);
            if pair.len() > 0 {
                assert!(pair.len() == 2);
                println!("Paired: {}: {}, {}: {}", i, pair[0].to_string(), j, pair[1].to_string());
                paired.push(i);
                paired.push(j);
                state = init_state.clone();
                break;
            }         
        }
    }
    // let matrices = eval_multidraw(state_to_draw);
    // matrices_to_png(&matrices, project_path("outputs/galaxyXX"));
}


fn main() {
    std::thread::Builder::new().stack_size(100 * 1024 * 1024)
        .spawn(click_all_pairs).unwrap()
        .join().unwrap();
}
