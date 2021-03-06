use crate::img_matrix::ImgMatrix;
use crate::img_matrix::Coord;
use crate::project_path;

use std::fs::{create_dir_all, File, read_dir, remove_file};
use std::path::Path;

const COLOR_SIZE: usize = 4;  // 4 components of each color in png
const CELL_SIZE: usize = 4;
const BLACK: [u8; 4] = [0, 0, 0, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];


// takes path to an image and turns it into WIDTH x HEIGHT bool matrix
pub fn borderless_png_to_matrix(path: impl AsRef<Path>) -> ImgMatrix {
    let decoder = png::Decoder::new(File::open(project_path(path)).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let pixel_size = 1;

    let width = info.width as usize / pixel_size;
    let height = info.height as usize / pixel_size;
    let mut v: ImgMatrix = ImgMatrix::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let coord = (y * (info.width as usize) + x) * pixel_size * COLOR_SIZE;
            v[Coord {x, y}] = if buf[coord] == 0 { 0 } else { 1 };
        }
    }
    v
}

// takes path to an image and turns it into WIDTH x HEIGHT bool matrix
pub fn bordered_png_to_matrix(path: impl AsRef<Path>) -> ImgMatrix {
    let decoder = png::Decoder::new(File::open(project_path(path)).unwrap());
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

pub fn matrix_to_png(matrix: &ImgMatrix, path: impl AsRef<Path>) {
    let image_width = matrix.width * CELL_SIZE;
    let image_height = matrix.height * CELL_SIZE;
    let mut encoder = png::Encoder::new(File::create(path).expect("No such file"),
                                        image_width as u32,
                                        image_height as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    let mut image_data = vec![0; matrix.width * matrix.height * CELL_SIZE * CELL_SIZE * COLOR_SIZE];
    for y in 0..matrix.height {
        for x in 0..matrix.width {
            for cell_y in 0..CELL_SIZE {
                for cell_x in 0..CELL_SIZE {
                    let y_pixel = y * CELL_SIZE + cell_y;
                    let x_pixel = x * CELL_SIZE + cell_x;
                    let offset = (y_pixel * image_width + x_pixel) * COLOR_SIZE;
                    if matrix[Coord { x, y }] == 1 {
                        for c in 0..COLOR_SIZE {
                            image_data[offset + c] = WHITE[c];
                        }
                    } else {
                        for c in 0..COLOR_SIZE {
                            image_data[offset + c] = BLACK[c];
                        }
                    };
                }
            }
        }
    }
    writer.write_image_data(&image_data).expect("Failed to write");
}

pub fn borderize(m: &ImgMatrix) -> ImgMatrix {
    let mut result = ImgMatrix::new(m.width + 4, m.height + 4);
    for x in 0..(m.width + 4) {
        result[Coord { x, y: 0 }] = 1;
        result[Coord { x, y: m.height + 3 }] = 1;
    }

    for y in 0..(m.height + 4) {
        result[Coord { x: 0, y }] = 1;
        result[Coord { x: m.width + 3, y }] = 1;
    }

    result.blit(2, 2, m);
    result
}

pub fn matrices_to_png(matrices: &Vec<ImgMatrix>, path: impl AsRef<Path>) {
    create_dir_all(&path).unwrap();

    for entry in read_dir(&path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().extension().map(|s| s == "png").unwrap_or(false) {
            remove_file(entry.path()).unwrap();
        }
    }

    for (i, m) in matrices.iter().enumerate() {
        matrix_to_png(&borderize(m), path.as_ref().join(format!("image{:03}.png", i)));
    }
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
