#![cfg_attr(test, feature(test))]

use std::collections::HashSet;

use util::*;

type N = i32;
type P = Pos<N>;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Wall,
    Ground,
    Blizzard(Dir),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn offset(self) -> (N, N) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }
}

type In = Vec<Tile>;
type Out = usize;

fn parse(s: &'static str) -> In {
    s.chars()
        .map(|c| match c {
            '#' => Tile::Wall,
            '.' => Tile::Ground,
            '^' => Tile::Blizzard(Dir::North),
            '>' => Tile::Blizzard(Dir::East),
            'v' => Tile::Blizzard(Dir::South),
            '<' => Tile::Blizzard(Dir::West),
            _ => panic!(),
        })
        .collect()
}

struct Blizzard {
    pos: P,
    dir: Dir,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Leg {
    First,  // start -> end
    Second, // end -> start
    Third,  // start -> end
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Person {
    pos: P,
    journey_leg: Leg,
}

fn part1(n: &[In]) -> Out {
    solution(n, false)
}

fn part2(n: &[In]) -> Out {
    solution(n, true)
}

fn solution(n: &[In], part2: bool) -> Out {
    let h = n.len() as N - 1;
    let w = n[0].len() as N - 1;

    let mut blizzards = Vec::new();
    for (y, row) in n.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if let Tile::Blizzard(dir) = *tile {
                let pos = P::new(x as N, y as N);
                blizzards.push(Blizzard { pos, dir });
            }
        }
    }

    let mut selves = vec![Person {
        pos: P::new(1, 0),
        journey_leg: Leg::First,
    }];

    let mut occupied = HashSet::new();

    let mut i = 0;
    loop {
        occupied.clear();
        for b in &mut blizzards {
            b.pos += b.dir.offset();
            if b.pos.x == 0 {
                b.pos.x = w - 1;
            } else if b.pos.x == w {
                b.pos.x = 1;
            }
            if b.pos.y == 0 {
                b.pos.y = h - 1;
            } else if b.pos.y == h {
                b.pos.y = 1;
            }
            occupied.insert(b.pos);
        }

        let mut futures = HashSet::new();
        for pr in selves {
            futures.insert(pr);
            for d in [Dir::North, Dir::South, Dir::East, Dir::West] {
                let mut prd = pr;
                prd.pos += d.offset();
                futures.insert(prd);
            }
        }

        futures.retain(|p| p.pos.x > 0 && p.pos.x < w);
        futures.retain(|p| p.pos.y > 0 || p.pos.x == 1);
        futures.retain(|p| p.pos.y < h || p.pos.x == w - 1);
        futures.retain(|p| !occupied.contains(&p.pos));

        i += 1;

        selves = Vec::with_capacity(futures.len());
        for mut f in futures {
            if part2 {
                if f.pos == P::new(w - 1, h) {
                    match f.journey_leg {
                        Leg::First => f.journey_leg = Leg::Second,
                        Leg::Second => (),
                        Leg::Third => return i,
                    }
                } else if f.pos == P::new(1, 0) {
                    match f.journey_leg {
                        Leg::First => (),
                        Leg::Second => f.journey_leg = Leg::Third,
                        Leg::Third => (),
                    }
                }
            } else if f.pos == P::new(w - 1, h) {
                return i;
            }

            selves.push(f);
        }
    }
}

util::register!(parse, part1, part2);
