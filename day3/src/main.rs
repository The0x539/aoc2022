#![cfg_attr(test, feature(test))]
#![feature(array_chunks)]

use std::collections::HashSet;

type In = &'static str;
type Out = u32;

fn parse(s: &'static str) -> In {
    s
}

fn set(s: &str) -> HashSet<char> {
    s.chars().collect()
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
        let ab = &set(a) & &set(b);
        sum += priority(ab.into_iter().next().unwrap());
    }
    sum
}

fn part2(n: &[In]) -> Out {
    let mut sum = 0;
    for [a, b, c] in n.array_chunks() {
        let ab = &set(a) & &set(b);
        let abc = &ab & &set(c);
        sum += priority(abc.into_iter().next().unwrap());
    }
    sum
}

util::register!(parse, part1, part2);
