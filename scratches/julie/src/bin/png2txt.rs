extern crate png;

use std::fs::File;

struct NumPlate { 
    value: i64, 
    width: usize, height: usize,
}

fn main() {
    let v = read_file("data/message8.png");
    let mut top = 2;
    let mut bottom = top;
    while bottom < v.len() {
        bottom += 1;
        if !is_divider(&v, bottom) { continue; }
        while(is_divider(&v, bottom)) { bottom += 1 };
        parse_line(&v, 2, top, bottom - top);
        top = bottom;
    }
}

fn read_file(path: &str) -> Vec<Vec<i64>> {
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let (width, height) = (info.width as usize / 4, info.height as usize / 4);
    let mut v: Vec<Vec<i64>> = vec![vec![0; height]; width];
    for x in 0..width {
        for y in 0..height {
            v[x][y] = if buf[(y * info.width as usize + x) * 4 * 4] == 0 { 0 } else { 1 };
        }
    }
    v
}

fn is_divider(img: &Vec<Vec<i64>>, y: usize) -> bool {
    for x in 1..img.len() - 1 {
        if img[x][y] == 1 {
            return false;
        }
    }
    true
}

fn parse_line(img: &Vec<Vec<i64>>, left: usize, top: usize, bottom: usize) {
    let mut x = left;
    print!("Line {}: ", top);
    while x < img.len() - 1 {
        if is_number(img, x, top) {
            let n = number(img, x, top);
            // assert!(top + n.height <= bottom);
            print!("{} ", n.value);
            x += n.width + 1;
        }
        else {
            x += 1;
        }
    }
    println!();
}

fn is_number(img : &Vec<Vec<i64>>, x: usize, y: usize) -> bool {
    img[x][y] == 0 && img[x + 1][y] == 1 && img[x][y + 1] == 1
}

fn number(img : &Vec<Vec<i64>>, x: usize, y: usize) -> NumPlate {
    let mut base = 0;
    for i in 1.. {
        if img[x + i][y] == 0 { break; }
        base = i;
    }

    let sgn = if (img[x][y + base + 1]) == 0 { 1 } else { -1 };
    let mut digit = 1;
    let mut n = 0;
    for i in 0..base {
        for j in 0..base {
            n += img[x + j + 1][y + i + 1] * digit;
            digit *= 2;
        }
    }
    NumPlate{value: n * sgn, width: base, height: base + ((sgn - 1) / (-2)) as usize}
}
