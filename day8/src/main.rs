#![cfg_attr(test, feature(test))]

type In = Vec<i8>;
type Out = usize;

fn parse(s: &'static str) -> In {
    s.bytes().map(|b| b - b'0').map(|b| b as i8).collect()
}

fn part1(n: &[In]) -> Out {
    let height = n.len();
    let width = n[0].len();

    let mut grid = vec![vec![false; width]; height];

    for x in 0..width {
        let mut max = -1;
        for y in 0..height {
            if n[y][x] > max {
                max = n[y][x];
                grid[y][x] = true;
            }
        }

        max = -1;
        for y in (0..height).rev() {
            if n[y][x] > max {
                max = n[y][x];
                grid[y][x] = true;
            }
        }
    }

    for y in 0..height {
        let mut max = -1;
        for x in 0..width {
            if n[y][x] > max {
                max = n[y][x];
                grid[y][x] = true;
            }
        }

        max = -1;
        for x in (0..width).rev() {
            if n[y][x] > max {
                max = n[y][x];
                grid[y][x] = true;
            }
        }
    }

    grid.iter().flatten().filter(|x| **x).count()
}

fn part2(n: &[In]) -> Out {
    let height = n.len();
    let width = n[0].len();

    let mut grid = vec![vec![0; width]; height];

    for y in 0..height {
        for x in 0..width {
            let tree = n[y][x];

            let [mut north, mut east, mut south, mut west] = [0; 4];

            for yy in (0..y).rev() {
                north += 1;
                if n[yy][x] >= tree {
                    break;
                }
            }

            for yy in y + 1..height {
                south += 1;
                if n[yy][x] >= tree {
                    break;
                }
            }

            for xx in (0..x).rev() {
                west += 1;
                if n[y][xx] >= tree {
                    break;
                }
            }

            for xx in x + 1..width {
                east += 1;
                if n[y][xx] >= tree {
                    break;
                }
            }

            grid[y][x] = north * east * south * west;
        }
    }

    grid.iter().flatten().copied().max().unwrap()
}

util::register!(parse, part1, part2);
