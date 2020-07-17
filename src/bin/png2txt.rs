use tbd::png_files as pngs;
use tbd::img_matrix::ImgMatrix;
use tbd::operations::read_operations;
use tbd::parse_image::{ show_image, parse_image };

use std::path::{Path};

// optional: add number of the message to parse as a command line argument

fn main() {
    let mut filename = "message2.png".to_string();
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        filename = format!("message{}.png", args[1]);
    }
    let v = pngs::bordered_png_to_matrix(Path::new("data")
        .join("messages")
        .join(filename));
    let mut unidentified: Vec<ImgMatrix> = Vec::new();
    let operations = read_operations();

    println!("{}", parse_image(&v, &mut unidentified, &operations));
    println!("Unidentified tokens:");
    for (i, m) in unidentified.iter().enumerate() {
        println!("\n?{}", i);
        show_image(m);
    }
}
