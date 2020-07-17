use julie::png_interface as pngs;

use std::collections::HashMap;

// optional: add number of the message to parse as a command line argument
type ImgMatrix = Vec<Vec<bool>>;

fn main() {
    let mut filename = "message2.png".to_string();
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        filename = format!("message{}.png", args[1]);
    }

    let v = pngs::bordered_png_to_matrix(&format!("scratches\\julie\\data\\{}", filename));
    let mut unidentified: Vec<ImgMatrix> = Vec::new();
    let operations = pngs::read_operations();

    println!("{}", parse_image(&v, &mut unidentified, &operations));
    println!("Unidentified tokens:");
    for (i, m) in unidentified.iter().enumerate() {
        println!("\n?{}", i);
        show_image(m);
    }
}

struct TokenFrameInfo {
    // frame of the current token, to be mutated while reading
    top: usize,
    left: usize,
    bottom: usize,
    right: usize,
}

enum Token {
    Integer(i64),
    // Float(f64),
    Variable(i64),
    Operation(String),
    Unknown(usize),  // id in list of unknown tokens
    Omission,
}


// =====================================
// PARSING BEGINS
// =====================================

fn parse_image(img: &ImgMatrix,
            unidentified: &mut Vec<ImgMatrix>,
            operations: &HashMap<String, ImgMatrix>) -> String {

    let height = img[0].len();
    let mut frame = TokenFrameInfo { top: 0, bottom: 1, left: 0, right: 0 };
    let mut result = String::new();

    loop {
        while frame.top < height && is_horizontal_separator(img, frame.top) {
            frame.top += 1;
        }
    
        frame.bottom = frame.top + 1;
        while frame.bottom < height && !is_horizontal_separator(img, frame.bottom) {
            frame.bottom += 1;
        }
        if frame.bottom >= height { break; }

        result.push_str(&parse_strip(img, &mut frame, unidentified, operations));
        result.push_str("\n");
        frame.top = frame.bottom;
    }
    result
}

// separator is a horizontal line with no white pixels, separates strips of tokens
fn is_horizontal_separator(img: &ImgMatrix, y: usize) -> bool {
    for x in 0..img.len() {
        if img[x][y] {
            return false;
        }
    }
    true
}

// parses one strip — a line of tokens
fn parse_strip(img: &ImgMatrix,
    frame: &mut TokenFrameInfo,
    unidentified: &mut Vec<ImgMatrix>,
    operations: &HashMap<String, ImgMatrix>) -> String {

    let width = img.len();
    let mut result = String::new();
    frame.left = 0;

    loop {
        while frame.left < width - 1 && is_vertical_separator(img, frame, frame.left) {
            frame.left += 1;
        }
        frame.right = frame.left + 1;
        while frame.right < width && !is_vertical_separator(img, frame, frame.right) {
            frame.right += 1;
        }

        if frame.right >= width { break; }

        match parse_token(img, frame, unidentified, operations) {
            Token::Integer(x)   => result.push_str(&x.to_string()),
            // Token::Float(x) => result.push_str(&x.to_string()),
            Token::Variable(x)  => result.push_str(&((97 + x as u8) as char).to_string()),
            Token::Operation(x) => result.push_str(&x),
            Token::Unknown(x)   => result.push_str(&format!("?{}", x)),
            Token::Omission     => {
                frame.right = frame.left + 8;
                result.push_str("....");
            },
        }
        result.push_str(" ");
        frame.left = frame.right + 1;
    }
    result
}

// vertical separator is a column within the strip with no white pixels, separates tokens
fn is_vertical_separator(img: &ImgMatrix, frame: &TokenFrameInfo, x: usize) -> bool {
    for y in frame.top..frame.bottom {
        if img[x][y] {
            return false;
        }
    }
    true
}

// ===================================
// PARSING INDIVIDUAL TOKENS
// ===================================

fn parse_token(img: &ImgMatrix,
    frame: &TokenFrameInfo,
    unidentified: &mut Vec<ImgMatrix>,
    operations: &HashMap<String, ImgMatrix>) -> Token {

    let width = img.len();
    assert!(frame.right < width);

    if is_integer(img, frame.left, frame.top) {
        return parse_integer(img, frame);
    }

    if is_variable(img, frame) {
        return parse_variable(img, frame);
    }

    if is_omission(img, frame, frame.left) {
        return Token::Omission;
    }

    let sample = crop_image(img, frame);
    if let Some(a) = get_operation(&sample, operations) {
        return a;
    }

    return get_unknown(&sample, unidentified);
}


// assuming every integer number has corner of 0, 1, 1
fn is_integer(img: &ImgMatrix, x: usize, y: usize) -> bool {
    !img[x][y] && img[x + 1][y] && img[x][y + 1]
}


fn parse_integer(img: &ImgMatrix, frame: &TokenFrameInfo) -> Token {
    let mut base = 0;
    for i in 1.. {
        if !img[frame.left + i][frame.top] { break; }
        base = i;
    }
    assert!(frame.right == frame.left + base + 1);

    let sgn = if img[frame.left][frame.top + base + 1] { -1 } else { 1 };

    let mut digit = 1;
    let mut n = 0;
    for i in 1..base+1 {
        for j in 1..base+1 {
            if img[frame.left + j][frame.top + i] { n += digit; }
            digit *= 2;
        }
    }
    Token::Integer(n * sgn)
}


fn is_variable(img: &ImgMatrix, frame: &TokenFrameInfo) -> bool {
    let size = frame.right - frame.left;
    if size < 4 { return false };
    for i in 0..size {
        if !img[frame.left + i][frame.top] { return false; }
        if !img[frame.left + i][frame.top + size - 1] { return false; }
        if !img[frame.left][frame.top + i] { return false; }
        if !img[frame.left + size - 1][frame.top + i] { return false; }
    }

    true
}


fn parse_variable(img: &ImgMatrix, frame: &TokenFrameInfo) -> Token {
    let base = frame.right - frame.left - 3;
    // additional -2 due to the border

    let mut digit = 1;
    let mut n = 0;
    for i in 2..base + 2 {
        for j in 2..base + 2 {
            if !img[frame.left + j][frame.top + i] { n += digit; }
            digit *= 2;
        }
    }
    Token::Variable(n)
}


// checks if it's "...." sign
fn is_omission(img: &ImgMatrix, frame: &TokenFrameInfo, x: usize) -> bool {
    if frame.left > img.len() - 8 { return false; }

    let mut h = 0;  // vertical position of the ellipsis
    for y in frame.top..frame.bottom {
        if img[x][y] { 
            h = y;
            break;
        }
    }

    for i in 0..4 {
        for y in frame.top..frame.bottom {
            if img[x + i * 2][y] != (y == h) { return false; }
        }
        if !is_vertical_separator(img, frame, x + i*2 + 1) { return false; }
    }
    true
}

fn get_operation(sample: &ImgMatrix, operations: &HashMap<String, ImgMatrix>) -> Option<Token> {
    for (k, v) in operations.iter() {
        if v == sample { return Some(Token::Operation(k.clone())) }
    }
    None
}

fn get_unknown(sample: &ImgMatrix, unidentified: &mut Vec<ImgMatrix>) -> Token {
    for (i, m) in unidentified.iter().enumerate() {
        if m == sample { return Token::Unknown(i); }
    }
    unidentified.push(sample.clone());
    Token::Unknown(unidentified.len() - 1)
}

// makes a small Image Matrix with just a given token in it
fn crop_image(img: &ImgMatrix, frame: &TokenFrameInfo) -> ImgMatrix {
    let mut v: ImgMatrix = vec![Vec::new(); frame.right - frame.left];
    for y in frame.top..frame.bottom {
        let mut end = true;
        for x in frame.left..frame.right {
            if img[x][y] { end = false; }
        }
        if end { break; }
        for x in frame.left..frame.right {
            v[x - frame.left].push(img[x][y]);
        }
    }

    v
}

fn show_image(img: &ImgMatrix) {
    for y in 0..img[0].len() {
        for x in 0..img.len() {
            if img[x][y] { print!("# "); }
            else { print!(". "); }
        }
        println!();
    }
}

