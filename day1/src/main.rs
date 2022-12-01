#![cfg_attr(test, feature(test))]

type In = Option<i32>;
type Out = i32;

fn parse(s: &'static str) -> In {
    if s.is_empty() {
        None
    } else {
        Some(util::poarse(s))
    }
}

fn part1(n: &[In]) -> Out {
    let mut counts = vec![0];
    for x in n {
        if let Some(cal) = x {
            *counts.last_mut().unwrap() += cal;
        } else {
            counts.push(0);
        }
    }
    *counts.iter().max().unwrap()
}

fn part2(n: &[In]) -> Out {
    let mut counts = vec![0];
    for x in n {
        if let Some(cal) = x {
            *counts.last_mut().unwrap() += cal;
        } else {
            counts.push(0);
        }
    }
    counts.sort();
    counts.reverse();
    counts[..3].iter().sum()
}

util::register!(parse, part1, part2);
