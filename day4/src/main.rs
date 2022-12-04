#![cfg_attr(test, feature(test))]

type In = ((u32, u32), (u32, u32));
type Out = i64;

fn parse0(s: &str) -> (u32, u32) {
    let (a, b) = s.split_once('-').unwrap();
    if a.parse::<u32>().is_err() || b.parse::<u32>().is_err() {
        println!("{a:?} {b:?}");
    }

    (util::poarse(a), util::poarse(b))
}

fn parse(s: &'static str) -> In {
    let (x, y) = s.split_once(',').unwrap();
    (parse0(x), parse0(y))
}

fn part1(n: &[In]) -> Out {
    let mut sum = 0;
    for ((a, b), (c, d)) in n {
        if a <= c && b >= d {
            sum += 1;
        } else if c <= a && d >= b {
            sum += 1;
        }
    }
    sum
}

fn part2(n: &[In]) -> Out {
    let mut sum = 0;
    for ((a, b), (c, d)) in n {
        if (c..=d).contains(&a)
            || (c..=d).contains(&b)
            || (a..=b).contains(&c)
            || (a..=b).contains(&d)
        {
            sum += 1;
        }
    }
    sum
}

util::register!(parse, part1, part2);
