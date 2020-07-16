extern crate png;
use std::fs::File;

// optional: add number of the message to parse as a command line argument
type ImgMatrix = Vec<Vec<bool>>;

fn main() {
    let mut filename = "message2.png".to_string();
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        filename = format!("message{}.png", args[1]);
    }

    let v = file_to_bool_matrix(&format!("scratches/julie/data/{}", filename));
    let mut unidentified: Vec<ImgMatrix> = Vec::new();
    println!("{}", parse_image(&v, &mut unidentified));
}

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
    Omission,
    EOL,
}

// takes path to an image and turns it into WIDTH x HEIGHT bool matrix
fn file_to_bool_matrix(path: &str) -> ImgMatrix {
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
    let mut v: ImgMatrix = vec![vec![false; height]; width];  
    for x in 0..width {
        for y in 0..height {
            // + 1 due to border
            let coord = ((y + 1) * (info.width as usize) + (x + 1)) * pixel_size * COLOR_SIZE;
            v[x][y] = if buf[coord] == 0 { false } else { true };
        }
    }
    v
}

fn parse_image(img: &ImgMatrix, unidentified: &mut Vec<ImgMatrix>) -> String {
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

        result.push_str(&format!("{}\n", &parse_strip(img, &mut frame, unidentified)));
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
               unidentified: &mut Vec<ImgMatrix>) -> String {
    let width = img.len();
    let mut result = String::new();
    frame.left = 0;

    loop {
        while frame.left < width && is_vertical_divider(img, frame, frame.left) {
            frame.left += 1;
        }
        if frame.left >= width { break; }

        // parse_symbol changes the right border of the frame while parsing
        match parse_symbol(img, frame, unidentified) {
            Symbol::Integer(x) => result.push_str(&x.to_string()),
            // Symbol::Float(x) => result.push_str(&x.to_string()),
            // Symbol::Operation(x) => result.push_str(&x),
            Symbol::Unknown(x) => result.push_str(&format!("#{}", x)),
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

fn parse_symbol(img: &ImgMatrix,
                frame: &mut SymbolFrameInfo,
                unidentified: &mut Vec<ImgMatrix>) -> Symbol {
    let width = img.len();
    // if frame.right >= width - 1 { return Symbol::EOL };
    
    if is_integer(img, frame.left, frame.top) {
        return parse_integer(img, frame);
    }

    if is_omission(img, frame, frame.left) {
        frame.right = frame.left + 8;
        return Symbol::Omission;
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
fn is_integer(img: &ImgMatrix, x: usize, y: usize) -> bool {
    !img[x][y] && img[x + 1][y] && img[x][y + 1]
}

fn parse_integer(img: &ImgMatrix, frame: &mut SymbolFrameInfo) -> Symbol {
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
