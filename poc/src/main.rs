#![cfg_attr(test, feature(test))]

fn part1(n: &[i32]) -> i32 {
    n.iter().sum()
}

fn part2(n: &[i32]) -> i32 {
    n.iter().product()
}

util::register!(util::poarse::<i32>, part1, part2);
