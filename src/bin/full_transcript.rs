use std::io::Write;
use tbd::{img_matrix::ImgMatrix, project_path, parse_image::parse_image};

fn main() {
    let fout = std::fs::File::create(project_path("outputs/full_transcript.html")).unwrap();
    let mut fout = std::io::BufWriter::new(fout);

    let operations = tbd::operations::read_operations();
    let mut unidentified: Vec<ImgMatrix> = Vec::new();
    for i in 2..=12 {
        writeln!(fout, "<h3>Message {}</h3>", i).unwrap();
        writeln!(fout, "<img src='../data/messages/message{}.png'><br>", i).unwrap();

        let matrix = tbd::png_files::bordered_png_to_matrix(
            format!("data/messages/message{}.png", i));
        let t = parse_image(&matrix, &mut unidentified, &operations);
        writeln!(fout, "<pre>{}</pre>", t).unwrap();
    }

    writeln!(fout, "<h3>Unindentified</h3>").unwrap();
    for (i, m) in unidentified.iter().enumerate() {
        writeln!(fout, "<pre>?{}\n{}</pre>", i, m).unwrap();
    }

    println!("See outputs/full_transcript.html");
}
