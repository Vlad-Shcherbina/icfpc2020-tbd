use crate::img_matrix::ImgMatrix;
// use crate::img_matrix::Coord;

use std::collections::HashMap;

pub const APP_OPERATION_NAME: &str = "`";

pub fn read_operations() -> HashMap<String, ImgMatrix> {
    let mut m: HashMap<String, ImgMatrix> = HashMap::new();
    m.insert(
        "=".to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 0],
                                  vec![1, 1, 1]])
    );
    m.insert(
        "neg".to_string(),
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
        APP_OPERATION_NAME.to_string(),
        ImgMatrix::from_vec(&vec![vec![1, 1],
                                  vec![1, 0]])
    );

    
      
    m.insert(
        "demodulate".to_string(),   // message38
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 1, 0, 1],
                                  vec![1, 0, 1, 0],
                                  vec![1, 1, 0, 1]])
    );

      
    m.insert(
        "modulate".to_string(),   // message38
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 0, 1, 0],
                                  vec![1, 1, 0, 1],
                                  vec![1, 0, 1, 0]])
    );

    //  CONTEST STARS

    m.insert(
        "S".to_string(),   // message18
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 1, 1],
                                  vec![1, 1, 0]])
    );

    m.insert(
        "flip".to_string(),   // message19
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 1],
                                  vec![1, 1, 0]])
    );

    m.insert(
        "(.)".to_string(),   // message20
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 1, 0],
                                  vec![1, 1, 0]])
    );

    m.insert(
        "pwr2".to_string(),   // message23
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0, 0, 1],
                                  vec![1, 0, 0, 1, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 0, 0, 1],
                                  vec![1, 0, 0, 0, 0, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1, 1]])
    );

    m.insert(
        "I".to_string(),   // message24
        ImgMatrix::from_vec(&vec![vec![1, 1],
                                  vec![1, 1]])
    );

    m.insert(
        "cons".to_string(),   // message25
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 1, 1, 1, 1]])
    );

    m.insert(
        "car".to_string(),   // message26
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1],
                                  vec![1, 0, 1, 1, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 1, 1, 1, 1]])
    );

    m.insert(
        "cdr".to_string(),   // message27
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1],
                                  vec![1, 1, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1],
                                  vec![1, 1, 1, 1, 1]])
    );

    m.insert(
        "nil".to_string(),   // message28
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 0, 1],
                                  vec![1, 1, 1]])
    );

    m.insert(
        "is_nil".to_string(),   // message29
        ImgMatrix::from_vec(&vec![vec![1, 1, 1],
                                  vec![1, 1, 1],
                                  vec![1, 1, 1]])
    );
  
    m.insert(
        "[".to_string(),   // message30
        ImgMatrix::from_vec(&vec![vec![0, 0, 1],
                                  vec![0, 1, 1],
                                  vec![1, 1, 1],
                                  vec![0, 1, 1],
                                  vec![0, 0, 1]])
    );

    m.insert(
        ",".to_string(),   // message30
        ImgMatrix::from_vec(&vec![vec![1, 1],
                                  vec![1, 1],
                                  vec![1, 1],
                                  vec![1, 1],
                                  vec![1, 1]])
    );

    m.insert(
        "]".to_string(),   // message30
        ImgMatrix::from_vec(&vec![vec![1, 0, 0],
                                  vec![1, 1, 0],
                                  vec![1, 1, 1],
                                  vec![1, 1, 0],
                                  vec![1, 0, 0]])
    );
  
    m.insert(
        "vec".to_string(),   // message31
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 1, 0, 0, 0, 0],
                                  vec![1, 0, 1, 0, 0, 0],
                                  vec![1, 0, 0, 1, 0, 0],
                                  vec![1, 0, 0, 0, 1, 0],
                                  vec![1, 0, 0, 0, 0, 1]])
    );

    m.insert(
        "draw".to_string(),   // message32
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0, 1],
                                  vec![1, 0, 0, 0, 0, 1],
                                  vec![1, 0, 0, 0, 0, 1],
                                  vec![1, 0, 0, 0, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1]])
    );

    m.insert(
        "checkerboard".to_string(),   // message33
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 1, 0, 1, 0],
                                  vec![1, 1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1, 0],
                                  vec![1, 1, 0, 1, 0, 1],
                                  vec![1, 0, 1, 0, 1, 0]])
    );


    m.insert(
        "multiple_draw".to_string(),   // message34
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 1, 0, 0, 1],
                                  vec![1, 0, 0, 1, 0, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 1, 0, 0, 1],
                                  vec![1, 0, 0, 1, 0, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1, 1]])
    );

    m.insert(
        "pic35_1".to_string(),   // message35
        ImgMatrix::from_vec(&vec![vec![0, 0],
                                  vec![1, 1]])
    );

    m.insert(
        "send".to_string(),   // message36
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 1, 1, 1, 1, 0],
                                  vec![1, 1, 1, 1, 0, 0],
                                  vec![1, 1, 1, 0, 0, 1],
                                  vec![1, 1, 0, 0, 0, 0]])
    );
  
    m.insert(
        "pic36_2".to_string(),   // message36
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1],
                                  vec![1, 0, 1, 1],
                                  vec![1, 1, 0, 1],
                                  vec![1, 0, 1, 0]])
    );

    m.insert(
        "if0".to_string(),   // message37
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0],
                                  vec![1, 0, 1, 1, 1],
                                  vec![1, 1, 1, 0, 0],
                                  vec![1, 0, 1, 1, 1]])
    );

 
    m.insert(
        "interact".to_string(),   // message38
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0, 1],
                                  vec![1, 0, 1, 1, 0, 1],
                                  vec![1, 0, 1, 1, 0, 1],
                                  vec![1, 0, 0, 0, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1]])
    );
  
    m.insert(
        "modem".to_string(),   // message38
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0],
                                  vec![1, 1, 0, 0, 1, 1],
                                  vec![1, 0, 1, 1, 0, 0]])
    );

    m.insert(
        "pic38_5".to_string(),   // message38
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 1, 0, 0, 1],
                                  vec![1, 0, 1, 1, 1, 1],
                                  vec![1, 1, 1, 1, 0, 1],
                                  vec![1, 0, 0, 1, 0, 1],
                                  vec![1, 1, 1, 1, 1, 1]])
    );

    m.insert(
        "pic40".to_string(),   // message40
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1, 1],
                                  vec![1, 0, 0, 0, 0, 1, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0],
                                  vec![1, 0, 1, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0]])
    );
  
    m.insert(
        "pic41".to_string(),   // message41
        ImgMatrix::from_vec(&vec![vec![1, 1, 1, 1, 1, 1, 1],
                                  vec![1, 1, 0, 0, 0, 0, 0],
                                  vec![1, 1, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0],
                                  vec![1, 0, 0, 1, 0, 0, 0],
                                  vec![1, 0, 0, 0, 0, 0, 0]])
    );
    
    m.insert(
        "pic42".to_string(),   // message42
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