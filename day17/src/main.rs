#![cfg_attr(test, feature(test))]

use std::collections::HashSet;

use util::*;

type N = i32;
type P = Pos<N>;

type In = Vec<bool>;
type Out = N;

#[derive(Clone)]
pub struct Rock {
    points: Vec<P>,
}

impl Rock {
    pub fn new<const LEN: usize>(points: [(N, N); LEN]) -> Rock {
        let points = points.map(|(x, y)| P::new(x, y)).into();
        Rock { points }
    }

    pub fn shift(&mut self, dx: N, dy: N) {
        for p in &mut self.points {
            *p += (dx, dy);
        }
    }

    pub fn shifted(&self, dx: N, dy: N) -> Self {
        let mut this = self.clone();
        this.shift(dx, dy);
        this
    }

    pub fn bottom(&self) -> N {
        self.points.iter().map(|p| p.y).min().unwrap()
    }

    pub fn top(&self) -> N {
        self.points.iter().map(|p| p.y).max().unwrap()
    }

    pub fn left(&self) -> N {
        self.points.iter().map(|p| p.x).min().unwrap()
    }

    pub fn right(&self) -> N {
        self.points.iter().map(|p| p.x).max().unwrap()
    }

    pub fn collides_with(&self, world: &World) -> bool {
        self.points.iter().any(|p| world.contains(p))
    }

    pub fn add_to(self, world: &mut World) {
        for p in self.points {
            world.insert(p);
        }
    }
}

fn rock_cycle() -> impl Iterator<Item = Rock> {
    std::iter::repeat([
        Rock::new([(0, 0), (1, 0), (2, 0), (3, 0)]),
        Rock::new([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
        Rock::new([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        Rock::new([(0, 0), (0, 1), (0, 2), (0, 3)]),
        Rock::new([(0, 0), (0, 1), (1, 0), (1, 1)]),
    ])
    .flatten()
}

type World = HashSet<P>;

fn parse(s: &'static str) -> In {
    s.trim().chars().map(|c| c == '>').collect()
}

pub fn print_world(world: &World, rock: &Rock, top: N) {
    for y in (0..top).rev() {
        print!("|");
        for x in 0..7 {
            let pt = P { x, y };
            let c = if world.contains(&pt) {
                '#'
            } else if rock.points.iter().any(|p| *p == pt) {
                '@'
            } else {
                '.'
            };
            print!("{c}")
        }
        print!("|");
        println!();
    }
    println!("+-------+");
    println!();
}

fn fall(mut gas: impl Iterator<Item = bool>, rock: &mut Rock, world: &World) {
    loop {
        assert!(rock.right() <= 6);
        assert!(rock.left() >= 0);

        let right = gas.next().unwrap();
        if right {
            if rock.right() < 6 {
                let shifted = rock.shifted(1, 0);
                if !shifted.collides_with(&world) {
                    *rock = shifted;
                }
            }
        } else {
            if rock.left() > 0 {
                let shifted = rock.shifted(-1, 0);
                if !shifted.collides_with(&world) {
                    *rock = shifted;
                }
            }
        }

        if rock.bottom() <= 0 {
            break;
        }

        let shifted = rock.shifted(0, -1);
        if shifted.collides_with(&world) {
            break;
        } else {
            *rock = shifted;
        }
    }
}

fn part1(n: &In) -> Out {
    let mut world = World::new();

    let mut gas = std::iter::repeat(n).flatten().copied();

    let mut spawn_location = 3;

    for mut rock in rock_cycle().take(2022) {
        rock.shift(2, spawn_location);
        fall(gas.by_ref(), &mut rock, &world);
        spawn_location = spawn_location.max(rock.top() + 4);
        rock.add_to(&mut world);
    }

    spawn_location - 3
}

fn part2(n: &In) -> Out {
    Default::default()
}

util::register!(parse, part1, part2, @alt);
