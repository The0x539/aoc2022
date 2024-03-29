#![cfg_attr(test, feature(test))]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use rayon::prelude::*;

type N = u32;

type In = Node;
type Out = N;

pub mod graph;
use graph::Graph;

pub mod state;
use state::part1::State1;
use state::part2::State2;

pub mod node;
use node::Node;

fn part1(n: &[In]) -> Out {
    let mut graph = Graph::from_nodes(n);
    graph.collapse_edges();
    solution(State1::new("AA", &graph), 30)
}

fn part2(n: &[In]) -> Out {
    let mut graph = Graph::from_nodes(n);
    graph.collapse_edges();
    solution(State2::new("AA", &graph), 26)
}

fn solution<S>(initial_state: S, max_time: N) -> N
where
    S: state::State + Send + Sync,
    S::Key: Send + Sync,
{
    let mut states = BTreeMap::new();
    states.insert(initial_state.key(), vec![initial_state]);

    let mut finished_states = vec![];

    for i in 1.. {
        let all_choices = states
            .into_par_iter()
            .flat_map(|(_key, group)| group)
            .flat_map(|state| state.choices(max_time))
            .collect::<Vec<_>>();

        states = BTreeMap::new();

        for choice in all_choices {
            if choice.time_elapsed() == max_time {
                finished_states.push(choice);
            } else {
                states.entry(choice.key()).or_default().push(choice);
            }
        }

        states.par_iter_mut().for_each(|(_key, group)| prune(group));

        let n = states.values().flatten().count();
        if n == 0 {
            break;
        } else {
            print!("t={i}: {n} states");
            let nf = finished_states.len();
            if nf > 0 {
                print!(", {nf} finished");
            }
            println!();
        }
    }

    finished_states
        .iter()
        .map(|s| s.pressure_released())
        .max()
        .unwrap()
}

fn prune<T: PartialOrd>(items: &mut Vec<T>) {
    let mut to_remove = BTreeSet::new();

    for (i, a) in items.iter().enumerate() {
        if to_remove.contains(&i) {
            continue;
        }

        for (j, b) in items.iter().enumerate().skip(i + 1) {
            if to_remove.contains(&j) {
                continue;
            }

            match a.partial_cmp(b) {
                Some(Ordering::Greater) | Some(Ordering::Equal) => {
                    to_remove.insert(j);
                }
                Some(Ordering::Less) => {
                    to_remove.insert(i);
                }
                None => (),
            }
        }
    }

    for j in to_remove.into_iter().rev() {
        items.remove(j);
    }
}

util::register!(Node::parse, part1, part2);
