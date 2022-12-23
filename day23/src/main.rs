#![cfg_attr(test, feature(test))]

use std::collections::{HashMap, HashSet};

use util::*;

type N = i32;
type P = Pos<N>;

type In = Vec<bool>;
type Out = N;

fn parse(s: &'static str) -> In {
    s.bytes().map(|b| b == b'#').collect()
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn positions(self) -> [(N, N); 3] {
        match self {
            Self::North => [(-1, -1), (0, -1), (1, -1)],
            Self::South => [(-1, 1), (0, 1), (1, 1)],
            Self::West => [(-1, -1), (-1, 0), (-1, 1)],
            Self::East => [(1, -1), (1, 0), (1, 1)],
        }
    }

    pub fn primary_position(self) -> (N, N) {
        self.positions()[1]
    }
}

fn all_neighbors() -> Vec<(N, N)> {
    let mut v = vec![];
    for x in -1..=1 {
        for y in -1..=1 {
            if (x, y) != (0, 0) {
                v.push((x, y));
            }
        }
    }
    v
}

fn gather_elves(grid: &[Vec<bool>]) -> HashSet<P> {
    let h = grid.len();
    let w = grid[0].len();
    let mut elves = HashSet::new();
    for y in 0..h {
        for x in 0..w {
            if grid[y][x] {
                elves.insert(P::new(x as _, y as _));
            }
        }
    }
    elves
}

fn gather_proposals(elves: &HashSet<P>, directions: [Direction; 4]) -> HashMap<P, Vec<P>> {
    let mut proposals = HashMap::<P, Vec<P>>::new();

    for &elf in elves {
        let mut proposal = elf;

        if all_neighbors()
            .into_iter()
            .any(|n| elves.contains(&(elf + n)))
        {
            'dir: for d in directions {
                for n in d.positions() {
                    if elves.contains(&(elf + n)) {
                        continue 'dir;
                    }
                }
                proposal = elf + d.primary_position();
                break;
            }
        }

        proposals.entry(proposal).or_default().push(elf);
    }

    proposals
}

fn perform_movement(proposals: HashMap<P, Vec<P>>, elves: &mut HashSet<P>) -> bool {
    elves.clear();

    let mut moved = false;

    for (dst, srcs) in proposals {
        if srcs.len() == 1 {
            if srcs[0] != dst {
                moved = true;
            }
            elves.insert(dst);
        } else {
            for src in srcs {
                elves.insert(src);
            }
        }
    }

    moved
}

fn part1(n: &[In]) -> Out {
    let mut elves = gather_elves(n);

    let mut directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for _ in 0..10 {
        let proposals = gather_proposals(&elves, directions);
        perform_movement(proposals, &mut elves);
        directions.rotate_left(1);
    }

    let x0 = elves.iter().map(|e| e.x).min().unwrap();
    let y0 = elves.iter().map(|e| e.y).min().unwrap();
    let x1 = elves.iter().map(|e| e.x).max().unwrap();
    let y1 = elves.iter().map(|e| e.y).max().unwrap();

    (x1 - x0 + 1) * (y1 - y0 + 1) - elves.len() as N
}

fn part2(n: &[In]) -> Out {
    let mut elves = gather_elves(n);

    let mut directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    let mut i = 0;
    loop {
        i += 1;

        let proposals = gather_proposals(&elves, directions);
        let moved = perform_movement(proposals, &mut elves);
        directions.rotate_left(1);

        if !moved {
            break i;
        }
    }
}

util::register!(parse, part1, part2);
