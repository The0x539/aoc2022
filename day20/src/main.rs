#![cfg_attr(test, feature(test))]

use util::*;

type N = i64;

type In = N;
type Out = N;

fn parse(s: &'static str) -> In {
    p(s)
}

#[derive(Copy, Clone, Debug)]
struct Num {
    value: N,
    index: usize,
    processed: bool,
}

impl Num {
    pub fn new((index, value): (usize, N)) -> Self {
        Self {
            value,
            index,
            processed: false,
        }
    }
}

fn mix(real_n: &mut [N]) {
    let mut n = real_n
        .iter()
        .copied()
        .enumerate()
        .map(Num::new)
        .collect::<Vec<_>>();

    let nl = n.len() as N;

    let mut i = 0;
    while i < n.len() {
        let v = &mut n[i];

        if v.processed {
            i += 1;
            continue;
        }

        v.processed = true;

        let mut j = i as N + v.value;
        while j <= 0 {
            j += nl - 1;
        }
        while j >= nl {
            j -= nl - 1;
        }

        let j = j as usize;
        if j == i {
            i += 1;
            continue;
        }

        if j > i {
            n[i..=j].rotate_left(1);
        } else {
            // move v to behind us
            n[j..=i].rotate_right(1);
            i += 1;
        }
    }

    for i in 0..n.len() {
        real_n[i] = n[i].value;
    }
}

fn mix_part2(n: &mut [Num]) {
    let nl = n.len() as N;

    for ii in 0..n.len() {
        let i = n.iter().position(|v| v.index == ii).unwrap();
        let v = &mut n[i];

        if v.processed {
            continue;
        }

        v.processed = true;

        let mut j = i as N + v.value;
        while j <= 0 {
            j += nl - 1;
        }
        while j >= nl {
            j -= nl - 1;
        }

        let j = j as usize;
        if j == i {
            continue;
        }

        if j > i {
            n[i..=j].rotate_left(1);
        } else {
            // move v to behind us
            n[j..=i].rotate_right(1);
        }

        println!("{ii}");
    }

    for x in n {
        x.processed = false;
    }
}

fn part1(n: &[In]) -> Out {
    let mut n = n.to_vec();
    mix(&mut n);

    let i0 = n.iter().position(|x| *x == 0).unwrap();
    [1000, 2000, 3000]
        .map(|i| n[(i0 + i) % n.len()])
        .iter()
        .sum()
}

fn part2(n: &[In]) -> Out {
    let mut n = n.to_vec();

    const KEY: N = 811589153;

    for v in &mut n {
        *v *= KEY;
    }

    let mut n = n
        .iter()
        .copied()
        .enumerate()
        .map(Num::new)
        .collect::<Vec<_>>();

    for _ in 0..10 {
        println!("{n:?}");
        mix_part2(&mut n);
    }

    let i0 = n.iter().position(|x| x.value == 0).unwrap();
    [1000, 2000, 3000]
        .map(|i| dbg!(n[(i0 + i) % n.len()].value))
        .iter()
        .sum()
}

util::register!(parse, part1, part2);
