use tbd::{tree::Tree, project_path};
use std::io::Write;

fn ap_to_none(s: &str) -> Option<&str> {
    if s == "ap" { None } else { Some(s) }
}

fn main1() {
    let src = std::fs::read_to_string(project_path("data/messages/galaxy.txt")).unwrap();
    //let src = std::fs::read_to_string(project_path("data/mantycore_galaxy.txt")).unwrap();

    let fout = std::fs::File::create(project_path("outputs/galaxy_2.hs")).unwrap();
    let mut fout = std::io::BufWriter::new(fout);

    for line in src.split_terminator('\n') {
        let mut it = line.split_whitespace();
        let function = it.next().unwrap();
        let eq = it.next().unwrap();
        assert_eq!(eq, "=");
        let mut it = it.map(ap_to_none);
        print!("{}\n", function);
        let tree = Tree::parse(&mut it);
        assert!(it.next().is_none());
        writeln!(fout, "{} = {}", function, tree.better_to_haskell()).unwrap();
    }
    println!("see outputs/galaxy_2.hs");
}

fn main() {
    std::thread::Builder::new().stack_size(32 * 1024 * 1024)
        .spawn(main1).unwrap()
        .join().unwrap();
}
