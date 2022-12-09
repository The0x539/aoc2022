#![cfg_attr(test, feature(test))]

use std::collections::HashSet;

type In = Step;
type Out = usize;

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Step {
    dir: Dir,
    count: u32,
}

fn parse(s: &'static str) -> In {
    let dir = match s.as_bytes()[0] {
        b'U' => Dir::Up,
        b'D' => Dir::Down,
        b'L' => Dir::Left,
        b'R' => Dir::Right,
        _ => panic!(),
    };
    let count = s[2..].parse().unwrap();
    Step { dir, count }
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

fn part1(n: &[In]) -> Out {
    let mut head = Pos::default();
    let mut tail = Pos::default();

    let mut visited = HashSet::new();
    visited.insert(tail);

    for &step in n {
        for _ in 0..step.count {
            match step.dir {
                Dir::Up => head.y -= 1,
                Dir::Left => head.x -= 1,
                Dir::Down => head.y += 1,
                Dir::Right => head.x += 1,
            }

            let (dy, dx) = (head.y - tail.y, head.x - tail.x);
            if dx.abs() > 1 {
                tail.x += dx.signum();
                if dy.abs() > 0 {
                    tail.y += dy.signum();
                }
            } else if dy.abs() > 1 {
                tail.y += dy.signum();
                if dx.abs() > 0 {
                    tail.x += dx.signum();
                }
            }

            visited.insert(tail);
        }
    }

    visited.len()
}

fn part2(n: &[In]) -> Out {
    let mut links = vec![Pos::default(); 10];

    let mut visited = HashSet::new();

    visited.insert(links[0]);

    for &step in n {
        for _ in 0..step.count {
            let head = &mut links[0];

            match step.dir {
                Dir::Up => head.y -= 1,
                Dir::Left => head.x -= 1,
                Dir::Down => head.y += 1,
                Dir::Right => head.x += 1,
            }

            for i in 1..10 {
                let (dy, dx) = (links[i - 1].y - links[i].y, links[i - 1].x - links[i].x);
                if dx.abs() > 1 {
                    links[i].x += dx.signum();
                    if dy.abs() > 0 {
                        links[i].y += dy.signum();
                    }
                } else if dy.abs() > 1 {
                    links[i].y += dy.signum();
                    if dx.abs() > 0 {
                        links[i].x += dx.signum();
                    }
                }
            }

            /*
            for y in -8..7 {
                for x in -13..13 {
                    let pos = Pos { x, y };

                    let c: String;
                    if links[0] == pos {
                        c = "H".into();
                    } else if let Some(i) = links.iter().position(|p| *p == pos) {
                        c = i.to_string();
                    } else if visited.contains(&Pos { x, y }) {
                        c = "#".to_owned();
                    } else {
                        c = ".".to_owned();
                    }
                    print!("{c}");
                }
                println!();
            }
            println!();
            */

            visited.insert(links[9]);
        }
    }

    visited.len()
}

util::register!(parse, part1, part2);
