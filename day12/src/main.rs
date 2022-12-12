#![cfg_attr(test, feature(test))]

use std::collections::{HashMap, HashSet};

type In = Input;
type Out = u64;

type Pos = (usize, usize);

#[derive(Clone)]
struct Input(HashMap<Pos, u8>, Pos, Pos);

fn parse(s: &'static str) -> In {
    let mut start = Pos::default();
    let mut goal = Pos::default();
    let mut grid = HashMap::new();

    for (y, row) in s.lines().map(str::trim).enumerate() {
        for (x, c) in row.bytes().enumerate() {
            let v = match c {
                b'S' => {
                    start = (x, y);
                    b'a'
                }
                b'E' => {
                    goal = (x, y);
                    b'z'
                }
                _ => c,
            };
            grid.insert((x, y), v);
        }
    }

    Input(grid, start, goal)
}

fn neighbors(pos: Pos, dims: Pos, unvisited: &HashSet<Pos>) -> impl Iterator<Item = Pos> + '_ {
    let (w, h) = dims;
    let (cx, cy) = (pos.0 as i32, pos.1 as i32);
    [(cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)]
        .into_iter()
        .filter(move |(x, y)| (0..w as i32).contains(x) && (0..h as i32).contains(y))
        .map(|(x, y)| (x as usize, y as usize))
        .filter(|p| unvisited.contains(p))
}

fn setup(
    n: &In,
) -> (
    HashMap<Pos, u8>,
    Pos,
    Pos,
    Pos,
    HashSet<Pos>,
    HashMap<Pos, u64>,
) {
    let Input(heights, start, goal) = n.clone();
    let w = heights.keys().map(|p| p.0).max().unwrap() + 1;
    let h = heights.keys().map(|p| p.1).max().unwrap() + 1;
    let mut unvisited = HashSet::new();
    let mut distances = HashMap::new();

    for y in 0..h {
        for x in 0..w {
            unvisited.insert((x, y));
            distances.insert((x, y), u64::MAX);
        }
    }

    (heights, start, goal, (w, h), unvisited, distances)
}

fn part1(n: &In) -> Out {
    let (heights, start, goal, dims, mut unvisited, mut distances) = setup(n);
    distances.insert(start, 0);

    loop {
        let current = unvisited.iter().min_by_key(|pos| distances[pos]).unwrap();
        assert_ne!(distances[current], u64::MAX);

        for ref neighbor in neighbors(*current, dims, &unvisited) {
            if heights[neighbor] <= heights[current] + 1 {
                let td = distances[current] + 1;
                let dn = distances.get_mut(neighbor).unwrap();
                *dn = u64::min(*dn, td);
            }
        }

        if *current == goal {
            return distances[&current];
        }

        unvisited.remove(&current.clone());
    }
}

fn part2(n: &In) -> Out {
    let (heights, _, goal, dims, mut unvisited, mut distances) = setup(n);
    distances.insert(goal, 0);

    loop {
        let current = unvisited.iter().min_by_key(|pos| distances[pos]).unwrap();
        assert_ne!(distances[current], u64::MAX);

        for ref neighbor in neighbors(*current, dims, &unvisited) {
            if heights[neighbor] >= heights[current] - 1 {
                let td = distances[current] + 1;
                let dn = distances.get_mut(neighbor).unwrap();
                *dn = u64::min(*dn, td);
            }
        }

        if heights[current] == b'a' {
            return distances[current];
        }

        unvisited.remove(&current.clone());
    }
}

util::register!(parse, part1, part2, @alt);
