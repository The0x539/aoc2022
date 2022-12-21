#![cfg_attr(test, feature(test))]

use std::collections::HashMap;

use util::*;

type Name = &'static str;

type N = i64;

#[derive(Copy, Clone)]
enum Expr {
    Literal(N),
    Op(Oper, Name, Name),
}

#[derive(Debug, Copy, Clone)]
enum Oper {
    Add,
    Mul,
    Sub,
    Div,
}

#[derive(Copy, Clone)]
struct Def {
    name: Name,
    val: Expr,
}

type In = Def;
type Out = N;

fn parse(s: &'static str) -> In {
    let toks = s.split_whitespace().collect::<Vec<_>>();
    let name = &toks[0][..4];
    let val = if toks.len() == 2 {
        Expr::Literal(p(toks[1]))
    } else {
        let lhs = toks[1];
        let rhs = toks[3];
        let op = match toks[2] {
            "+" => Oper::Add,
            "-" => Oper::Sub,
            "*" => Oper::Mul,
            "/" => Oper::Div,
            _ => panic!(),
        };
        Expr::Op(op, lhs, rhs)
    };
    Def { name, val }
}

fn part1(n: &[In]) -> Out {
    let mut vals = HashMap::<Name, N>::new();

    for def in n {
        if let Expr::Literal(v) = def.val {
            vals.insert(def.name, v);
        }
    }

    while vals.len() < n.len() {
        for def in n {
            if let Expr::Op(op, lhs, rhs) = def.val {
                if let (Some(lhv), Some(rhv)) = (vals.get(lhs), vals.get(rhs)) {
                    let result = match op {
                        Oper::Add => lhv + rhv,
                        Oper::Mul => lhv * rhv,
                        Oper::Sub => lhv - rhv,
                        Oper::Div => lhv / rhv,
                    };

                    vals.insert(def.name, result);
                }
            }
        }
    }

    vals["root"]
}

#[derive(Debug, Clone)]
enum Mv {
    Simple(N),
    Human(Vec<(Oper, N)>),
}

impl std::ops::Add for Mv {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Mv::Simple(a), Mv::Simple(b)) => Mv::Simple(a + b),
            (Mv::Simple(s), Mv::Human(mut h)) | (Mv::Human(mut h), Mv::Simple(s)) => {
                h.push((Oper::Add, s));
                Mv::Human(h)
            }
            _ => panic!("oh no"),
        }
    }
}

impl std::ops::Sub for Mv {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Mv::Simple(a), Mv::Simple(b)) => Mv::Simple(a - b),
            (Mv::Human(mut h), Mv::Simple(s)) => {
                h.push((Oper::Sub, s));
                Mv::Human(h)
            }
            (Mv::Simple(s), Mv::Human(mut h)) => {
                h.push((Oper::Mul, -1));
                h.push((Oper::Add, s));
                Mv::Human(h)
            }
            _ => panic!("oh no"),
        }
    }
}

impl std::ops::Mul for Mv {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Mv::Simple(a), Mv::Simple(b)) => Mv::Simple(a * b),
            (Mv::Simple(s), Mv::Human(mut h)) | (Mv::Human(mut h), Mv::Simple(s)) => {
                h.push((Oper::Mul, s));
                Mv::Human(h)
            }
            _ => panic!("oh no"),
        }
    }
}

impl std::ops::Div for Mv {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Mv::Simple(a), Mv::Simple(b)) => Mv::Simple(a / b),
            (Mv::Human(mut h), Mv::Simple(s)) => {
                h.push((Oper::Div, s));
                Mv::Human(h)
            }
            _ => panic!("oh no"),
        }
    }
}

fn part2(n: &[In]) -> Out {
    let mut vals = HashMap::<Name, Mv>::new();

    for def in n {
        if let Expr::Literal(v) = def.val {
            vals.insert(def.name, Mv::Simple(v));
        }
    }

    vals.insert("humn", Mv::Human(vec![]));

    while vals.len() < n.len() {
        for def in n {
            let Expr::Op(op, lhs, rhs) = def.val else { continue };
            let (Some(lhv), Some(rhv)) = (vals.get(lhs).cloned(), vals.get(rhs).cloned()) else { continue };

            let result = match op {
                Oper::Add => lhv + rhv,
                Oper::Mul => lhv * rhv,
                Oper::Sub => lhv - rhv,
                Oper::Div => lhv / rhv,
            };

            vals.insert(def.name, result);
        }
    }

    let root_expr = n.iter().find(|d| d.name == "root").unwrap();
    let Expr::Op(_, lhs, rhs) = root_expr.val else { panic!() };

    let (human, target) = match (vals[lhs].clone(), vals[rhs].clone()) {
        (Mv::Simple(s), Mv::Human(h)) | (Mv::Human(h), Mv::Simple(s)) => (h, s),
        _ => panic!(),
    };

    let mut v = target;
    for (op, operand) in human.into_iter().rev() {
        match op {
            Oper::Add => v -= operand,
            Oper::Sub => v += operand,
            Oper::Mul => v /= operand,
            Oper::Div => v *= operand,
        }
    }

    v
}

util::register!(parse, part1, part2);
