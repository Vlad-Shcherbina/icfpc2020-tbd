use std::io::Write;
use tbd::{img_matrix::ImgMatrix, project_path, parse_image::{Token, parse_image}};
use tbd::operations::APP_OPERATION_NAME;
use tbd::tree::Tree;

fn app_to_none(t: Token) -> Option<Token> {
    match t {
        Token::Operation(s) if s == APP_OPERATION_NAME => None,
        _ => Some(t),
    }
}

fn main() {
    let fout = std::fs::File::create(project_path("outputs/full_transcript.html")).unwrap();
    let mut fout = std::io::BufWriter::new(fout);

    let operations = tbd::operations::read_operations();
    let mut unidentified: Vec<ImgMatrix> = Vec::new();
    for i in 2..=42 {
        println!("{}", i);
        writeln!(fout, "<h3>Message {}</h3>", i).unwrap();
        writeln!(fout, "<img src='../data/messages/message{}.png'><br>", i).unwrap();

        match i {
            15 => {
                writeln!(fout, "not supported by the parser").unwrap();
                continue;
            }
            _ => {}
        }

        let matrix = tbd::png_files::bordered_png_to_matrix(
            format!("data/messages/message{}.png", i));

        let mut s = String::new();
        let mut hs = String::new();
        for line in parse_image(&matrix, &mut unidentified, &operations) {
            use std::fmt::Write;
            for token in &line {
                write!(s, "{} ", token).unwrap();
            }
            writeln!(s).unwrap();

            if line == [Token::Operation(APP_OPERATION_NAME.to_string())] {
                writeln!(hs, "app").unwrap();
                continue;
            }
            let trees = Tree::parse_many(&mut line.into_iter().map(app_to_none));
            for tree in trees {
                write!(hs, "{} ", tree.to_haskell(&|t| format!("{}", t), false)).unwrap();
            }
            writeln!(hs).unwrap();
        }

        writeln!(fout, "<pre>{}</pre>", s).unwrap();
        writeln!(fout, "haskell notation:").unwrap();
        writeln!(fout, "<pre>{}</pre>", hs).unwrap();
    }

    writeln!(fout, "<h3>Unindentified</h3>").unwrap();
    for (i, m) in unidentified.iter().enumerate() {
        writeln!(fout, "<pre>?{}\n{}</pre>", i, m).unwrap();
    }

    println!("See outputs/full_transcript.html");
}
