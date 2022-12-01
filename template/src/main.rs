#![cfg_attr(test, feature(test))]

type In = i32;
type Out = i64;

fn parse(s: &'static str) -> In {
    util::poarse(s)
}

fn part1(n: &[In]) -> Out {
    n.iter().sum::<i32>() as i64
}

fn part2(n: &[In]) -> Out {
    n.iter().product::<i32>() as i64
}

util::register!(parse, part1, part2);
