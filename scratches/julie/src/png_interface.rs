use crate::img_matrix::ImgMatrix;
use crate::img_matrix::Coord;

use std::fs::File;
use std::collections::HashMap;
use std::path::Path;

const COLOR_SIZE: usize = 4;  // 4 components of each color in png


// takes path to an image and turns it into WIDTH x HEIGHT bool matrix
pub fn bordered_png_to_matrix(path: impl AsRef<Path>) -> ImgMatrix {
    let decoder = png::Decoder::new(File::open(tbd::project_path(path)).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    // assuming there is a border, find out the size of the "pixel" (4x4 on messages 2-13)
    let mut pixel_size = 1;
    while buf[(pixel_size * info.width as usize + pixel_size) * COLOR_SIZE] != 0 { 
        pixel_size += 1
    }

    // -2 - excluding the border
    let width = info.width as usize / pixel_size - 2;
    let height = info.height as usize / pixel_size - 2;
    let mut v: ImgMatrix = ImgMatrix::new(width, height);
    for x in 0..width {
        for y in 0..height {
            // + 1 due to border
            let coord = ((y + 1) * (info.width as usize) + (x + 1)) * pixel_size * COLOR_SIZE;
            v[Coord {x, y}] = if buf[coord] == 0 { 0 } else { 1 };
        }
    }
    v
}

// Filename without .png
// pub fn matrix_to_bordered_png(img: &ImgMatrix, pixel_size: usize, filename: &str) {
//     let path = format!("\\outputs\\{}.png", filename);
//     let encoder = png::Encoder::new(
//                   std::io::BufWriter(
//                   File::create(tbd::project_path(path)).unwrap()
//                   ), img.len(), img[0].len());
// }


// NO USE until it read copies of pngs
// fn sample_png_to_matrix(path: &str) -> ImgMatrix {
//     let decoder = png::Decoder::new(File::open(path).unwrap());
//     let (info, mut reader) = decoder.read_info().unwrap();
//     let mut buf = vec![0; info.buffer_size()];
//     reader.next_frame(&mut buf).unwrap();

//     let pixel_size = 4;
    
//     let width = info.width as usize / pixel_size;
//     let height = info.height as usize / pixel_size;
//     let mut v: ImgMatrix = vec![vec![false; height]; width];  
//     for x in 0..width {
//         for y in 0..height {
//             let coord = (y * (info.width as usize) + x) * pixel_size * COLOR_SIZE;
//             v[x][y] = if buf[coord] == 0 { false } else { true };
//             println!("{}, {} : {}", x, y, buf[coord])
//         }
//     }
//     v
// }
