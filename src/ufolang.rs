use crate::{project_path, tree::Tree};
use std::{collections::HashMap, rc::Rc, convert::TryFrom, cell::RefCell};
use crate::squiggle::Data;
use crate::{webapi::aliens_send, img_matrix::*};

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
    Number(i128),
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

    Thunk(RefCell<Rc<Value>>),
}
use Value::*;

impl Value {
    fn try_as_number(&self) -> Option<i128> {
        match self {
            Number(x) => Some(*x),
            _ => None,
        }
    }
}

impl TryFrom<&Value> for Data {
    type Error = ();

    fn try_from(val: &Value) -> Result<Self, Self::Error> {
        match val {
            Number(i) => Ok(Data::Number(*i)),
            Nil => Ok(Data::Nil),
            Pair(left, right) => Ok(Data::Cons(
                Box::new(Self::try_from(left.as_ref())?),
                Box::new(Self::try_from(right.as_ref())?))),
            _ => Err(())
        }
    }
}

impl From<&Data> for Value {
    fn from(val: &Data) -> Self {
        match val {
            Data::Nil => Nil,
            Data::Number(value) => Number(*value),
            Data::Cons(left, right) => Value::Pair(
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

fn eval_draw(value: Data) -> ImgMatrix {
    struct Point2d {
        pub x: i128,
        pub y: i128,
    }
    let points = value.into_vec();
    if points.is_empty() {
        return ImgMatrix::new(1, 1)
    }
    let points: Vec<Point2d> = points.into_iter().map(|p| match p {
        Data::Cons(car, cdr) => Point2d {
            x: car.try_as_number().unwrap(),
            y: cdr.try_as_number().unwrap()
        },
        _ => panic!("{:?}", p)
    }).collect();
    let min_x = points.iter().map(|it| it.x).min().unwrap();
    let min_y = points.iter().map(|it| it.y).min().unwrap();
    let max_x = points.iter().map(|it| it.x).max().unwrap();
    let max_y = points.iter().map(|it| it.y).max().unwrap();
    eprintln!("{} {} {} {}", min_x, min_y, max_x, max_y);

    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut image = ImgMatrix::new(width as usize, height as usize);
    for p in points {
        image[Coord {x: (p.x - min_x) as usize, y: (p.y - min_y) as usize}] = 1;
    }
    image
}

pub fn eval_multidraw(value: Data) -> Vec<ImgMatrix> {
    value.into_vec().into_iter().map(|it| eval_draw(it)).collect()
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
            // println!("calling {}...", ctx.names[def_idx]);
            eval(ctx.defs[def_idx].clone(), ctx)
        }
        Thunk(ref cell) => {
            let x = Rc::clone(&*cell.borrow());
            let result = eval(x, ctx);
            *cell.borrow_mut() = result.clone();
            result
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

            let c = Rc::new(Thunk(RefCell::new(x)));

            let ac = Rc::new(App(a, c.clone()));
            let bc = Rc::new(App(b, c));
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

        Car => eval(Rc::new(App(x, Rc::new(K))), ctx),
        Cdr => eval(Rc::new(App(x, Rc::new(False))), ctx),

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
    continue_flag: i128,
    new_state: Data,
    data_out: Data,
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

    pub fn invoke(&self, internal_state: &Data, data_in: &Data) -> ProtocolResponse {
        let internal_state = Rc::new(internal_state.into());
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
            Pair(new_state, tail2) => (new_state, tail2.clone()),
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
        let new_state = Data::try_from(new_state.as_ref()).unwrap();
        let data_out = Data::try_from(data_out.as_ref()).unwrap();

        ProtocolResponse {
            continue_flag,
            new_state,
            data_out,
        }
    }

    pub fn interact(&self, initial_state: Data, mut data_in: Data) -> InteractResult {
        let mut state = initial_state;
        loop {
            let resp = self.invoke(&state, &data_in);
            if resp.continue_flag == 0 {
                return InteractResult {
                    final_state: resp.new_state,
                    data_out_to_multipledraw: resp.data_out,
                }
            }
            state = resp.new_state;
            eprintln!("sending to aliens: {:?}", resp.data_out);
            data_in = aliens_send(resp.data_out);
            eprintln!("received from aliens: {:?}", data_in);
        }
    }
}

#[derive(Debug)]
pub struct InteractResult {
    pub final_state: Data,
    pub data_out_to_multipledraw: Data,
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
    fn car_cdr() {
        assert_eq!(run_snippet("\
            main = ap ap cons ap car ap ap cons 1 2 ap cdr ap ap cons 3 4
        "), Rc::new(Pair(Rc::new(Number(1)), Rc::new(Number(4)))));
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
        let resp = protocol.invoke(&Data::Nil, &42.into());
        assert_eq!(resp.continue_flag, 0);

        match resp.new_state {
            Data::Nil => {},
            _ => panic!(),
        }

        let expected_data_out = Data::make_list1(Data::make_list1(42));
        assert_eq!(resp.data_out, expected_data_out);
    }

    #[test]
    fn statelessdraw_interact() {
        // example from https://message-from-space.readthedocs.io/en/latest/message40.html
        // ap ap ap interact statelessdraw nil ap ap vec 2 3 = ( nil , ( [2,3] ) )
        let protocol = Protocol::from_snippet("\
        main = ap ap c ap ap b b ap ap b ap b ap cons 0 ap ap c ap ap b b cons ap ap c cons nil ap ap c ap ap b cons ap ap c cons nil nil
        ");
        let res = protocol.interact(Data::Nil, Data::make_cons(2, 3));
        dbg!(&res);

        match res.final_state {
            Data::Nil => {}
            _ => panic!(),
        }

        let expected = Data::make_list1(Data::make_list1(Data::make_cons(2, 3)));
        assert_eq!(res.data_out_to_multipledraw, expected);
    }

    #[test]
    fn statefuldraw_interact() {
        // example from https://message-from-space.readthedocs.io/en/latest/message41.html

        let protocol = Protocol::from_snippet("\
        main = ap ap b ap b ap ap s ap ap b ap b ap cons 0 ap ap c ap ap b b cons ap ap c cons nil ap ap c cons nil ap c cons
        ");
        let res = protocol.interact(Data::Nil, Data::make_cons(0, 0));
        assert_eq!(res.final_state.to_pretty_string(), "[(0, 0)]");
        assert_eq!(res.data_out_to_multipledraw.to_pretty_string(), "[[(0, 0)]]");

        let res = protocol.interact(res.final_state, Data::make_cons(2, 3));
        assert_eq!(res.data_out_to_multipledraw.to_pretty_string(), "[[(2, 3), (0, 0)]]");

        let res = protocol.interact(res.final_state, Data::make_cons(1, 2));
        assert_eq!(res.data_out_to_multipledraw.to_pretty_string(), "[[(1, 2), (2, 3), (0, 0)]]");
    }
}
