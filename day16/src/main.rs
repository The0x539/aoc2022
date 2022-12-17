#![cfg_attr(test, feature(test))]

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};

use rayon::prelude::*;

use util::*;

type N = u32;

type In = ValveDef;
type Out = N;

#[derive(Clone, PartialEq)]
pub struct ValveDef {
    pub name: &'static str,
    pub flow: N,
    pub adjacencies: HashMap<&'static str, N>,
}

fn parse(s: &'static str) -> In {
    let tokens = s.split_whitespace().collect::<Vec<_>>();

    let name = tokens[1];
    let flow = p(tokens[4][5..].strip_suffix(';').unwrap());

    let adjacencies = tokens[9..]
        .iter()
        .map(|s| s.strip_suffix(',').unwrap_or(s))
        .map(|n| (n, 1))
        .collect();

    ValveDef {
        name,
        flow,
        adjacencies,
    }
}

pub fn graphviz(n: &[In]) {
    println!("graph G {{");

    for foo in n {
        print!("\t{}", foo.name);
        if foo.name == "AA" {
            print!(" [label=\"{}\"]", foo.name);
        } else if foo.flow != 0 {
            print!(" [label=\"{}\"]", foo.flow);
        }
        println!();
    }

    let mut edges: Vec<([&'static str; 2], N)> = n
        .iter()
        .flat_map(|v| v.adjacencies.iter().map(|(dst, len)| ([v.name, dst], *len)))
        .collect();

    for (edge, _) in &mut edges {
        edge.sort();
    }

    for ([a, b], n) in HashMap::<_, _>::from_iter(edges) {
        println!("\t{a} -- {b} [label=\"{n}\"]");
    }

    println!("}}");
}

type Graph = HashMap<&'static str, ValveDef>;

fn find_node_to_remove(g: &Graph) -> Option<&'static str> {
    for (name, node) in g {
        if node.flow == 0 && node.adjacencies.len() == 2 {
            return Some(name);
        }
    }
    None
}

fn remove_node(g: &mut Graph, name: &str) {
    let node = g.remove(name).unwrap();

    assert_eq!(node.adjacencies.len(), 2);

    let mut iter = node.adjacencies.into_iter();
    let (name_a, dist_a) = iter.next().unwrap();
    let (name_b, dist_b) = iter.next().unwrap();

    let dist_ab = dist_a + dist_b;

    let neighbor_a = g.get_mut(name_a).unwrap();
    assert_eq!(neighbor_a.adjacencies.remove(name).unwrap(), dist_a);
    neighbor_a.adjacencies.insert(name_b, dist_ab);

    let neighbor_b = g.get_mut(name_b).unwrap();
    assert_eq!(neighbor_b.adjacencies.remove(name).unwrap(), dist_b);
    neighbor_b.adjacencies.insert(name_a, dist_ab);
}

fn collapse_edges(g: &mut Graph) {
    while let Some(name) = find_node_to_remove(g) {
        remove_node(g, name);
    }
}

#[derive(Clone, PartialEq)]
struct State<'a> {
    graph: &'a Graph,
    location: &'static str,
    time_elapsed: N,
    opened_valves: HashSet<&'static str>,
    pressure_released: N,
}

impl<'a> State<'a> {
    pub fn new(location: &'static str, graph: &'a Graph) -> Self {
        Self {
            graph,
            location,
            time_elapsed: 0,
            opened_valves: HashSet::default(),
            pressure_released: 0,
        }
    }

    fn release_pressure(&mut self, time: N) {
        for name in &self.opened_valves {
            self.pressure_released += self.graph[name].flow * time;
        }
    }

    fn elapse_time(&mut self, time: N) {
        self.time_elapsed += time;
        self.release_pressure(time);
    }

    fn current_node(&self) -> &ValveDef {
        &self.graph[self.location]
    }

    fn with(&self, f: impl FnOnce(&mut Self)) -> Self {
        let mut this = self.clone();
        f(&mut this);
        this
    }

    pub fn idle(&mut self) {
        self.elapse_time(1);
    }

    pub fn open_valve(&mut self) {
        self.elapse_time(1);
        self.opened_valves.insert(self.location);
    }

    pub fn travel_to(&mut self, destination: &'static str) {
        let n = self.current_node();
        assert!(n.adjacencies.contains_key(destination));
        self.elapse_time(n.adjacencies[destination]);
        self.location = destination;
    }

    pub fn choices(&self, max_time: N) -> Vec<Self> {
        assert!(self.time_elapsed < max_time);

        if self.opened_valves.len() == self.graph.len() {
            return vec![self.with(Self::idle)];
        }

        let mut selves = vec![];

        if !self.opened_valves.contains(&self.location) && self.current_node().flow > 0 {
            selves.push(self.with(Self::open_valve));
        }

        for (name, dist) in &self.current_node().adjacencies {
            if self.time_elapsed + dist <= max_time {
                selves.push(self.with(|s| s.travel_to(name)));
            }
        }

        selves
    }
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

            if a >= b {
                to_remove.insert(j);
            } else if b >= a {
                to_remove.insert(i);
            }
        }
    }

    for j in to_remove.into_iter().rev() {
        items.remove(j);
    }
}

fn silly_comparison(sp: N, op: N, sv: &HashSet<&str>, ov: &HashSet<&str>) -> Option<Ordering> {
    if sv.symmetric_difference(ov).next().is_some() {
        // neither set of valves is a subset of the other
        return None;
    }

    assert!(sv.is_subset(ov) || ov.is_subset(sv));

    match (sv.len().cmp(&ov.len()), sp.cmp(&op)) {
        (Ordering::Less, Ordering::Less)
        | (Ordering::Less, Ordering::Equal)
        | (Ordering::Equal, Ordering::Less) => Some(Ordering::Less),

        (Ordering::Greater, Ordering::Greater)
        | (Ordering::Greater, Ordering::Equal)
        | (Ordering::Equal, Ordering::Greater) => Some(Ordering::Greater),

        // they've opened the same valves and released the same pressure
        // (in the same amount of time)
        (Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),

        _ => None,
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.time_elapsed != other.time_elapsed {
            return None;
        }

        if self.location != other.location {
            return None;
        }

        silly_comparison(
            self.pressure_released,
            other.pressure_released,
            &self.opened_valves,
            &other.opened_valves,
        )
    }
}

fn part1(n: &[In]) -> Out {
    let mut graph = n.iter().cloned().map(|v| (v.name, v)).collect::<Graph>();

    collapse_edges(&mut graph);

    let initial = State::new("AA", &graph);

    let mut states = BTreeMap::new();
    states.insert((initial.time_elapsed, initial.location), vec![initial]);

    let mut finished_states: Vec<State> = vec![];

    let max_time = 30;

    loop {
        let n = states.values().flatten().count();
        //println!("{n} states, {} finished", finished_states.len());
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
    let graph = n.iter().cloned().map(|v| (v.name, v)).collect::<Graph>();
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

#[derive(Clone, PartialEq)]
struct State2<'a> {
    graph: &'a Graph,
    location: &'static str,
    elephant: &'static str,
    time_elapsed: N,
    opened_valves: HashSet<&'static str>,
    pressure_released: N,
}

#[derive(Copy, Clone, PartialEq)]
enum Action {
    Idle,
    Open,
    Move(&'static str),
}

impl<'a> State2<'a> {
    pub fn new(location: &'static str, graph: &'a Graph) -> Self {
        for node in graph.values() {
            for edge in &node.adjacencies {
                assert_eq!(*edge.1, 1);
            }
        }

        Self {
            graph,
            location,
            elephant: location,
            time_elapsed: 0,
            opened_valves: HashSet::default(),
            pressure_released: 0,
        }
    }

    fn tick(&mut self) {
        for name in &self.opened_valves {
            self.pressure_released += self.graph[name].flow;
        }
        self.time_elapsed += 1;
    }

    fn current_node(&self) -> &ValveDef {
        &self.graph[self.location]
    }

    fn elephant_node(&self) -> &ValveDef {
        &self.graph[self.elephant]
    }

    fn update(&mut self, action: Action, elephant_action: Action) {
        self.tick();

        match action {
            Action::Idle => (),
            Action::Open => _ = self.opened_valves.insert(self.location),
            Action::Move(dest) => self.location = dest,
        }

        match elephant_action {
            Action::Idle => (),
            Action::Open => _ = self.opened_valves.insert(self.elephant),
            Action::Move(dest) => self.elephant = dest,
        }
    }

    fn with_update(&self, action: Action, elephant_action: Action) -> Self {
        let mut this = self.clone();
        this.update(action, elephant_action);
        this
    }

    pub fn choices(&self, max_time: N) -> Vec<Self> {
        assert!(self.time_elapsed < max_time);

        if self.opened_valves.len() == self.graph.len() {
            let mut wait = self.clone();
            while wait.time_elapsed < max_time {
                wait.update(Action::Idle, Action::Idle);
            }
        }

        let mut my_actions = vec![];

        if !self.opened_valves.contains(&self.location) && self.current_node().flow > 0 {
            my_actions.push(Action::Open);
        }

        for (name, _) in &self.current_node().adjacencies {
            my_actions.push(Action::Move(name));
        }

        let mut elephant_actions = vec![];

        if !self.opened_valves.contains(&self.elephant) && self.elephant_node().flow > 0 {
            elephant_actions.push(Action::Open);
        }

        for (name, _) in &self.elephant_node().adjacencies {
            elephant_actions.push(Action::Move(name));
        }

        let mut choices = Vec::with_capacity(my_actions.len() * elephant_actions.len());

        for x in my_actions {
            for &y in &elephant_actions {
                choices.push(self.with_update(x, y));
            }
        }

        prune(&mut choices);

        choices
    }
}

impl PartialOrd for State2<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.time_elapsed != other.time_elapsed {
            return None;
        }

        if self.location != other.location {
            return None;
        }

        if self.elephant != other.elephant {
            return None;
        }

        silly_comparison(
            self.pressure_released,
            other.pressure_released,
            &self.opened_valves,
            &other.opened_valves,
        )
    }
}

util::register!(parse, part1, part2);
