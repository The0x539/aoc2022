#![cfg_attr(test, feature(test))]

use std::collections::{BTreeMap, HashSet};

use util::*;

type N = i64;
type P = Pos<N>;

type In = Pair;
type Out = i64;

#[derive(Copy, Clone)]
struct Pair {
    sensor: P,
    beacon: P,
}

impl Pair {
    fn radius(&self) -> N {
        self.sensor.x.abs_diff(self.beacon.x) as N + self.sensor.y.abs_diff(self.beacon.y) as N
    }

    fn in_range(&self, p: P) -> bool {
        let r = self.sensor.x.abs_diff(p.x) as N + self.sensor.y.abs_diff(p.y) as N;
        r <= self.radius()
    }
}

fn parse(s: &'static str) -> In {
    let toks = s.split_whitespace().collect::<Vec<_>>();

    let x = toks[2][2..].strip_suffix(',').unwrap().parse().unwrap();
    let y = toks[3][2..].strip_suffix(':').unwrap().parse().unwrap();
    let sensor = P { x, y };

    let x = toks[8][2..].strip_suffix(',').unwrap().parse().unwrap();
    let y = toks[9][2..].parse().unwrap();
    let beacon = P { x, y };

    Pair { sensor, beacon }
}

fn part1(n: &[In]) -> Out {
    let beacons = n.iter().map(|pair| pair.beacon).collect::<HashSet<_>>();

    let min_x = n
        .iter()
        .map(|pair| pair.sensor.x - pair.radius())
        .min()
        .unwrap();

    let max_x = n
        .iter()
        .map(|pair| pair.sensor.x + pair.radius())
        .max()
        .unwrap();

    let mut num_hashes = 0;

    let y = if n.len() == 14 { 10 } else { 2000000 };

    for x in min_x..=max_x {
        let sample = Pos { x, y };

        if beacons.contains(&sample) {
            continue;
        }

        for pair in n {
            if pair.in_range(sample) {
                num_hashes += 1;
                break;
            }
        }
    }

    num_hashes
}

fn part2(pairs: &[In]) -> Out {
    let mut edge_points = BTreeMap::<P, u32>::new();
    let mut true_candidates = Vec::<P>::new();

    let acceptable = if pairs.len() == 14 {
        0..=20
    } else {
        0..=4000000
    };

    let mut add_point = |x, y| {
        if !acceptable.contains(&x) || !acceptable.contains(&y) {
            return;
        }
        let pt = P::new(x, y);
        let ent = edge_points.entry(pt).or_default();
        *ent += 1;
        if *ent > 2 {
            true_candidates.push(pt);
        }
    };

    for pair in pairs {
        let r = pair.radius() + 1;

        let x0 = pair.sensor.x;
        let y0 = pair.sensor.y;

        let mut dy = -r;
        let mut dx = 0;

        // north -> east
        while dy != 0 {
            add_point(x0 + dx, y0 + dy);
            dy += 1;
            dx += 1;
        }

        // east -> south
        while dx != 0 {
            add_point(x0 + dx, y0 + dy);
            dy += 1;
            dx -= 1;
        }

        // south -> west
        while dy != 0 {
            add_point(x0 + dx, y0 + dy);
            dy -= 1;
            dx -= 1;
        }

        // west -> north
        while dx != 0 {
            add_point(x0 + dx, y0 + dy);
            dy -= 1;
            dx += 1;
        }
    }

    let beacon = true_candidates
        .into_iter()
        .find(|pos| pairs.iter().all(|pair| !pair.in_range(*pos)))
        .unwrap();

    beacon.x * 4000000 + beacon.y
}

util::register!(parse, part1, part2);
