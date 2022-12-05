#![cfg_attr(test, feature(test))]

type In = Input;
type Out = String;

#[derive(Debug, Clone)]
struct Input {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

#[derive(Debug, Copy, Clone)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn parse(s: &'static str) -> In {
    let mut lines = s.lines();

    let mut stacks = vec![];
    for line in lines.by_ref() {
        if line.starts_with(" 1") {
            break;
        }

        if stacks.is_empty() {
            let n = (line.len() + 1) / 4;
            stacks = vec![vec![]; n];
        }

        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].insert(0, c);
            }
        }
    }
    lines.next();

    let mut moves = vec![];
    for line in lines {
        let mut vals = line.split_whitespace().skip(1).step_by(2);
        let mv = Move {
            amount: vals.next().unwrap().parse().unwrap(),
            from: vals.next().unwrap().parse().unwrap(),
            to: vals.next().unwrap().parse().unwrap(),
        };
        moves.push(mv);
    }

    Input { stacks, moves }
}

fn part1(n: &In) -> Out {
    let mut state = n.clone();

    for m in state.moves {
        for _ in 0..m.amount {
            let cr = state.stacks[m.from - 1].pop().unwrap();
            state.stacks[m.to - 1].push(cr);
        }
    }

    state.stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn part2(n: &In) -> Out {
    let mut state = n.clone();

    for m in state.moves {
        let src = &mut state.stacks[m.from - 1];
        let group = src.split_off(src.len() - m.amount);
        state.stacks[m.to - 1].extend(group);
    }

    state.stacks.iter().map(|s| s.last().unwrap()).collect()
}

util::register!(parse, part1, part2, @alt);
