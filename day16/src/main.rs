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
use state::part1::State;
use state::part2::State2;

pub mod node;
use node::Node;

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

fn part1(n: &[In]) -> Out {
    let mut graph = Graph::from_nodes(n);
    graph.collapse_edges();

    std::fs::write("graph.dot", graph.to_string().as_bytes()).unwrap();

    let initial = State::new("AA", &graph);
    let mut states = BTreeMap::new();
    states.insert((initial.time_elapsed, initial.location), vec![initial]);

    let mut finished_states: Vec<State> = vec![];

    let max_time = 30;

    loop {
        let n = states.values().flatten().count();
        println!("{n} states, {} finished", finished_states.len());
        if n == 0 {
            break;
        }

        let all_choices = states
            .into_par_iter()
            .flat_map(|(_key, group)| group)
            .flat_map(|state| state.choices(max_time))
            .collect::<Vec<_>>();

        let mut new_states = BTreeMap::<_, Vec<State>>::new();

        for choice in all_choices {
            if choice.time_elapsed == max_time {
                finished_states.push(choice);
            } else {
                new_states
                    .entry((choice.time_elapsed, choice.location))
                    .or_default()
                    .push(choice);
            }
        }

        new_states
            .par_iter_mut()
            .for_each(|(_key, group)| prune(group));

        states = new_states;
    }

    finished_states
        .iter()
        .map(|s| s.pressure_released)
        .max()
        .unwrap()
}

fn part2(n: &[In]) -> Out {
    let graph = Graph::from_nodes(n);
    //collapse_edges(&mut graph);

    let initial = State2::new("AA", &graph);
    let mut states = BTreeMap::<_, Vec<State2>>::new();
    states.insert((initial.location, initial.elephant), vec![initial]);

    let mut finished_states: Vec<State2> = vec![];

    let max_time = 26;

    let mut i = 0;

    loop {
        let n = states.values().flatten().count();
        println!("t={i}: {n} states");
        i += 1;
        if n == 0 {
            break;
        }

        let all_choices = states
            .into_par_iter()
            .flat_map(|(_key, group)| group)
            .flat_map(|state| state.choices(max_time))
            .collect::<Vec<_>>();

        let mut new_states = BTreeMap::<_, Vec<State2>>::new();

        for choice in all_choices {
            if choice.time_elapsed == max_time {
                finished_states.push(choice);
            } else {
                new_states
                    .entry((choice.location, choice.elephant))
                    .or_default()
                    .push(choice);
            }
        }

        new_states
            .par_iter_mut()
            .for_each(|(_key, group)| prune(group));

        states = new_states;
    }

    finished_states
        .iter()
        .map(|s| s.pressure_released)
        .max()
        .unwrap()
}

util::register!(Node::parse, part1, part2);
