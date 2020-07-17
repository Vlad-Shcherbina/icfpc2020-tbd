use crate::img_matrix::ImgMatrix;
// use crate::img_matrix::Coord;

use std::collections::HashMap;

pub fn read_operations() -> HashMap<String, ImgMatrix> {
    let mut m: HashMap<String, ImgMatrix> = HashMap::new();
    m.insert(
        "=".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 0],
                                  vec![1, 1, 1]])
    );
    m.insert(
        "-".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 1],
                                  vec![1, 0, 1]])
    );
    m.insert(
        "TRUE".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 1],
                                  vec![1, 0, 0]])
    );
    m.insert(
        "FALSE".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 0],
                                  vec![1, 0, 1]])
    );
    m.insert(
        "incr".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 1, 0, 0],
                                  vec![1, 0, 0, 1],
                                  vec![1, 0, 1, 1]])
    );
    m.insert(
        "decr".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 1, 0, 0],
                                  vec![1, 0, 1, 0],
                                  vec![1, 0, 1, 1]])
    );
    m.insert(
        "+".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 1, 0, 1],
                                  vec![1, 1, 0, 1],
                                  vec![1, 1, 0, 1]])
    );
    m.insert(
        "*".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 0, 1, 0],
                                  vec![1, 0, 1, 0],
                                  vec![1, 0, 1, 0]])
    );
    m.insert(
        "/".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 0, 0, 0],
                                  vec![1, 1, 0, 1],
                                  vec![1, 0, 0, 0]])
    );
    m.insert(
        "==".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 0, 0, 0],
                                  vec![1, 0, 0, 0],
                                  vec![1, 1, 1, 1]])
    );
    m.insert(
        "<".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 0, 0, 0],
                                  vec![1, 0, 0, 1],
                                  vec![1, 0, 1, 1]])
    );
    m.insert(
        "_".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1],
                                  vec![1, 0]])
    );

    //  CONTEST STARS

    m.insert(
        "pic18".to_string(),   // message18
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 1, 1],
                                  vec![1, 1, 0]])
    );

    m.insert(
        "pic19".to_string(),   // message19
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 1],
                                  vec![1, 1, 0]])
    );

    m.insert(
        "pic20".to_string(),   // message20
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 1, 0],
                                  vec![1, 1, 0]])
    );

    m.insert(
        "pic21".to_string(),   // message21
        ImgMatrix::from_vec(&vec![vec![1, 1],
                                  vec![1, 1]])
    );
 
    m.insert(
        "pic23".to_string(),   // message23
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0, 0, 1],
                                  vec![1, 0, 0, 1, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 0, 0, 1],
                                  vec![1, 0, 0, 0, 0, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1, 1]])
    );
 
    m.insert(
        "pic25".to_string(),   // message25
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 1, 1, 1, 1]])
    );
 
    m.insert(
        "pic26".to_string(),   // message26
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1],
                                  vec![1, 0, 1, 1, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 1, 1, 1, 1]])
    );
 
    m.insert(
        "pic28".to_string(),   // message28
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 1],
                                  vec![1, 1, 1]])
    );

    m.insert(
        "pic29".to_string(),   // message29
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 1, 1],
                                  vec![1, 1, 1]])
    );
  
    m.insert(
        "pic30_1".to_string(),   // message30
        ImgMatrix::from_vec(&vec![vec![0, 0, 1],
                                  vec![0, 1, 1],
                                  vec![1, 1, 1],
                                  vec![0, 1, 1],
                                  vec![0, 0, 1]])
    );

    m.insert(
        "pic30_2".to_string(),   // message30
        ImgMatrix::from_vec(&vec![vec![1, 1],
                                  vec![1, 1],
                                  vec![1, 1],
                                  vec![1, 1],
                                  vec![1, 1]])
    );

    m.insert(
        "pic30_3".to_string(),   // message30
        ImgMatrix::from_vec(&vec![vec![1, 0, 0],
                                  vec![1, 1, 0],
                                  vec![1, 1, 1],
                                  vec![1, 1, 0],
                                  vec![1, 0, 0]])
    );
  
    m.insert(
        "pic31".to_string(),   // message31
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 1, 0, 0, 0, 0],
                                  vec![1, 0, 1, 0, 0, 0],
                                  vec![1, 0, 0, 1, 0, 0],
                                  vec![1, 0, 0, 0, 1, 0],
                                  vec![1, 0, 0, 0, 0, 1]])
    );
  
    m
}