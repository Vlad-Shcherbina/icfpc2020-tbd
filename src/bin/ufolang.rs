use tbd::{project_path, tree::Tree};
use std::{collections::HashMap, rc::Rc};

fn ap_to_none(s: &str) -> Option<&str> {
    if s == "ap" { None } else { Some(s) }
}

fn parse(src: &str) -> Vec<(&str, Tree<&str>)> {
    let mut result = Vec::new();
    for line in src.split_terminator('\n') {
        dbg!(&line);
        let mut it = line.split_whitespace();
        let function = it.next().unwrap();
        let eq = it.next().unwrap();
        assert_eq!(eq, "=");
        let mut it = it.map(ap_to_none);
        let tree = Tree::parse(&mut it);
        assert!(it.next().is_none());
        result.push((function, tree));
    }
    result
}

// For combinator X that takes more than one argument,
// X1 is X partially applied to one argument,
// X2 is X partially applied to two.
// For example,
//   S a b c  =>  S1(a) b c  =>  S2(a, b) c  =>  (a c) (b c)
#[derive(Debug, PartialEq)]
enum Value {
    App(Rc<Value>, Rc<Value>),
    Number(i64),
    Use(usize),  // reference to a definition like ':1234'
    K,  // aka True
    C, C1(Rc<Value>), C2(Rc<Value>),
    B, B1(Rc<Value>), B2(Rc<Value>, Rc<Value>),
    S, S1(Rc<Value>), S2(Rc<Value>, Rc<Value>),
    I,
    Nil,
    IsNil,
    Cons,
    Car,
    Cdr,
    Neg,
    Eq, Eq1(Rc<Value>),
    Mul,
    Add, Add1(Rc<Value>),
    Lt,
    Div,
}
use Value::*;

struct Context {
    names: Vec<String>,
    defs: Vec<Rc<Value>>,
    name_to_def_idx: HashMap<String, usize>,
}

impl Context {
    fn new(src: &[(&str, Tree<&str>)]) -> Self {
        let mut names = Vec::new();
        let mut name_to_def_idx = HashMap::new();
        for (name, _) in src {
            name_to_def_idx.entry(name.to_string())
                .and_modify(|_| panic!())
                .or_insert(names.len());
            names.push(name.to_string());
        }

        let mut defs = Vec::new();
        for (_, tree) in src {
            let value = tree_to_value(tree, &mut |&s| {
                if let Ok(num) = s.parse() {
                    return Rc::new(Number(num))
                }
                if s.starts_with(':') {
                    return Rc::new(Use(name_to_def_idx[s]))
                }
                match s {
                    "cons" => Rc::new(Cons),
                    "nil" => Rc::new(Nil),
                    "neg" => Rc::new(Neg),
                    "c" => Rc::new(C),
                    "b" => Rc::new(B),
                    "s" => Rc::new(S),
                    "isnil" => Rc::new(IsNil),
                    "car" => Rc::new(Car),
                    "cdr" => Rc::new(Cdr),
                    "eq" => Rc::new(Eq),
                    "mul" => Rc::new(Mul),
                    "add" => Rc::new(Add),
                    "lt" => Rc::new(Lt),
                    "div" => Rc::new(Div),
                    "i" => Rc::new(I),
                    "t" => Rc::new(K),
                    _ => panic!("{:?}", s)
                }
            });
            defs.push(value);
        }
        Context {
            names,
            defs,
            name_to_def_idx,
        }
    }
}

// never returns App, so eval() is idempotent
fn eval(value: Rc<Value>, ctx: &Context) -> Rc<Value> {
    // dbg!(&value);
    match *value {
        App(ref f, ref x) =>
            apply(
                eval(Rc::clone(f), ctx),
                eval(Rc::clone(x), ctx),
                ctx),
        Use(def_idx) => {
            println!("calling {}...", ctx.names[def_idx]);
            eval(ctx.defs[def_idx].clone(), ctx)
        }
        _ => value,
    }
}

// never returns App
fn apply(f: Rc<Value>, x: Rc<Value>, ctx: &Context) -> Rc<Value> {
    // todo!()
    match *f {
        B => Rc::new(B1(x)),
        B1(ref a) => Rc::new(B2(a.clone(), x)),

        C => Rc::new(C1(x)),

        Eq => Rc::new(Eq1(x)),

        S => Rc::new(S1(x)),

        Add => Rc::new(Add1(x)),
        Add1(ref a) => {
            let a = match **a {
                Number(a) => a,
                _ => panic!("can't add {:?}", a),
            };
            let b = match *x {
                Number(b) => b,
                _ => panic!("canot add {:?}", x),
            };
            Rc::new(Number(a + b))
        }

        _ => panic!("{:?}", f),
    }
}

fn tree_to_value<T>(tree: &Tree<T>, leaf_to_value: &mut dyn FnMut(&T) -> Rc<Value>) -> Rc<Value> {
    match tree {
        Tree::Leaf(leaf) => leaf_to_value(leaf),
        Tree::App(f, x) => Rc::new(App(
            tree_to_value(f, leaf_to_value),
            tree_to_value(x, leaf_to_value),
        )),
    }
}

fn main1() {
    // let src = std::fs::read_to_string(project_path("data/messages/galaxy.txt")).unwrap();
    // let src = "\
    // :1141 = ap ap c b ap ap s ap ap b c ap ap b ap b b ap eq 0 ap ap b ap c :1141 ap add -1
    // main = ap :1141 0
    // ".trim_end();
    let src = "\
    main = ap ap add 20 30
    ".trim_end();
    let src = parse(&src);
    let ctx = Context::new(&src);
    let x = ctx.defs[ctx.name_to_def_idx["main"]].clone();
    let res = eval(x, &ctx);
    dbg!(res);

    // let galaxy = ctx.defs[ctx.name_to_def_idx["galaxy"]].clone();
    // let x = Rc::new(App(galaxy, Rc::new(Number(0))));
    // eval(x, &ctx);
}

fn main() {
    std::thread::Builder::new().stack_size(500 * 1024 * 1024)
        .spawn(main1).unwrap()
        .join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Value::*;

    fn run_snippet(src: &str) -> Rc<Value> {
        let src = parse(src.trim_end());
        let ctx = Context::new(&src);
        let x = ctx.defs[ctx.name_to_def_idx["main"]].clone();
        eval(x, &ctx)
    }

    #[test]
    fn test_add() {
        assert_eq!(run_snippet("\
            main = ap ap add 20 30
        "), Rc::new(Number(50)));

        // main = add 100 (add 20 3)
        assert_eq!(run_snippet("\
            main = ap ap add 100 ap ap add 20 3
        "), Rc::new(Number(123)));

        // main = add (add 100 20) 3
        assert_eq!(run_snippet("\
            main = ap ap add ap ap add 100 20 3
        "), Rc::new(Number(123)));
    }
}
