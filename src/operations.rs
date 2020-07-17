use crate::img_matrix::ImgMatrix;
// use crate::img_matrix::Coord;

use std::collections::HashMap;

pub const APP_OPERATION_NAME: &str = "ap";

pub fn read_operations() -> HashMap<String, ImgMatrix> {
    let mut m: HashMap<String, ImgMatrix> = HashMap::new();

    m.insert(
        "=".to_string(), // message 4
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 0],
                                  vec![1, 1, 1]])
    );
    m.insert(
        "inc".to_string(), // message 5
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 1, 0, 0],
                                  vec![1, 0, 0, 1],
                                  vec![1, 0, 1, 1]])
    );
    m.insert(
        "dec".to_string(), // message 6
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 1, 0, 0],
                                  vec![1, 0, 1, 0],
                                  vec![1, 0, 1, 1]])
    );
    m.insert(
        "add".to_string(), // message 7
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 1, 0, 1],
                                  vec![1, 1, 0, 1],
                                  vec![1, 1, 0, 1]])
    );
    // message 8 is "variables"
    m.insert(
        "mul".to_string(), // message 9
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 0, 1, 0],
                                  vec![1, 0, 1, 0],
                                  vec![1, 0, 1, 0]])
    );
    m.insert(
        "div".to_string(), // message 10
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 0, 0, 0],
                                  vec![1, 1, 0, 1],
                                  vec![1, 0, 0, 0]])
    );
    m.insert(
        "eq".to_string(), // message 11
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 0, 0, 0],
                                  vec![1, 0, 0, 0],
                                  vec![1, 1, 1, 1]])
    );
    m.insert(
        "lt".to_string(),  // message 12
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 0, 0, 0],
                                  vec![1, 0, 0, 1],
                                  vec![1, 0, 1, 1]])
    );
    m.insert(
        "mod".to_string(), // message 13
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 0, 1, 0],
                                  vec![1, 1, 0, 1],
                                  vec![1, 0, 1, 0]])
    );
    m.insert(
        "dem".to_string(), // message 14
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 1, 0, 1],
                                  vec![1, 0, 1, 0],
                                  vec![1, 1, 0, 1]])
    );
    m.insert(
        "send".to_string(), // message 15
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 0, 1, 1],
                                  vec![1, 1, 0, 1],
                                  vec![1, 0, 1, 0]])
    );
    m.insert(
        "neg".to_string(), // message 16
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 1],
                                  vec![1, 0, 1]])
    );
    m.insert(
        APP_OPERATION_NAME.to_string(), // message 17
        ImgMatrix::from_vec(&vec![vec![1, 1],
                                  vec![1, 0]])
    );
    m.insert(
        "s".to_string(), // message 18
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 1, 1],
                                  vec![1, 1, 0]])
    );
    m.insert(
        "c".to_string(), // message 19
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 1],
                                  vec![1, 1, 0]])
    );
    m.insert(
        "b".to_string(), // message 20
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 1, 0],
                                  vec![1, 1, 0]])
    );
    m.insert(
        "t".to_string(), // message 21
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 1],
                                  vec![1, 0, 0]])
    );
    m.insert(
        "f".to_string(), // message 22
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 0],
                                  vec![1, 0, 1]])
    );
    m.insert(
        "pwr2".to_string(), // message 23
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0, 0, 1],
                                  vec![1, 0, 0, 1, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 0, 0, 1],
                                  vec![1, 0, 0, 0, 0, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1, 1]])
    );
    m.insert(
        "i".to_string(), // message 24
        ImgMatrix::from_vec(&vec![vec![1, 1],
                                  vec![1, 1]])
    );

    m.insert(
        "cons".to_string(), // message 25
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 1, 1, 1, 1]])
    );
    m.insert(
        "car".to_string(), // message 26
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1],
                                  vec![1, 0, 1, 1, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 1, 1, 1, 1]])
    );
    m.insert(
        "cdr".to_string(), // message 27
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1],
                                  vec![1, 1, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 1, 1, 1, 1]])
    );
    m.insert(
        "nil".to_string(), // message 28
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 1],
                                  vec![1, 1, 1]])
    );
    m.insert(
        "isnil".to_string(), // message 29
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 1, 1],
                                  vec![1, 1, 1]])
    );
    m.insert(
        "(".to_string(), // message 30
        ImgMatrix::from_vec(&vec![vec![0, 0, 1],
                                  vec![0, 1, 1],
                                  vec![1, 1, 1],
                                  vec![0, 1, 1],
                                  vec![0, 0, 1]])
    );
    m.insert(
        ",".to_string(), // message 30
        ImgMatrix::from_vec(&vec![vec![1, 1],
                                  vec![1, 1],
                                  vec![1, 1],
                                  vec![1, 1],
                                  vec![1, 1]])
    );
    m.insert(
        ")".to_string(), // message 30
        ImgMatrix::from_vec(&vec![vec![1, 0, 0],
                                  vec![1, 1, 0],
                                  vec![1, 1, 1],
                                  vec![1, 1, 0],
                                  vec![1, 0, 0]])
    );
    m.insert(
        "vec".to_string(), // message 31
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 1, 0, 0, 0, 0],
                                  vec![1, 0, 1, 0, 0, 0],
                                  vec![1, 0, 0, 1, 0, 0],
                                  vec![1, 0, 0, 0, 1, 0],
                                  vec![1, 0, 0, 0, 0, 1]])
    );
    m.insert(
        "draw".to_string(), // message 32
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0, 1],
                                  vec![1, 0, 0, 0, 0, 1],
                                  vec![1, 0, 0, 0, 0, 1],
                                  vec![1, 0, 0, 0, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1]])
    );
    m.insert(
        "checkerboard".to_string(), // message 33
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 1, 0, 1, 0],
                                  vec![1, 1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1, 0],
                                  vec![1, 1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1, 0]])
    );
    m.insert(
        "multipledraw".to_string(), // message 34
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 1, 0, 0, 1],
                                  vec![1, 0, 0, 1, 0, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 1, 0, 0, 1],
                                  vec![1, 0, 0, 1, 0, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1, 1]])
    );
    // message 35 is "mod cons"
    m.insert(
        "planetfall".to_string(), // message 36, not official name
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 1, 1, 1, 1, 0],
                                  vec![1, 1, 1, 1, 0, 0],
                                  vec![1, 1, 1, 0, 0, 1],
                                  vec![1, 1, 0, 0, 0, 0]])
    );
    m.insert(
        "if0".to_string(), // message 37
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0],
                                  vec![1, 0, 1, 1, 1],
                                  vec![1, 1, 1, 0, 0],
                                  vec![1, 0, 1, 1, 1]])
    );
    m.insert(
        "interact".to_string(), // message 38
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0, 1],
                                  vec![1, 0, 1, 1, 0, 1],
                                  vec![1, 0, 1, 1, 0, 1],
                                  vec![1, 0, 0, 0, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1]])
    );

    m.insert(
        "modem".to_string(), // message 38
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0],
                                  vec![1, 1, 0, 0, 1, 1],
                                  vec![1, 0, 1, 1, 0, 0]])
    );
    m.insert(
        "f38".to_string(), // message 38
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 1, 0, 0, 1],
                                  vec![1, 0, 1, 1, 1, 1],
                                  vec![1, 1, 1, 1, 0, 1],
                                  vec![1, 0, 0, 1, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1]])
    );
    // message 39 is "interaction protocol"
    m.insert(
        "statelessdraw".to_string(),   // message 40
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0, 1, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0],
                                  vec![1, 0, 1, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0]])
    );
    m.insert(
        "statefuldraw".to_string(), // message 41, unofficial name
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1, 1],
                                  vec![1, 1, 0, 0, 0, 0, 0],
                                  vec![1, 1, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 1, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0]])
    );
    m.insert(
        "galaxy".to_string(),   // message 42
        ImgMatrix::from_vec(&vec![vec![0, 0, 1, 1, 1, 0, 0],
                                  vec![0, 0, 0, 0, 0, 1, 0],
                                  vec![0, 1, 1, 1, 0, 0, 1],
                                  vec![1, 0, 1, 0, 1, 0, 1],
                                  vec![1, 0, 0, 1, 1, 1, 0],
                                  vec![0, 1, 0, 0, 0, 0, 0],
                                  vec![0, 0, 1, 1, 1, 0, 0]])
    );

    m
}
