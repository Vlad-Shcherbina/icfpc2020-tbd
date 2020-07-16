extern crate png;
use std::fs::File;

const COLOR_SIZE: usize = 4;  // 4 components of each color in png

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
    // Operation(String),
    Unknown(usize),  // id in list of unknown symbols
    EOL,
}

fn main() {
    let v = file_to_bool_matrix("scratches/julie/data/message3.png");
    println!("{}", parse_image(&v));
}

// takes path to an image and turns it into WIDTH x HEIGHT bool matrix
fn file_to_bool_matrix(path: &str) -> Vec<Vec<bool>> {
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    // assuming there is a border, find out the size of the "pixel" (4x4 on messages 2-13)
    let mut pixel_size = 1;
    while buf[(pixel_size * info.width as usize + pixel_size) * COLOR_SIZE] == 255 { 
        pixel_size += 1
    }

    // -2 - excluding the border
    let width = info.width as usize / pixel_size - 2;
    let height = info.height as usize / pixel_size - 2;
    let mut v: Vec<Vec<bool>> = vec![vec![false; height]; width];  
    for x in 0..width {
        for y in 0..height {
            // + 1 due to border
            let coord = ((y + 1) * (info.width as usize) + (x + 1)) * pixel_size * COLOR_SIZE;
            v[x][y] = if buf[coord] == 0 { false } else { true };
        }
    }
    v
}

fn parse_image(img: &Vec<Vec<bool>>) -> String {
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

        result.push_str(&format!("{}\n", &parse_strip(img, &mut frame)));
        frame.top = frame.bottom;
    }
    result
}

// divider is a horizontal line with no white pixels, divides strips of symbols
fn is_horizontal_divider(img: &Vec<Vec<bool>>, y: usize) -> bool {
    for x in 0..img.len() {
        if img[x][y] {
            return false;
        }
    }
    true
}

// parses one strip of symbols
fn parse_strip(img: &Vec<Vec<bool>>, frame: &mut SymbolFrameInfo) -> String {
    let width = img.len();
    let mut result = String::new();

    loop {
        while frame.left < width && is_vertical_divider(img, frame, frame.left) {
            frame.left += 1;
        }
        if frame.left >= width { break; }

        // parse_symbol changes the right border of the frame while parsing
        match parse_symbol(img, frame) {
            Symbol::Integer(x) => result.push_str(&x.to_string()),
            // Symbol::Float(x) => result.push_str(&x.to_string()),
            // Symbol::Operation(x) => result.push_str(&x),
            Symbol::Unknown(x) => result.push_str(&format!("#{}", x)),
            Symbol::EOL => break,
        }
        result.push_str(" ");
    
        frame.left = frame.right + 1;
    }
    result
}

fn is_vertical_divider(img: &Vec<Vec<bool>>, frame: &SymbolFrameInfo, x: usize) -> bool {
    for y in frame.top..frame.bottom {
        if img[x][y] {
            return false;
        }
    }
    true
}

fn parse_symbol(img: &Vec<Vec<bool>>, frame: &mut SymbolFrameInfo) -> Symbol {
    let width = img.len();
    if frame.right >= width - 1 { return Symbol::EOL };
    
    if is_integer(img, frame.left, frame.top) {
        return parse_integer(img, frame);
    }

    frame.right = frame.left + 1;
    while frame.right < width && !is_vertical_divider(img, frame, frame.right) {
        frame.right += 1;
    }

    if frame.right >= width {
        return Symbol::EOL;
    }

    return Symbol::Unknown(0);  // a stab
}


// assuming every integer number has corner of 0, 1, 1
fn is_integer(img : &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    !img[x][y] && img[x + 1][y] && img[x][y + 1]
}

fn parse_integer(img : &Vec<Vec<bool>>, frame: &mut SymbolFrameInfo) -> Symbol {
    let mut base = 0;
    for i in 1.. {
        if !img[frame.left + i][frame.top] { break; }
        base = i;
    }

    frame.right = frame.left + base + 1;
    assert!(is_vertical_divider(img, frame, frame.right));

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
