use crate::{project_path, tree::Tree};
use std::{collections::HashMap, rc::Rc, convert::TryFrom};
use crate::squiggle;
use crate::img_matrix::*;

fn ap_to_none(s: &str) -> Option<&str> {
    if s == "ap" { None } else { Some(s) }
}

fn parse(src: &str) -> Vec<(&str, Tree<&str>)> {
    let mut result = Vec::new();
    for line in src.split_terminator('\n') {
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
    Cons, Cons1(Rc<Value>),
    Pair(Rc<Value>, Rc<Value>),  // aka Cons2
    Car,
    Cdr,
    Neg,
    Eq, Eq1(Rc<Value>),
    Mul, Mul1(Rc<Value>),
    Add, Add1(Rc<Value>),
    Lt, Lt1(Rc<Value>),
    Div, Div1(Rc<Value>),
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

impl TryFrom<&Value> for squiggle::Data {
    type Error = ();

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        match val {
            Number(i) => Ok(squiggle::Data::Number(
                if *i >= 0 {squiggle::Sign::Plus} else {squiggle::Sign::Minus},
                i.abs() as u64)),
            Nil => Ok(squiggle::Data::Nil),
            Pair(left, right) => Ok(squiggle::Data::Cons(
                Box::new(Self::try_from(left.as_ref())?),
                Box::new(Self::try_from(right.as_ref())?))),
            _ => Err(())
        }
    }
}

impl From<&squiggle::Data> for Value {
    fn from(val: &squiggle::Data) -> Self {
        match val {
            squiggle::Data::Nil => Nil,
            squiggle::Data::Number(sign, value) => Number(*value as i64 * match sign {
                squiggle::Sign::Plus => 1,
                squiggle::Sign::Minus => -1,
            }),
            squiggle::Data::Cons(left, right) => Value::Pair(
                Rc::new(Self::from(left.as_ref())),
                Rc::new(Self::from(right.as_ref())))
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

fn eval_draw(value: Rc<Value>) -> ImgMatrix {
    let mut points: Vec<Coord> = Vec::new();
    let mut val = value.as_ref();
    loop {
        match val {
            Nil => break,
            Pair(ref car, ref cdr) => {
                match car.as_ref() {
                    Pair(ref car2, ref cdr2) => points.push(Coord {
                        x: car2.try_as_number().expect("not an int") as usize,
                        y: cdr2.try_as_number().expect("not an int") as usize
                    }),
                    _ => panic!("{:?}", *value)
                }
                val = cdr.as_ref();
            }
            _ => panic!("{:?}", *value)
        }
    }
    let max_x = points.iter().map(|it| it.x).max().unwrap();
    let max_y = points.iter().map(|it| it.y).max().unwrap();
    let mut image = ImgMatrix::new(max_y, max_x);
    for p in points {
        image[p] = 1;
    }
    image
}

// never returns App, so eval() is idempotent
fn eval(value: Rc<Value>, ctx: &Context) -> Rc<Value> {
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
    match *f {
        K => Rc::new(K1(x)),
        K1(ref a) => eval(a.clone(), ctx),

        I => eval(x, ctx),

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

        Lt => Rc::new(Lt1(x)),
        Lt1(ref a) => {
            let a = eval(a.clone(), ctx);
            let x = eval(x, ctx);
            if a.try_as_number().unwrap() < x.try_as_number().unwrap() {
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

        Neg => {
            let x = eval(x, ctx);
            Rc::new(Number(-x.try_as_number().unwrap()))
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

        Div => Rc::new(Div1(x)),
        Div1(ref a) => {
            let a = eval(a.clone(), ctx);
            let b = eval(x, ctx);
            // TODO: make sure that native div has the same behavior on negative numbers
            Rc::new(Number(a.try_as_number().unwrap() / b.try_as_number().unwrap()))
        }

        Cons => Rc::new(Cons1(x)),
        Cons1(ref a) => {
            let a = eval(a.clone(), ctx);
            let b = eval(x, ctx);
            Rc::new(Pair(a, b))
        }
        Pair(ref a, ref b) => {
            let xa = Rc::new(App(x, a.clone()));
            let xab = Rc::new(App(xa, b.clone()));
            eval(xab, ctx)
        }

        IsNil =>
            match &*eval(x, ctx) {
                Nil => Rc::new(K),  // true
                Pair(_, _) => Rc::new(False),
                zzz => panic!("calling isnill on {:?}", zzz),
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

pub struct Protocol {
    ctx: Context,
    entry_point: &'static str,
}

#[derive(Debug)]
pub struct ProtocolResponse {
    continue_flag: i64,
    new_state: Rc<Value>,
    data_out: squiggle::Data,
}

impl Protocol {
    pub fn from_snippet(src: &str) -> Self {
        let src = parse(src.trim_end());
        let ctx = Context::new(&src);
        Protocol {
            ctx,
            entry_point: "main",
        }
    }

    pub fn load_galaxy() -> Self {
        let src = std::fs::read_to_string(project_path("data/messages/galaxy.txt")).unwrap();
        let src = parse(&src);
        let ctx = Context::new(&src);
        Protocol {
            ctx,
            entry_point: "galaxy",
        }
    }

    pub fn invoke(&self, internal_state: Rc<Value>, data_in: &squiggle::Data) -> ProtocolResponse {
        let data_in = Rc::new(data_in.into());
        let entry_point = self.ctx.defs[self.ctx.name_to_def_idx[self.entry_point]].clone();
        let expr = Rc::new(App(Rc::new(App(entry_point, internal_state)), data_in));
        let result = eval(expr, &self.ctx);

        // continue_flag, new_state, data_out = result
        let (continue_flag, tail) = match &*result {
            Pair(continue_flag, tail) => (continue_flag, tail.clone()),
            _ => panic!("{:?}", result),
        };
        let (new_state, tail2) = match &*tail {
            Pair(new_state, tail2) => (new_state.clone(), tail2.clone()),
            _ => panic!("{:?}", tail),
        };
        let (data_out, tail3) = match &*tail2 {
            Pair(data_out, tail3) => (data_out, tail3.clone()),
            _ => panic!("{:?}", tail2),
        };
        match &*tail3 {
            Nil => {}
            _ => panic!("{:?}", tail3),
        }

        let continue_flag = continue_flag.try_as_number().unwrap();
        let data_out = squiggle::Data::try_from(data_out.as_ref()).unwrap();

        ProtocolResponse {
            continue_flag,
            new_state,
            data_out,
        }
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
    use crate::squiggle::Data;

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

    #[test]
    fn statelessdraw_invoke() {
        // example from https://message-from-space.readthedocs.io/en/latest/message40.html
        // ap ap statelessdraw x0 x1 = ( 0 , nil , ( ( x1 ) ) )
        let protocol = Protocol::from_snippet("\
        main = ap ap c ap ap b b ap ap b ap b ap cons 0 ap ap c ap ap b b cons ap ap c cons nil ap ap c ap ap b cons ap ap c cons nil nil
        ");
        // x0 = nil, x1 = 42
        let resp = protocol.invoke(Rc::new(Nil), &42.into());
        assert_eq!(resp.continue_flag, 0);

        match *resp.new_state {
            Value::Nil => {},
            _ => panic!(),
        }

        let expected_data_out = Data::make_list1(Data::make_list1(42));
        assert_eq!(resp.data_out, expected_data_out);
    }
}
