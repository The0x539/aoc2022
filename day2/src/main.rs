#![cfg_attr(test, feature(test))]

#[derive(Debug, Copy, Clone)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn score(self) -> u32 {
        self as u32 + 1
    }

    fn play(self, other: Self) -> u32 {
        use Rps::*;
        match (self, other) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 6,
            (Scissors, Rock) | (Rock, Paper) | (Paper, Scissors) => 0,
            _ => 3,
        }
    }

    fn round(self, other: Self) -> u32 {
        self.play(other) + self.score()
    }

    fn to_outcome(self) -> Outcome {
        match self {
            Self::Rock => Outcome::Lose,
            Self::Paper => Outcome::Draw,
            Self::Scissors => Outcome::Win,
        }
    }

    fn weakness(&self) -> Self {
        use Rps::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn strength(&self) -> Self {
        use Rps::*;
        match self {
            Paper => Rock,
            Scissors => Paper,
            Rock => Scissors,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn value(self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

type In = (Rps, Rps);
type Out = u32;

fn parse(s: &'static str) -> In {
    let you = match s.chars().nth(0).unwrap() {
        'A' => Rps::Rock,
        'B' => Rps::Paper,
        'C' => Rps::Scissors,
        _ => panic!(),
    };
    let opponent = match s.chars().nth(2).unwrap() {
        'X' => Rps::Rock,
        'Y' => Rps::Paper,
        'Z' => Rps::Scissors,
        _ => panic!(),
    };
    (you, opponent)
}

fn part1(n: &[In]) -> Out {
    n.iter().map(|(x, y)| y.round(*x)).sum::<u32>()
}

fn part2(n: &[In]) -> Out {
    let mut s = 0;
    for &(x, y) in n {
        let z = y.to_outcome();
        let w = match z {
            Outcome::Draw => x,
            Outcome::Win => x.weakness(),
            Outcome::Lose => x.strength(),
        };
        s += z.value() + w.score();
    }
    s
}

util::register!(parse, part1, part2);
