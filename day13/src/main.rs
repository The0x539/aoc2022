#![cfg_attr(test, feature(test))]

use std::cmp::Ordering;

#[derive(Clone, PartialEq, Ord, Eq)]
enum Thing {
    One(u32),
    Many(Vec<Thing>),
}

impl From<u32> for Thing {
    fn from(value: u32) -> Self {
        Self::One(value)
    }
}

impl From<Vec<Thing>> for Thing {
    fn from(value: Vec<Thing>) -> Self {
        Self::Many(value)
    }
}

impl Thing {
    fn just_one(n: impl Into<Self>) -> Self {
        vec![n.into()].into()
    }
}

impl std::fmt::Debug for Thing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Thing::One(a) => a.fmt(f),
            Thing::Many(v) => v.fmt(f),
        }
    }
}

impl PartialOrd for Thing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::One(n), Self::One(m)) => n.partial_cmp(m),
            (Self::Many(v1), Self::Many(v2)) => v1.partial_cmp(v2),
            (Self::One(a), v @ Self::Many(_)) => Self::partial_cmp(&Self::just_one(*a), v),
            (v @ Self::Many(_), Self::One(a)) => Self::partial_cmp(v, &Self::just_one(*a)),
        }
    }
}

type In = Option<Thing>;
type Out = usize;

fn parse_packet(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Thing {
    match chars.next().unwrap() {
        '[' => {
            let mut things = vec![];
            if chars.peek() == Some(&']') {
                return Thing::Many(things);
            }

            loop {
                things.push(parse_packet(chars));
                match chars.next() {
                    Some(']') => return Thing::Many(things),
                    Some(',') => continue,
                    c => panic!("oh nyo: {c:?}"),
                }
            }
        }
        c @ '0'..='9' => {
            let mut n = c.to_digit(10).unwrap();
            while let Some(c @ '0'..='9') = chars.peek().copied() {
                n = 10 * n + c.to_digit(10).unwrap();
                chars.next();
            }
            Thing::One(n)
        }
        c => panic!("oh no: {c:?}"),
    }
}

fn parse(s: &'static str) -> In {
    if s.is_empty() {
        return None;
    }

    Some(parse_packet(&mut s.chars().peekable()))
}

fn part1(packets: &[In]) -> Out {
    let mut sum = 0;

    for i in 0..(packets.len() / 3) {
        let a = &packets[i * 3];
        let b = &packets[i * 3 + 1];
        if a <= b {
            sum += i + 1;
        }
    }

    sum
}

fn part2(packets: &[In]) -> Out {
    let a = Thing::just_one(Thing::just_one(2));
    let b = Thing::just_one(Thing::just_one(6));

    let mut packets = packets.iter().flatten().collect::<Vec<_>>();

    packets.extend([&a, &b]);
    packets.sort_unstable();

    let ai = packets.iter().position(|x| **x == a).unwrap() + 1;
    let bi = packets.iter().position(|x| **x == b).unwrap() + 1;

    ai * bi
}

util::register!(parse, part1, part2);
