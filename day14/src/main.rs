#![cfg_attr(test, feature(test))]

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Pos {
    x: u32,
    y: u32,
}

type In = Vec<Pos>;
type Out = usize;

fn parse(s: &'static str) -> In {
    s.split(" -> ")
        .filter_map(|ss| ss.split_once(","))
        .map(|(a, b)| Pos {
            x: util::poarse(a),
            y: util::poarse(b),
        })
        .collect()
}

enum Tile {
    Wall,
    Sand,
}

fn build_world(paths: &[Vec<Pos>]) -> HashMap<Pos, Tile> {
    let mut world = HashMap::new();

    for path in paths {
        for win in path.windows(2) {
            let start = win[0];
            let end = win[1];

            let x0 = start.x.min(end.x);
            let x1 = start.x.max(end.x);
            let y0 = start.y.min(end.y);
            let y1 = start.y.max(end.y);
            if x0 == x1 {
                let x = x0;
                for y in y0..=y1 {
                    world.insert(Pos { x, y }, Tile::Wall);
                }
            } else {
                assert_eq!(y0, y1);
                let y = y0;
                for x in x0..=x1 {
                    world.insert(Pos { x, y }, Tile::Wall);
                }
            }
        }
    }

    world
}

fn fall(world: &HashMap<Pos, Tile>, bottom: u32) -> Pos {
    let mut x = 500;
    let mut y = 0;
    loop {
        if !world.contains_key(&Pos { x, y: y + 1 }) {
            y += 1;
        } else if !world.contains_key(&Pos { x: x - 1, y: y + 1 }) {
            x -= 1;
            y += 1;
        } else if !world.contains_key(&Pos { x: x + 1, y: y + 1 }) {
            x += 1;
            y += 1;
        } else {
            break;
        }

        if y >= bottom {
            break;
        }
    }

    Pos { x, y }
}

fn part1(n: &[In]) -> Out {
    let mut world = build_world(n);
    let bottom = world.keys().map(|p| p.y).max().unwrap();

    let mut ngrains = 0;

    loop {
        let pos = fall(&world, bottom);

        if pos.y >= bottom {
            break;
        } else {
            ngrains += 1;
            world.insert(pos, Tile::Sand);
        }
    }

    ngrains
}

fn part2(n: &[In]) -> Out {
    let mut world = build_world(n);
    let bottom = world.keys().map(|p| p.y).max().unwrap() + 1;

    let mut ngrains = 0;

    loop {
        let pos = fall(&world, bottom);

        ngrains += 1;
        world.insert(pos, Tile::Sand);

        if pos == (Pos { x: 500, y: 0 }) {
            break;
        }
    }

    ngrains
}

util::register!(parse, part1, part2);
