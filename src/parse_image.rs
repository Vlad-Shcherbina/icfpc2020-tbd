use crate::img_matrix::ImgMatrix;
use crate::img_matrix::Coord;

use std::{fmt::Display, collections::HashMap};

struct TokenFrameInfo {
    // frame of the current token, to be mutated while reading
    top: usize,
    left: usize,
    bottom: usize,
    right: usize,
}

#[derive(PartialEq, Eq)]
pub enum Token {
    Integer(i64),
    // Float(f64),
    Variable(i64),
    Operation(String),
    Unknown(usize),  // id in list of unknown tokens
    Omission,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Integer(x)   => write!(f, "{}", x),
            Token::Variable(x)  => write!(f, "{}", (97 + *x as u8) as char),
            Token::Operation(x) => write!(f, "{}", x),
            Token::Unknown(x)   => write!(f, "?{}", x),
            Token::Omission     => write!(f, "..."),
        }
    }
}


// =====================================
// PARSING BEGINS
// =====================================

pub fn parse_image(img: &ImgMatrix,
            unidentified: &mut Vec<ImgMatrix>,
            operations: &HashMap<String, ImgMatrix>) -> Vec<Vec<Token>> {

    let mut frame = TokenFrameInfo { top: 0, bottom: 1, left: 0, right: 0 };
    let mut result = Vec::new();

    loop {
        while frame.top < img.height && is_horizontal_separator(img, frame.top) {
            frame.top += 1;
        }
    
        frame.bottom = frame.top + 1;
        while frame.bottom < img.height && !is_horizontal_separator(img, frame.bottom) {
            frame.bottom += 1;
        }
        if frame.bottom >= img.height { break; }

        result.push(parse_strip(img, &mut frame, unidentified, operations));
        frame.top = frame.bottom;
    }
    result
}

// separator is a horizontal line with no white pixels, separates strips of tokens
fn is_horizontal_separator(img: &ImgMatrix, y: usize) -> bool {
    for x in 0..img.width {
        if img[Coord {x, y}] == 1 {
            return false;
        }
    }
    true
}

// parses one strip — a line of tokens
fn parse_strip(img: &ImgMatrix,
    frame: &mut TokenFrameInfo,
    unidentified: &mut Vec<ImgMatrix>,
    operations: &HashMap<String, ImgMatrix>) -> Vec<Token> {

    let mut result = Vec::new();
    frame.left = 0;

    loop {
        while frame.left < img.width - 1 && is_vertical_separator(img, frame, frame.left) {
            frame.left += 1;
        }
        frame.right = frame.left + 1;
        while frame.right < img.width && !is_vertical_separator(img, frame, frame.right) {
            frame.right += 1;
        }

        if frame.right >= img.width { break; }

        let token = parse_token(img, frame, unidentified, operations);
        if token == Token::Omission {
            frame.right = frame.left + 8;
        }
        result.push(token);

        frame.left = frame.right + 1;
    }
    result
}

// vertical separator is a column within the strip with no white pixels, separates tokens
fn is_vertical_separator(img: &ImgMatrix, frame: &TokenFrameInfo, x: usize) -> bool {
    for y in frame.top..frame.bottom {
        if img[Coord{x, y}] == 1 {
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

    assert!(frame.right < img.width);
    if let Some(a) = parse_integer(img, frame) {
        return a;
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

fn parse_integer(img: &ImgMatrix, frame: &TokenFrameInfo) -> Option<Token> {
    if img[Coord {x : frame.left, y : frame.top}] != 0 { return None; }

    let base = frame.right - frame.left - 1;
    if base < 1 ||  base > (frame.bottom - frame.top - 1) { return None; }
    for i in 1..base + 1 {
        if img[Coord {x : frame.left, y : frame.top + i}] == 0 { return None; }
        if img[Coord {x : frame.left + i, y : frame.top}] == 0 { return None; }
    }

    let sgn = if img[Coord { x: frame.left, y: frame.top + base + 1}] == 1 { -1 } else { 1 };

    let mut digit = 1;
    let mut n = 0;
    for i in 1..base+1 {
        for j in 1..base+1 {
            if img[Coord { x: frame.left + j, y: frame.top + i }] == 1 {
                n += digit;
            }
            digit *= 2;
        }
    }
    Some(Token::Integer(n * sgn))
}


fn is_variable(img: &ImgMatrix, frame: &TokenFrameInfo) -> bool {
    let size = frame.right - frame.left;
    if size < 4 { return false };
    if size < 1 ||  size > (frame.bottom - frame.top - 1) { return false; }
    for i in 0..size {
        if img[Coord { x : frame.left + i, y : frame.top }] == 0 
        || img[Coord { x : frame.left + i, y : frame.top + size - 1 }] == 0
        || img[Coord { x : frame.left, y : frame.top + i }] == 0
        || img[Coord { x : frame.left + size - 1, y : frame.top + i }] == 0 { 
               return false;
        }
    }

    img[Coord {x : frame.left + 1, y : frame.top + 1 }] == 1
}


fn parse_variable(img: &ImgMatrix, frame: &TokenFrameInfo) -> Token {
    let base = frame.right - frame.left - 3;
    // additional -2 due to the border

    let mut digit = 1;
    let mut n = 0;
    for i in 2..base + 2 {
        for j in 2..base + 2 {
            if img[Coord{ x : frame.left + j, y : frame.top + i}] == 0 { n += digit; }
            digit *= 2;
        }
    }
    Token::Variable(n)
}


// checks if it's "...." sign
fn is_omission(img: &ImgMatrix, frame: &TokenFrameInfo, x: usize) -> bool {
    if frame.left > img.width - 8 { return false; }

    let mut h = 0;  // vertical position of the ellipsis
    for y in frame.top..frame.bottom {
        if img[Coord{ x, y }] == 1 { 
            h = y;
            break;
        }
    }

    for i in 0..4 {
        for y in frame.top..frame.bottom {
            if (img[Coord { x : x + i * 2, y }] == 1) != (y == h) { return false; }
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
    let mut v: Vec<Vec<u8>> = Vec::new();
    for y in frame.top..frame.bottom {
        let mut u: Vec<u8> = Vec::new();
        let mut end = true;
        for x in frame.left..frame.right {
            u.push(img[Coord { x, y }]);
            if img[Coord { x, y }] == 1 { end = false; }
        }
        if end && v.len() > 0 { break; }
        v.push(u)
    }
    ImgMatrix::from_vec(&v)
}

impl Display for ImgMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self[Coord { x, y }] {
                    0 => write!(f, ". ")?,
                    1 => write!(f, "* ")?,
                    x => panic!("{:?}", x),
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

pub fn show_image(img: &ImgMatrix) {
    print!("{}", img);
}

