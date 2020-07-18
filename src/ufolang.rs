use crate::tree::Tree;
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
pub enum Value {
    App(Rc<Value>, Rc<Value>),
    Number(i64),
    Use(usize),  // reference to a definition like ':1234'
    K,  // aka True
    K1(Rc<Value>),
    C, C1(Rc<Value>), C2(Rc<Value>, Rc<Value>),
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
    Mul, Mul1(Rc<Value>),
    Add, Add1(Rc<Value>),
    Lt,
    Div,
    False, False1,
}
use Value::*;

impl Value {
    fn try_as_number(&self) -> Option<i64> {
        match self {
            Number(x) => Some(*x),
            _ => None,
        }
    }
}

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
                if let Some(&def_idx) = name_to_def_idx.get(s) {
                    return Rc::new(Use(def_idx));
                }
                // if s.starts_with(':') {
                //     return Rc::new(Use(name_to_def_idx[s]))
                // }
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
                Rc::clone(x),
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
        K => Rc::new(K1(x)),
        K1(ref a) => eval(a.clone(), ctx),

        B => Rc::new(B1(x)),
        B1(ref a) => Rc::new(B2(a.clone(), x)),
        B2(ref a, ref b) => {
            let bx = Rc::new(App(b.clone(), x));
            eval(Rc::new(App(a.clone(), bx)), ctx)
        }

        C => Rc::new(C1(x)),
        C1(ref a) => Rc::new(C2(a.clone(), x)),
        C2(ref a, ref b) => {
            let ax = Rc::new(App(a.clone(), x));
            eval(Rc::new(App(ax, b.clone())), ctx)
        }

        Eq => Rc::new(Eq1(x)),
        Eq1(ref a) => {
            let a = eval(a.clone(), ctx);
            let x = eval(x, ctx);
            if a.try_as_number().unwrap() == x.try_as_number().unwrap() {
                Rc::new(K)
            } else {
                Rc::new(False)
            }
        }

        S => Rc::new(S1(x)),
        S1(ref a) => Rc::new(S2(a.clone(), x)),
        S2(ref a, ref b) => {
            let a = a.clone();
            let b = b.clone();

            let ac = Rc::new(App(a, x.clone()));
            let bc = Rc::new(App(b, x));
            eval(Rc::new(App(ac, bc)), ctx)
        }

        Add => Rc::new(Add1(x)),
        Add1(ref a) => {
            let a = eval(a.clone(), ctx);
            let b = eval(x, ctx);
            Rc::new(Number(a.try_as_number().unwrap() + b.try_as_number().unwrap()))
        }

        Mul => Rc::new(Mul1(x)),
        Mul1(ref a) => {
            let a = eval(a.clone(), ctx);
            let b = eval(x, ctx);
            Rc::new(Number(a.try_as_number().unwrap() * b.try_as_number().unwrap()))
        }

        False => Rc::new(False1),
        False1 => eval(x, &ctx),

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

/*fn main1() {
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
}*/

#[cfg(test)]
mod tests {
    use super::*;

    fn run_snippet(src: &str) -> Rc<Value> {
        let src = parse(src.trim_end());
        let ctx = Context::new(&src);
        let x = ctx.defs[ctx.name_to_def_idx["main"]].clone();
        eval(x, &ctx)
    }

    #[test]
    fn add() {
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

    #[test]
    fn pwr2() {
        assert_eq!(run_snippet("\
            pwr2   =   ap ap s ap ap c ap eq 0 1 ap ap b ap mul 2 ap ap b pwr2 ap add -1
            main = ap pwr2 7
        "), Rc::new(Number(128)));
    }
}
