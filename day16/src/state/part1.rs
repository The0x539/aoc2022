use super::{silly_comparison, SillySet};
use crate::graph::Graph;
use crate::{Node, N};
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq)]
pub struct State<'a> {
    graph: &'a Graph,
    pub location: &'static str,
    pub time_elapsed: N,
    opened_valves: SillySet<'a>,
    pub pressure_released: N,
}

impl<'a> State<'a> {
    pub fn new(location: &'static str, graph: &'a Graph) -> Self {
        Self {
            graph,
            location,
            time_elapsed: 0,
            opened_valves: SillySet::new(&graph.keyer),
            pressure_released: 0,
        }
    }

    fn release_pressure(&mut self, time: N) {
        for name in self.opened_valves.iter() {
            self.pressure_released += self.graph.flow(name) * time;
        }
    }

    fn elapse_time(&mut self, time: N) {
        self.time_elapsed += time;
        self.release_pressure(time);
    }

    fn current_node(&self) -> &Node {
        &self.graph.map[self.location]
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

        if self.opened_valves.len() == self.graph.map.len() {
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
