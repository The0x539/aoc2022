#![cfg_attr(test, feature(test))]

type In = Vec<Monkey>;
type Out = u64;

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Op,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64,
}

#[derive(Copy, Clone)]
enum Op {
    Add(u64),
    Mul(u64),
    Square,
}

impl Op {
    pub fn apply(&self, n: u64) -> u64 {
        match self {
            &Op::Add(a) => n + a,
            &Op::Mul(a) => n * &a,
            Op::Square => n * n,
        }
    }
}

fn parse_monkey(mut lines: impl Iterator<Item = &'static str>) -> Option<Monkey> {
    let si_line = lines.next()?;
    let items = si_line[16..]
        .split(", ")
        .filter(|s| !s.is_empty())
        .map(util::poarse)
        .collect::<Vec<u64>>();

    let op_line = lines.next()?;
    let s = op_line.strip_prefix("Operation: new = old ")?;
    let op = if s == "* old" {
        Op::Square
    } else if s.starts_with("*") {
        Op::Mul(s[2..].parse().ok()?)
    } else {
        Op::Add(s[2..].parse().ok()?)
    };

    let test = lines.next()?.split_whitespace().last()?.parse().ok()?;
    let if_true = lines.next()?.split_whitespace().last()?.parse().ok()?;
    let if_false = lines.next()?.split_whitespace().last()?.parse().ok()?;

    lines.next();

    Some(Monkey {
        items,
        op,
        test,
        if_true,
        if_false,
        inspections: 0,
    })
}

fn parse(s: &'static str) -> In {
    let mut ms = Vec::new();
    let mut lines = s.lines().map(str::trim);

    while let Some(_) = lines.next() {
        ms.push(parse_monkey(&mut lines).unwrap());
    }

    ms
}

fn round(monkeys: &mut [Monkey], reducer: impl Fn(u64) -> u64) {
    for i in 0..monkeys.len() {
        for mut item in std::mem::take(&mut monkeys[i].items) {
            monkeys[i].inspections += 1;
            let monkey = &monkeys[i];
            item = reducer(monkey.op.apply(item));
            let monkey_index = if item % monkey.test == 0 {
                monkey.if_true
            } else {
                monkey.if_false
            };
            monkeys[monkey_index].items.push(item);
        }
    }
}

fn part1(n: &In) -> Out {
    let mut monkeys = n.clone();

    for _ in 0..20 {
        round(&mut monkeys, |n| n / 3);
    }

    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

fn part2(n: &In) -> Out {
    let mut monkeys = n.clone();

    let modulus = monkeys.iter().map(|m| m.test).product::<u64>();

    for _ in 0..10000 {
        round(&mut monkeys, |n| n % modulus);
    }

    let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    inspections.sort();
    inspections.iter().rev().take(2).product()
}

util::register!(parse, part1, part2, @alt);
