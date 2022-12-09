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

fn take_step(head: &mut Pos, dir: Dir) {
    match dir {
        Dir::Up => head.y -= 1,
        Dir::Left => head.x -= 1,
        Dir::Down => head.y += 1,
        Dir::Right => head.x += 1,
    }
}

fn catch_up(head: &Pos, tail: &mut Pos) {
    let (dy, dx) = (head.y - tail.y, head.x - tail.x);

    if dx.abs() > 1 {
        tail.x += dx.signum();
        if dy != 0 {
            tail.y += dy.signum();
        }
    } else if dy.abs() > 1 {
        tail.y += dy.signum();
        if dx != 0 {
            tail.x += dx.signum();
        }
    }
}

fn part1(n: &[In]) -> Out {
    let mut head = Pos::default();
    let mut tail = Pos::default();

    let mut visited = HashSet::new();
    visited.insert(tail);

    for &step in n {
        for _ in 0..step.count {
            take_step(&mut head, step.dir);
            catch_up(&head, &mut tail);
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
            take_step(&mut links[9], step.dir);
            for i in (1..links.len()).rev() {
                let [.., tail, head] = &mut links[..=i] else { panic!() };
                catch_up(head, tail);
            }
            visited.insert(links[0]);
        }
    }

    visited.len()
}

util::register!(parse, part1, part2);
