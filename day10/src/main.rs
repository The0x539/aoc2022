#![cfg_attr(test, feature(test))]

type In = Instr;
type Out = i32;

#[derive(Debug, Copy, Clone)]
enum Instr {
    Noop,
    AddX(i32),
}

fn parse(s: &'static str) -> In {
    if s == "noop" {
        Instr::Noop
    } else {
        Instr::AddX(s[5..].parse().unwrap())
    }
}

fn part1(n: &[In]) -> Out {
    let mut x = 1;
    let mut cc = 0;
    let mut ip = 0;
    let mut foo = false;

    let mut total = 0;

    loop {
        cc += 1;

        let mut dx = 0;

        match n[ip] {
            Instr::Noop => {
                ip += 1;
            }
            Instr::AddX(v) => {
                if foo {
                    foo = false;
                    dx = v;
                    ip += 1;
                } else {
                    foo = true;
                }
            }
        }

        if (cc - 20) % 40 == 0 {
            let strength = cc * x;
            total += strength;
        }

        x += dx;

        if cc == 220 {
            break;
        }
    }
    total
}

fn part2(n: &[In]) -> Out {
    let mut x = 1;
    let mut cc = 0;
    let mut ip = 0;
    let mut foo = false;

    let mut grid = vec![];

    loop {
        cc += 1;

        let mut dx = 0;

        match n[ip] {
            Instr::Noop => {
                ip += 1;
            }
            Instr::AddX(v) => {
                if foo {
                    foo = false;
                    dx = v;
                    ip += 1;
                } else {
                    foo = true;
                }
            }
        }

        let raster = (cc - 1) % 40;
        if raster == 0 {
            grid.push([' '; 40]);
        }
        let pixel = &mut grid.last_mut().unwrap()[raster];
        if (x - 1..=x + 1).contains(&(raster as _)) {
            *pixel = '█';
        }

        x += dx;

        if cc == 240 {
            break;
        }
    }

    for row in grid {
        println!("{}", String::from_iter(row));
    }

    0
}

util::register!(parse, part1, part2);
