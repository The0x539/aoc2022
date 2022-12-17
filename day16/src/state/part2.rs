use crate::graph::Graph;
use crate::prune;
use crate::{Node, N};
use fnv::FnvHashSet as HashSet;
use std::cmp::Ordering;

use super::silly_comparison;

#[derive(Clone, PartialEq)]
pub struct State2<'a> {
    graph: &'a Graph,
    pub location: &'static str,
    pub elephant: &'static str,
    pub time_elapsed: N,
    opened_valves: HashSet<&'static str>,
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
        for (_, _, len) in graph.edges() {
            assert_eq!(len, 1);
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
            self.pressure_released += self.graph.flow(name);
        }
        self.time_elapsed += 1;
    }

    fn current_node(&self) -> &Node {
        &self.graph.0[self.location]
    }

    fn elephant_node(&self) -> &Node {
        &self.graph.0[self.elephant]
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

        if self.opened_valves.len() == self.graph.0.len() {
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
