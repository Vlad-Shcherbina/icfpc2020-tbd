//use lambda_calculus::*;
use regex::Regex;
use lamcal::*;
use tbd::project_path;
use std::io::Write;

fn reduce_line(source: String) -> Term {
    let mut replaced = source;

    replaced = Regex::new(r"\bap\b").unwrap().replace_all(replaced.as_ref(), "(λx.λy.x y)").into_owned();


    replaced = Regex::new(r"\bb\b").unwrap().replace_all(replaced.as_ref(), "(λx.λy.λz.x (y z))").into_owned();
    replaced = Regex::new(r"\bc\b").unwrap().replace_all(replaced.as_ref(), "(λx.λy.λz.x z y)").into_owned();
    replaced = Regex::new(r"\bs\b").unwrap().replace_all(replaced.as_ref(), "(λx.λy.λz.x z (y z))").into_owned();
    replaced = Regex::new(r"\bi\b").unwrap().replace_all(replaced.as_ref(), "(λx.x)").into_owned();

    replaced = Regex::new(r"\bt\b").unwrap().replace_all(replaced.as_ref(), "(λx.λy.x)").into_owned();
    replaced = Regex::new(r"\bf\b").unwrap().replace_all(replaced.as_ref(), "(λx.λy.y)").into_owned();
    
    replaced = Regex::new(r"-(\d+)").unwrap().replace_all(replaced.as_ref(), "neg_$1").into_owned();
    replaced = Regex::new(r":").unwrap().replace_all(replaced.as_ref(), "call_").into_owned();


    let res = parse(replaced.chars()).unwrap();
    return reduce::<HybridApplicativeOrder<Enumerate>>(&res);
}

fn main1() {
    let src = std::fs::read_to_string(project_path("outputs/galaxy_2.hs")).unwrap();
    let mut fout = std::fs::File::create(project_path("outputs/galaxy_2.lambda")).unwrap();

    for line in src.split_terminator('\n') {
        let mut it = line.split_whitespace();
        let function = it.next().unwrap();
        let eq = it.next().unwrap();
        assert_eq!(eq, "=");

        let body = it.collect::<Vec<&str>>().join(" ");

        println!("{}", function);

        writeln!(fout, "{} = {}", function, reduce_line(body));
    }
    println!("see outputs/galaxy_2.lambda");
}

fn main() {
    std::thread::Builder::new().stack_size(32 * 1024 * 1024)
        .spawn(main1).unwrap()
        .join().unwrap();
}