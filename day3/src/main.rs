#![cfg_attr(test, feature(test))]
#![feature(array_chunks)]

use std::collections::HashSet;

type In = &'static str;
type Out = u32;

fn parse(s: &'static str) -> In {
    s
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32 - 'a' as u32) + 1,
        'A'..='Z' => (c as u32 - 'A' as u32) + 27,
        _ => 0,
    }
}

fn part1(n: &[In]) -> Out {
    let mut sum = 0;
    for s in n {
        let (a, b) = s.split_at(s.len() / 2);
        let aa = a.chars().collect::<HashSet<_>>();
        let bb = b.chars().collect::<HashSet<_>>();
        let c = aa.intersection(&bb).next().unwrap();
        sum += priority(*c);
    }
    sum
}

fn part2(n: &[In]) -> Out {
    let mut sum = 0;
    for [a, b, c] in n.array_chunks() {
        let aa = a.chars().collect::<HashSet<_>>();
        let bb = b.chars().collect::<HashSet<_>>();
        let cc = c.chars().collect::<HashSet<_>>();
        let ab = aa.intersection(&bb).copied().collect::<HashSet<_>>();
        let abc = ab.intersection(&cc).next().unwrap();
        sum += priority(*abc);
    }
    sum
}

util::register!(parse, part1, part2);
