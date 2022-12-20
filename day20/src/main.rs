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
}

impl Num {
    pub fn new((index, value): (usize, N)) -> Self {
        Self { value, index }
    }
}

fn prepare(n: &[N]) -> (Vec<Num>, Vec<usize>) {
    let nums = n.iter().copied().enumerate().map(Num::new).collect();
    let locations = (0..n.len()).collect();
    (nums, locations)
}

fn mix(nums: &mut [Num], locations: &mut [usize]) {
    let nl = nums.len() as N;

    for ii in 0..nums.len() {
        let i = locations[ii];
        let v = &mut nums[i];
        assert_eq!(ii, v.index);

        let mut j = i as N + v.value;
        j %= nl - 1;
        if j <= 0 {
            j += nl - 1;
        }
        let j = j as usize;

        if j > i {
            nums[i..=j].rotate_left(1);
            for k in i..j {
                let kk = nums[k].index;
                locations[kk] -= 1;
            }
            locations[ii] = j;
        } else if j < i {
            nums[j..=i].rotate_right(1);
            for k in (j + 1)..=i {
                let kk = nums[k].index;
                locations[kk] += 1;
            }
            locations[ii] = j;
        }
    }
}

fn part1(n: &[In]) -> Out {
    let (mut nums, mut locations) = prepare(n);
    mix(&mut nums, &mut locations);
    result(&nums)
}

fn part2(n: &[In]) -> Out {
    let (mut nums, mut locations) = prepare(n);

    const KEY: N = 811589153;
    for v in &mut nums {
        v.value *= KEY;
    }

    for _ in 0..10 {
        mix(&mut nums, &mut locations);
    }

    result(&nums)
}

fn result(n: &[Num]) -> Out {
    std::iter::repeat(n)
        .flatten()
        .map(|v| v.value)
        .skip_while(|v| *v != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

util::register!(parse, part1, part2);
