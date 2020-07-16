extern crate julie;
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

    let v = pngs::bordered_png_to_matrix(&format!("scratches/julie/data/{}", filename));
    let mut unidentified: Vec<ImgMatrix> = Vec::new();
    let operations = pngs::read_operations();
    println!("{}", parse_image(&v, &mut unidentified, &operations));

    println!("Unidentified symbols:");
    for (i, m) in unidentified.iter().enumerate() {
        println!("\n?{}", (97 + i as u8) as char);
        show_image(m);
    }
}

struct SymbolFrameInfo {
    // frame of current symbol, to be mutated while reading
    top: usize,
    left: usize,
    bottom: usize,
    right: usize,
}

enum Symbol {
    Integer(i64),
    // Float(f64),
    Operation(String),
    Unknown(usize),  // id in list of unknown symbols
    Omission,
    EOL,
}



// =====================================
// PARSING BEGINS
// =====================================

fn parse_image(img: &ImgMatrix,
            unidentified: &mut Vec<ImgMatrix>,
            operations: &HashMap<String, ImgMatrix>) -> String {
    let height = img[0].len();
    let mut frame = SymbolFrameInfo { top: 0, bottom: 1, left: 0, right: 0 };
    let mut result = String::new();

    loop {
        while frame.top < height && is_horizontal_divider(img, frame.top) {
            frame.top += 1;
        }
    
        frame.bottom = frame.top + 1;
        while frame.bottom < height && !is_horizontal_divider(img, frame.bottom) {
            frame.bottom += 1;
        }
        if frame.bottom >= height { break; }

        result.push_str(&format!("{}\n", &parse_strip(img, &mut frame, unidentified, operations)));
        frame.top = frame.bottom;
    }
    result
}

// divider is a horizontal line with no white pixels, divides strips of symbols
fn is_horizontal_divider(img: &ImgMatrix, y: usize) -> bool {
    for x in 0..img.len() {
        if img[x][y] {
            return false;
        }
    }
    true
}

// parses one strip of symbols
fn parse_strip(img: &ImgMatrix,
    frame: &mut SymbolFrameInfo,
    unidentified: &mut Vec<ImgMatrix>,
    operations: &HashMap<String, ImgMatrix>) -> String {

    let width = img.len();
    let mut result = String::new();
    frame.left = 0;

    loop {
        while frame.left < width && is_vertical_divider(img, frame, frame.left) {
            frame.left += 1;
        }
        if frame.left >= width { break; }

        // parse_symbol changes the right border of the frame while parsing
        match parse_symbol(img, frame, unidentified, operations) {
            Symbol::Integer(x) => result.push_str(&x.to_string()),
            // Symbol::Float(x) => result.push_str(&x.to_string()),
            Symbol::Operation(x) => result.push_str(&x),
            Symbol::Unknown(x) => result.push_str(&format!("?{}", (97 + x as u8) as char)),
            Symbol::Omission => result.push_str("...."),
            Symbol::EOL => break,
        }
        result.push_str(" ");
    
        frame.left = frame.right + 1;
    }
    result
}

fn is_vertical_divider(img: &ImgMatrix, frame: &SymbolFrameInfo, x: usize) -> bool {
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

fn parse_symbol(img: &ImgMatrix,
    frame: &mut SymbolFrameInfo,
    unidentified: &mut Vec<ImgMatrix>,
    operations: &HashMap<String, ImgMatrix>) -> Symbol {

    let width = img.len();

    frame.right = frame.left + 1;
    while frame.right < width && !is_vertical_divider(img, frame, frame.right) {
        frame.right += 1;
    }
    
    if is_integer(img, frame.left, frame.top) {
        return parse_integer(img, frame);
    }

    if is_omission(img, frame, frame.left) {
        frame.right = frame.left + 8;
        return Symbol::Omission;
    }

    if frame.right >= width {
        return Symbol::EOL;
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

fn parse_integer(img: &ImgMatrix, frame: &SymbolFrameInfo) -> Symbol {
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
    Symbol::Integer(n * sgn)
}


// checks if it's "...." sign
fn is_omission(img: &ImgMatrix, frame: &SymbolFrameInfo, x: usize) -> bool {
    let mut h = 0;
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
        if !is_vertical_divider(img, frame, x + i*2 + 1) { return false; }
    }
    true
}

fn get_operation(sample: &ImgMatrix, operations: &HashMap<String, ImgMatrix>) -> Option<Symbol> {
    for (k, v) in operations.iter() {
        if v == sample { return Some(Symbol::Operation(k.clone())) }
    }
    None
}

fn get_unknown(sample: &ImgMatrix, unidentified: &mut Vec<ImgMatrix>) -> Symbol {
    for (i, m) in unidentified.iter().enumerate() {
        if m == sample { return Symbol::Unknown(i); }
    }
    unidentified.push(sample.clone());
    Symbol::Unknown(unidentified.len() - 1)
}

// makes a small Image Matrix with just a given symbol in it
fn crop_image(img: &ImgMatrix, frame: &SymbolFrameInfo) -> ImgMatrix {
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

// надо брать из WSL-ного ~/.ssh/id_rsa.pub
