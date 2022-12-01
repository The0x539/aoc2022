use std::fmt::{Debug, Display};
use std::str::FromStr;

pub fn poarse<T>(s: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    s.parse().unwrap()
}

pub fn parse_input_lines<T, F: FnMut(&'static str) -> T>(input_data: &'static str, f: F) -> Vec<T> {
    input_data.lines().map(str::trim).map(f).collect()
}

pub fn run<Parser, Part1, Part2, In, Out>(
    input_data: &'static str,
    parser: Parser,
    part1: Part1,
    part2: Part2,
) where
    Parser: FnMut(&'static str) -> In,
    Part1: FnOnce(&[In]) -> Out,
    Part2: FnOnce(&[In]) -> Out,
    Out: Display,
{
    let input = parse_input_lines(input_data, parser);

    let output1 = part1(&input);
    println!("{output1}");

    let output2 = part2(&input);
    println!("{output2}");
}

pub fn run_alt<Parser, Part1, Part2, In, Out>(
    input_data: &'static str,
    parser: Parser,
    part1: Part1,
    part2: Part2,
) where
    Parser: FnOnce(&'static str) -> In,
    Part1: FnOnce(&In) -> Out,
    Part2: FnOnce(&In) -> Out,
    Out: Display,
{
    let input = parser(input_data);

    let output1 = part1(&input);
    println!("{output1}");

    let output2 = part2(&input);
    println!("{output2}");
}

pub fn test<Parser, Part1, Part2, In, Out>(
    test_data: &'static str,
    output_data: &'static str,
    parser: Parser,
    part1: Part1,
    part2: Part2,
) where
    Parser: FnMut(&str) -> In,
    Part1: FnOnce(&[In]) -> Out,
    Part2: FnOnce(&[In]) -> Out,
    Out: Debug + FromStr + PartialEq,
    Out::Err: Debug,
{
    let input = parse_input_lines(test_data, parser);
    let (x, y) = parse_output::<Out>(output_data);

    assert_eq!(part1(&input), x);
    assert_eq!(part2(&input), y);
}

pub fn parse_output<T>(output_data: &'static str) -> (T, T)
where
    T: FromStr,
    T::Err: Debug,
{
    let (a, b) = output_data.split_once("\n").unwrap();
    let [x, y] = [a, b].map(poarse::<T>);
    (x, y)
}

#[macro_export]
macro_rules! register {
    ($parser:expr, $part1:expr, $part2:expr) => {
        $crate::register!($parser, $part1, $part2, run, test);
    };

    ($parser:expr, $part1:expr, $part2:expr, @alt) => {
        $crate::register!($parser, $part1, $part2, run_alt, test_alt);
    };

    ($parser:expr, $part1:expr, $part2:expr, $run:ident, $test:ident) => {
        const INPUT: &str = include_str!("../input.txt");

        fn main() {
            $crate::$run(INPUT, $parser, $part1, $part2);
        }

        #[cfg(test)]
        extern crate test;

        const TEST_INPUT: &str = include_str!("../test.txt");
        const TEST_OUTPUT: &str = include_str!("../test.out.txt");

        #[cfg(test)]
        #[test]
        fn test_part1() {
            let input = $crate::parse_input_lines(TEST_INPUT, $parser);
            let (x, _y) = $crate::parse_output(TEST_OUTPUT);

            assert_eq!($part1(&input), x);
        }

        #[cfg(test)]
        #[test]
        fn test_part2() {
            let input = $crate::parse_input_lines(TEST_INPUT, $parser);
            let (_x, y) = $crate::parse_output(TEST_OUTPUT);

            assert_eq!($part2(&input), y);
        }
    };
}

// TODO: benchmarking
