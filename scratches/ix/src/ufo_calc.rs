#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
enum Expr {
    Num(i32),
    Var(char),
    Let {var: char, val: i32, expr: Box<Expr>},
    App {op: Op, args: Vec<Box<Expr>>}
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul
}

impl Op {
    fn apply(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Mul => x * y,
        }
    }

    fn identity(&self) -> i32 {
        match self {
            Self::Add => 0,
            Self::Mul => 1,
        }
    }
}

#[derive(Debug)]
enum StackFrame {
    NewState(char),
    OpExpr {op: Op, val: i32}
}

// needs a correct well-formed expression -- that's on parsing
fn calc(e: Expr) -> i32 {
    let mut context = HashMap::new();

    reduce(e, &mut context)

}

fn reduce(e: Expr, context: &mut HashMap<char, Vec<i32>>) -> i32 {

    let res = match e {
        Expr::Num(val) => val,
        Expr::Var(var) => if let Some(cs) = context.get(&var) {
            if let Some(val) = cs.last() { *val }
            else { panic!("malformed expr") }
        } else { panic!("malformed expr") }
        Expr::Let {var, val, expr} => {
            update_context(context, var, val);
            let res = reduce(*expr, context);
            if let Some(cs) = context.get_mut(&var) { cs.pop(); }
            res
        },
        Expr::App {op, args} => {
            let mut tmp_acc: i32 = op.identity();
            for arg in args {
                tmp_acc = op.apply(tmp_acc, reduce(*arg, context));
            }
            tmp_acc
        }
    };
    res
}

fn update_context(context: &mut HashMap<char, Vec<i32>>, var: char, val: i32) {
    context.entry(var).or_insert(Vec::new()).push(val);
}

// At this point, the expr is forced to be a Num.
fn collapse_stack(res: Expr, stack: &mut Vec<StackFrame>, context: &mut HashMap<char, Vec<i32>>) -> Expr {
    if let Expr::Num(mut acc) = res {
        while let Some(top) = stack.pop() {
            match top {
                StackFrame::NewState(var) => { if let Some(cs) = context.get_mut(&var) { cs.pop(); }},
                StackFrame::OpExpr {op, val} => { acc = op.apply(acc, val); },
            }
        }
        return Expr::Num(acc)
    }

    res
}

pub fn test() {
    let e: Expr = Expr::App {
        op: Op::Add, args: vec![Box::new(Expr::Num(7)), Box::new( Expr::Let { var: 'x', val: 5, expr: Box::new( Expr::Var('x') ) } )]
    };
    println!("{:?}", e);

    println!("(+ 7 (let x 5 x)) == 12? {}", calc(e) == 12);

}
