use crate::graph::Graph;
use crate::{Node, N};
use std::cmp::Ordering;

use super::{silly_comparison, SillySet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    At(&'static str),
    Towards(&'static str, N),
}

impl Status {
    fn tick(&mut self) {
        if let Status::Towards(dest, dist) = self {
            assert!(*dist > 0);
            *dist -= 1;
            if *dist == 0 {
                *self = Self::At(dest);
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct State2<'a> {
    graph: &'a Graph,
    pub human: Status,
    pub elephant: Status,
    pub time_elapsed: N,
    opened_valves: SillySet<'a>,
    pub pressure_released: N,
}

#[derive(Copy, Clone, PartialEq)]
enum Action {
    Idle,
    Open,
    Move(&'static str),
}

impl<'a> State2<'a> {
    pub fn new(location: &'static str, graph: &'a Graph) -> Self {
        Self {
            graph,
            human: Status::At(location),
            elephant: Status::At(location),
            time_elapsed: 0,
            opened_valves: SillySet::new(&graph.keyer),
            pressure_released: 0,
        }
    }

    fn human_location(&self) -> Option<&'static str> {
        match self.human {
            Status::At(x) => Some(x),
            _ => None,
        }
    }

    fn human_node(&self) -> Option<&Node> {
        self.graph.map.get(self.human_location()?)
    }

    fn elephant_location(&self) -> Option<&'static str> {
        match self.elephant {
            Status::At(x) => Some(x),
            _ => None,
        }
    }

    fn elephant_node(&self) -> Option<&Node> {
        self.graph.map.get(self.elephant_location()?)
    }

    fn apply_update(&mut self, human_action: Action, elephant_action: Action) {
        for name in self.opened_valves.iter() {
            self.pressure_released += self.graph.flow(name);
        }

        match human_action {
            Action::Idle => (),
            Action::Open => {
                let location = self.human_location().unwrap();
                self.opened_valves.insert(location);
            }
            Action::Move(dest) => {
                let node = self.human_node().unwrap();
                self.human = Status::Towards(dest, node.adjacencies[dest]);
            }
        }

        match elephant_action {
            Action::Idle => (),
            Action::Open => {
                let location = self.elephant_location().unwrap();
                self.opened_valves.insert(location);
            }
            Action::Move(dest) => {
                let node = self.elephant_node().unwrap();
                self.elephant = Status::Towards(dest, node.adjacencies[dest]);
            }
        }

        self.elephant.tick();
        self.human.tick();

        self.time_elapsed += 1;
    }

    fn with_update(mut self, human_action: Action, elephant_action: Action) -> Self {
        self.apply_update(human_action, elephant_action);
        self
    }

    fn should_open(&self, node: &Node) -> bool {
        node.flow > 0 && !self.opened_valves.contains(node.name)
    }

    fn get_actions_for(&self, location: Option<&'static str>, max_time: N) -> Vec<Action> {
        let mut actions = vec![];

        if let Some(loc) = location {
            let node = &self.graph.map[loc];

            if self.should_open(node) {
                actions.push(Action::Open);
            }

            for (name, dist) in &node.adjacencies {
                if self.graph.map[name].is_leaf() && self.opened_valves.contains(name) {
                    continue;
                }

                if self.time_elapsed + *dist > max_time {
                    continue;
                }

                actions.push(Action::Move(name));
            }
        }

        if actions.is_empty() {
            actions.push(Action::Idle);
        }

        actions
    }

    pub fn choices(&self, max_time: N) -> Vec<Self> {
        debug_assert!(self.time_elapsed < max_time);

        if self.opened_valves.len() == self.graph.map.len() {
            return vec![self.with_update(Action::Idle, Action::Idle)];
        }

        let human_actions = self.get_actions_for(self.human_location(), max_time);
        let elephant_actions = self.get_actions_for(self.elephant_location(), max_time);

        let mut choices = Vec::with_capacity(human_actions.len() * elephant_actions.len());
        for h in human_actions {
            for &e in &elephant_actions {
                choices.push(self.with_update(h, e));
            }
        }

        choices
    }
}

impl PartialOrd for State2<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.time_elapsed != other.time_elapsed {
            return None;
        }

        if self.human != other.human {
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
