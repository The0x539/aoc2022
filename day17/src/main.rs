#![cfg_attr(test, feature(test))]

use std::collections::{HashMap, HashSet};

use util::*;

type N = i64;
type P = Pos<N>;

type In = Vec<bool>;
type Out = N;

#[derive(Clone, Hash, PartialEq, Eq)]
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

fn rocks() -> [Rock; 5] {
    [
        Rock::new([(0, 0), (1, 0), (2, 0), (3, 0)]),
        Rock::new([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
        Rock::new([(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        Rock::new([(0, 0), (0, 1), (0, 2), (0, 3)]),
        Rock::new([(0, 0), (0, 1), (1, 0), (1, 1)]),
    ]
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

fn take_snapshot(world: &World, bottom: N, top: N) -> Vec<[bool; 7]> {
    let mut rows = Vec::with_capacity((top - bottom) as usize);
    for y in bottom..=top {
        let row = std::array::from_fn(|i| {
            let x = i as N;
            world.contains(&Pos { x, y })
        });
        rows.push(row);
    }
    rows
}

fn part1(n: &In) -> Out {
    let mut world = World::new();

    let mut gas = std::iter::repeat(n).flatten().copied();

    let mut spawn_location = 3;

    for mut rock in std::iter::repeat(rocks()).flatten().take(2022) {
        rock.shift(2, spawn_location);
        fall(&mut gas, &mut rock, &world);
        spawn_location = spawn_location.max(rock.top() + 4);
        rock.add_to(&mut world);
    }

    spawn_location - 3
}

fn part2(gases: &Vec<bool>) -> Out {
    let mut world = World::new();

    let rocks = rocks();

    let mut spawn_location = 3;

    let mut gas_index = 0;

    let mut recurrences = HashMap::<_, N>::new();
    let mut snapshots = HashMap::<_, Vec<[bool; 7]>>::new();
    let mut snapshot_times = HashMap::new();

    let mut result_offset = 0;

    let mut rock_number: N = -1;
    while rock_number < 1000000000000 {
        rock_number += 1;

        let rock_index = rock_number as usize % rocks.len();
        let mut rock = rocks[rock_index].clone();
        rock.shift(2, spawn_location);

        let key = (rock_index, gas_index);
        let n = recurrences.entry(key).or_default();
        *n += 1;
        if *n >= 2 {
            let snapshot = take_snapshot(&world, spawn_location - 500, spawn_location);

            let snapshot_value = (rock_number, spawn_location);
            if snapshots.get(&key) == Some(&snapshot) {
                let (new_rock_num, new_spawn_loc) = snapshot_value;
                let (old_rock_num, old_spawn_loc) = snapshot_times[&key];
                let rock_delta = new_rock_num - old_rock_num;
                let spawn_delta = new_spawn_loc - old_spawn_loc;

                while rock_number + rock_delta < 1000000000000 {
                    rock_number += rock_delta;
                    result_offset += spawn_delta;
                }
            } else {
                snapshots.insert(key, snapshot);
                snapshot_times.insert(key, snapshot_value);
            }
        }

        let gas_iter = std::iter::from_fn(|| {
            let v = gases[gas_index];
            gas_index = (gas_index + 1) % gases.len();
            Some(v)
        });
        fall(gas_iter, &mut rock, &world);

        spawn_location = spawn_location.max(rock.top() + 4);
        rock.add_to(&mut world);
    }

    spawn_location + result_offset - 4
}

util::register!(parse, part1, part2, @alt);
