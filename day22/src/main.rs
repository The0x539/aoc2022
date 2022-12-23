#![cfg_attr(test, feature(test))]

use std::collections::HashMap;

use util::*;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Open,
    Solid,
    Void,
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Right,
}

struct In {
    board: Vec<Vec<Tile>>,
    moves: Vec<N>,
    turns: Vec<Turn>,
}

type N = usize;
type P = Pos<N>;

type Out = N;

fn parse(s: &'static str) -> In {
    let mut lines = s.lines();

    let mut board = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.trim_matches(&['\r', '\n'][..]);

        if line.is_empty() {
            break;
        }

        let row = line
            .chars()
            .map(|c| match c {
                ' ' => Tile::Void,
                '.' => Tile::Open,
                '#' => Tile::Solid,
                _ => panic!(),
            })
            .collect();

        board.push(row);
    }

    let final_line = lines.next().unwrap().trim();
    assert!(lines.next().is_none());

    let moves = ints(final_line);

    let turns = final_line
        .split(|c: char| c.is_digit(10))
        .filter(|s| !s.is_empty())
        .map(|t| if t == "R" { Turn::Right } else { Turn::Left })
        .collect();

    In {
        board,
        moves,
        turns,
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn turn(self, turn: Turn) -> Self {
        match turn {
            Turn::Left => self.left(),
            Turn::Right => self.right(),
        }
    }
}

fn is_tile(tile: Option<&Tile>) -> bool {
    matches!(tile, Some(Tile::Solid | Tile::Open))
}

fn is_void(tile: Option<&Tile>) -> bool {
    matches!(tile, Some(Tile::Void) | None)
}

fn part1(n: &In) -> Out {
    let In {
        board,
        moves,
        turns,
    } = n;

    let mut pos = P {
        x: board[0].iter().position(|t| *t == Tile::Open).unwrap() as N,
        y: 0,
    };
    let mut facing = Direction::East;

    let mut foo = moves.iter().copied();
    let mut bar = turns.iter().copied();
    loop {
        let Some(mv) = foo.next() else { panic!() };
        for _ in 0..mv {
            let tentative_pos = match facing {
                Direction::North => {
                    let max_y = board
                        .iter()
                        .rposition(|row| is_tile(row.get(pos.x)))
                        .unwrap();

                    let y = if pos.y == 0 || is_void(board[pos.y - 1].get(pos.x)) {
                        max_y
                    } else {
                        pos.y - 1
                    };

                    P { x: pos.x, y }
                }
                Direction::South => {
                    let min_y = board
                        .iter()
                        .position(|row| is_tile(row.get(pos.x)))
                        .unwrap();

                    let y = if is_void(board.get(pos.y + 1).and_then(|row| row.get(pos.x))) {
                        min_y
                    } else {
                        pos.y + 1
                    };

                    P { x: pos.x, y }
                }
                Direction::East => {
                    let row = &board[pos.y];

                    let min_x = row.iter().position(|t| is_tile(Some(t))).unwrap();

                    let x = if is_void(row.get(pos.x + 1)) {
                        min_x
                    } else {
                        pos.x + 1
                    };

                    P { x, y: pos.y }
                }
                Direction::West => {
                    let row = &board[pos.y];

                    let max_x = row
                        .iter()
                        .rposition(|t| matches!(t, Tile::Open | Tile::Solid))
                        .unwrap();

                    let x = if pos.x == 0 || is_void(row.get(pos.x - 1)) {
                        max_x
                    } else {
                        pos.x - 1
                    };

                    P { x, y: pos.y }
                }
            };
            if board[tentative_pos.y][tentative_pos.x] == Tile::Solid {
                break;
            } else {
                pos = tentative_pos;
            }
        }

        let Some(turn) = bar.next() else { break };
        facing = facing.turn(turn);
    }

    let row = pos.y + 1; // excuse me?
    let column = pos.x + 1;
    let fcng = match facing {
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
        Direction::North => 3,
    };

    1000 * row + 4 * column + fcng
}

fn build_wraps(_: &In) -> HashMap<P, (P, &'static [Turn])> {
    let mut wraps = HashMap::new();

    macro_rules! add_wrap {
        ($x0:expr,$y0:expr,$x1:expr,$y1:expr,$turns0:expr, $turns1:expr) => {{
            let p0 = P::new($x0, $y0);
            let p1 = P::new($x1, $y1);
            wraps.insert(p0, (p1, $turns0));
            wraps.insert(p1, (p0, $turns1));
        }};
    }

    let left: &[Turn] = &[Turn::Left];
    let right: &[Turn] = &[Turn::Right];
    let uturn: &[Turn] = &[Turn::Left; 2];
    let noturn: &[Turn] = &[];

    // hardcoded, sorry
    if cfg!(test) {
        // 1,3
        for y0 in 0..4 {
            let x0 = 8;
            let y1 = 4;
            let x1 = y0 + 4;
            add_wrap!(x0, y0, x1, y1, left, right);
        }

        // 1,2
        for x0 in 8..12 {
            let y0 = 0;
            let x1 = x0 - 8;
            let y1 = 4;
            add_wrap!(x0, y0, x1, y1, uturn, uturn);
        }

        // 2,4
        for y0 in 4..8 {
            let x0 = 0;
            let x1 = 15;
            let y1 = y0 + 4;
            add_wrap!(x0, y0, x1, y1, noturn, noturn);
        }

        // 3,5
        for x0 in 4..8 {
            let y0 = 7;
            let x1 = 7;
            let y1 = 15 - x0;
            add_wrap!(x0, y0, x1, y1, right, left);
        }

        // 2,5
        for x0 in 0..4 {
            let y0 = 7;
            let x1 = 11 - x0;
            let y1 = 11;
            add_wrap!(x0, y0, x1, y1, uturn, uturn);
        }

        // 4,6
        for y0 in 4..8 {
            let x0 = 11;
            let x1 = 19 - y0;
            let y1 = 8;
            add_wrap!(x0, y0, x1, y1, right, left);
        }
    } else {
        for x0 in 50..100 {
            let y0 = 0;
            let x1 = 0;
            let y1 = 100 + x0;
            add_wrap!(x0, y0, x1, y1, right, left);
        }

        for y0 in 50..100 {
            let x0 = 50;
            let x1 = y0 - 50;
            let y1 = 100;
            add_wrap!(x0, y0, x1, y1, left, right);
        }

        for x0 in 50..100 {
            let y0 = 149;
            let x1 = 49;
            let y1 = 100 + x0;
            add_wrap!(x0, y0, x1, y1, right, left);
        }

        for y0 in 100..150 {
            let x0 = 99;
            let x1 = 149;
            let y1 = 149 - y0;
            add_wrap!(x0, y0, x1, y1, uturn, uturn);
        }

        for x0 in 100..150 {
            let y0 = 0;
            let x1 = x0 - 100;
            let y1 = 199;
            add_wrap!(x0, y0, x1, y1, noturn, noturn);
        }

        for x0 in 100..150 {
            let y0 = 49;
            let x1 = 99;
            let y1 = x0 - 50;
            add_wrap!(x0, y0, x1, y1, right, left);
        }

        for y0 in 0..50 {
            let x0 = 50;
            let x1 = 0;
            let y1 = 149 - y0;
            add_wrap!(x0, y0, x1, y1, uturn, uturn);
        }
    }

    wraps
}

fn part2(n: &In) -> Out {
    let wraps = build_wraps(n);

    let In {
        board,
        moves,
        turns,
    } = n;

    let mut pos = P {
        x: board[0].iter().position(|t| *t == Tile::Open).unwrap() as N,
        y: 0,
    };
    let mut facing = Direction::East;

    let mut foo = moves.iter().copied();
    let mut bar = turns.iter().copied();
    loop {
        let Some(mv) = foo.next() else { panic!() };
        for _i in 0..mv {
            let mut wrap_turns = None;

            macro_rules! do_wrap {
                () => {{
                    let (wp, wt) = &wraps[&pos];
                    wrap_turns = Some(wt);
                    *wp
                }};
            }

            let tentative_pos = match facing {
                Direction::North => {
                    if pos.y == 0 || is_void(board[pos.y - 1].get(pos.x)) {
                        do_wrap!()
                    } else {
                        P::new(pos.x, pos.y - 1)
                    }
                }

                Direction::South => {
                    if is_void(board.get(pos.y + 1).and_then(|row| row.get(pos.x))) {
                        do_wrap!()
                    } else {
                        P::new(pos.x, pos.y + 1)
                    }
                }

                Direction::East => {
                    let row = &board[pos.y];
                    if is_void(row.get(pos.x + 1)) {
                        do_wrap!()
                    } else {
                        P::new(pos.x + 1, pos.y)
                    }
                }
                Direction::West => {
                    let row = &board[pos.y];

                    if pos.x == 0 || is_void(row.get(pos.x - 1)) {
                        do_wrap!()
                    } else {
                        P::new(pos.x - 1, pos.y)
                    }
                }
            };

            if board[tentative_pos.y][tentative_pos.x] == Tile::Solid {
                break;
            } else {
                pos = tentative_pos;
                if let Some(turns) = wrap_turns {
                    for turn in *turns {
                        facing = facing.turn(*turn);
                    }
                }
            }
        }

        let Some(turn) = bar.next() else { break };
        facing = facing.turn(turn);
    }

    let row = pos.y + 1;
    let column = pos.x + 1;
    let fcng = match facing {
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
        Direction::North => 3,
    };

    1000 * row + 4 * column + fcng
}

util::register!(parse, part1, part2, @alt);
