#![cfg_attr(test, feature(test))]

use std::collections::HashSet;

type In = &'static str;
type Out = usize;

fn parse(s: &'static str) -> In {
    s
}

fn part1(&n: &In) -> Out {
    n.as_bytes()
        .windows(4)
        .position(|w| w.iter().collect::<HashSet<_>>().len() == 4)
        .unwrap()
        + 4
}

fn part2(&n: &In) -> Out {
    n.as_bytes()
        .windows(14)
        .position(|w| w.iter().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        + 14
}

util::register!(parse, part1, part2, @alt);
