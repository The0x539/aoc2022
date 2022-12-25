#![cfg_attr(test, feature(test))]

type N = i64;

type In = &'static str;
type Out = String;

fn parse(s: &'static str) -> In {
    s
}

fn unsnafu(s: &str) -> i64 {
    let mut n = 0;
    for c in s.chars() {
        let digit = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!(),
        };
        n = (n * 5) + digit;
    }
    n
}

fn snafu(n: N) -> String {
    let mut place = 1;
    let mut m = -2;
    while nine_nine_nine(place * 5) < n {
        place *= 5;
        m -= 2 * place;
    }

    let mut s = String::new();

    while place > 0 {
        let mut digit = -2;
        while m + place <= n {
            m += place;
            digit += 1;
        }
        place /= 5;

        s.push(match digit {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            x => panic!("{x}"),
        });
    }
    s
}

fn nine_nine_nine(mut place: N) -> N {
    let mut n = 2 * place;
    while place > 0 {
        assert!(place % 5 == 0 || place == 1, "{place}");
        place /= 5;
        n -= 2 * place;
    }
    n
}

fn part1(n: &[In]) -> Out {
    let sum = n.iter().copied().map(unsnafu).sum::<N>();
    snafu(sum)
}

fn part2(_: &[In]) -> Out {
    String::new()
}

util::register!(parse, part1, part2);
