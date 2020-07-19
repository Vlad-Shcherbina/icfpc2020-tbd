use tbd::png_files as pngs;
use tbd::img_matrix::ImgMatrix;
use tbd::operations::read_operations;
use tbd::parse_image::{ parse_image };

use std::path::{Path};

// optional: add filename w/o .png from data

fn read_some_file(path: &std::path::Path) {
    let v = pngs::borderless_png_to_matrix(path);
    let mut unidentified: Vec<ImgMatrix> = Vec::new();
    let operations = read_operations();

    let lines = parse_image(&v, &mut unidentified, &operations);
    for line in lines {
        for token in line {
            print!("{} ", token);
        }
        println!();
    }
    println!("Unidentified tokens:");
    for (i, m) in unidentified.iter().enumerate() {
        println!("\n?{}", i);
        print!("{}", m);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = if args.len() > 1 { format!("{}.png", &args[1]) } else { "canvas.png".to_string() };
    let pth = Path::new("data")
        .join("screens")
        .join(filename);
    read_some_file(&pth);
}
