use tbd::parse_image::Token;
use std::fs::File;
use std::path::{Path};
use tbd::img_matrix::{ImgMatrix, Coord};
use tbd::png_files::{matrix_to_png};
use std::io::{BufReader, BufRead};
use tbd::parse_image::Token::{Variable, Integer, Operation};
use tbd::operations::read_operations;
use std::cmp::max;

fn parse_tokens(s: String, unknown: &mut Vec<String>) -> Vec<Token> {
    let operations = read_operations();
    s.split_whitespace().map(|piece| {
        if piece.starts_with(":") {
            let idx: i64 = piece[1..].parse().expect("Variable expected");
            Variable(idx)
        } else if let Ok(num) = piece.parse::<i64>()  {
            Integer(num)
        } else if operations.contains_key(piece) {
            Operation(piece.to_string())
        } else {
            let unknown_id = unknown.len();
            unknown.push(piece.to_string());
            Token::Unknown(unknown_id)
        }
    }).collect()
}

fn render_integer(x: i64) -> ImgMatrix {
    let is_negative = x < 0;
    let mut x = x.abs();
    let mut bits = Vec::<u8>::with_capacity(64);
    loop {
        bits.push((x % 2) as u8);
        x /= 2;
        if x == 0 {
            break;
        }
    }
    let mut width = 1;
    while width * width < bits.len() {
        width += 1;
    }
    let mut img = ImgMatrix::new(width + 1, width + if is_negative { 2 } else { 1 });
    for i in 0..width {
        img[Coord { x: 0, y: i + 1 }] = 1;
        img[Coord { x: i + 1, y: 0 }] = 1;
    }
    for (i, b) in bits.iter().enumerate() {
        let x = i % width + 1;
        let y = i / width + 1;
        img[Coord { x, y }] = *b;
    }
    if is_negative {
        img[Coord { x: 0, y: width + 1}] = 1;
    }
    img
}

fn render_variable(idx: i64) -> ImgMatrix {
    let mut img = render_integer(idx);
    img[Coord { x: 0, y: 0 }] = 1;
    img
}

fn render_token(t: Token) -> ImgMatrix {
    let operations = read_operations();
    match t {
        Variable(idx) => {
            render_variable(idx)
        }
        Integer(x) => {
            render_integer(x)
        }
        Operation(op) => {
            operations.get(op.as_str()).unwrap().clone()
        }
        _ => {
            println!("Unsupported token");
            ImgMatrix::new(1, 1)
        }
    }
}

fn main() {
    let galaxy_path = tbd::project_path(Path::new("data").join("messages").join("galaxy.txt"));
    let galaxy_file = File::open(galaxy_path).expect("Galaxy.txt not found");
    let mut img = ImgMatrix::new(50, 50);
    let origin_x = 2;
    let origin_y = 2;
    let mut unknown = Vec::new();
    let mut current_x = origin_x;
    let mut current_y = origin_y;
    for l in BufReader::new(galaxy_file).lines() {
        let line = l.unwrap();
        let mut line_height = 0;
        for t in parse_tokens(line, &mut unknown) {
            let token_img = render_token(t);
            let new_width = if current_x + token_img.width > img.width {
                img.width * 2
            } else {
                img.width
            };
            let new_height = if current_y + token_img.height > img.height {
                img.height * 2
            } else {
                img.height
            };
            img.grow(new_width, new_height);

            img.blit(current_x, current_y, &token_img);
            current_x += token_img.width + 1;
            line_height = max(line_height, token_img.height);
        }
        current_y += line_height + 1;
        current_x = origin_x;
    }
    matrix_to_png(&img, tbd::project_path(Path::new("outputs")
        .join("galaxy.png")))
}