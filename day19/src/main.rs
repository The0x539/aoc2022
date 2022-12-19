#![cfg_attr(test, feature(test))]

use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashSet},
};

use rayon::prelude::*;

use util::*;

type N = u32;

type In = Vec<Blueprint>;
type Out = N;

struct Blueprint {
    id: N,
    ore_cost: N,
    clay_cost: N,
    obsidian_cost: (N, N),
    geode_cost: (N, N),
}

fn parse(s: &'static str) -> In {
    #[cfg(test)]
    let lines = s.split("\n\n");
    #[cfg(not(test))]
    let lines = s.lines();

    lines.map(parse_bp).collect()
}

fn parse_bp(s: &str) -> Blueprint {
    let i = ints(s);
    Blueprint {
        id: i[0],
        ore_cost: i[1],
        clay_cost: i[2],
        obsidian_cost: (i[3], i[4]),
        geode_cost: (i[5], i[6]),
    }
}

impl Blueprint {
    pub fn quality_level(&self, x: N) -> N {
        self.id * self.max_output(x)
    }

    pub fn max_output(&self, x: N) -> N {
        let mut states = HashSet::new();
        states.insert(State::new());

        for _i in 0..x {
            println!("{} {}: {}", self.id, _i, states.len());
            states = states.into_iter().flat_map(|s| s.choices(self)).collect();
        }

        states.into_iter().map(|s| s.geodes).max().unwrap()
    }

    pub fn max_ore_cost(&self) -> N {
        [
            self.ore_cost,
            self.clay_cost,
            self.obsidian_cost.0,
            self.geode_cost.0,
        ]
        .into_iter()
        .max()
        .unwrap()
    }
}

#[derive(Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct State {
    ore: N,
    clay: N,
    obsidian: N,
    geodes: N,

    ore_robots: N,
    clay_robots: N,
    obsidian_robots: N,
    geode_robots: N,
}

impl State {
    fn new() -> Self {
        Self {
            ore_robots: 1,
            ..Default::default()
        }
    }

    fn tick(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }

    fn build_ore_robot(mut self, blueprint: &Blueprint) -> Option<Self> {
        self.ore = self.ore.checked_sub(blueprint.ore_cost)?;
        self.tick();
        self.ore_robots += 1;
        Some(self)
    }

    fn build_clay_robot(mut self, blueprint: &Blueprint) -> Option<Self> {
        self.ore = self.ore.checked_sub(blueprint.clay_cost)?;
        self.tick();
        self.clay_robots += 1;
        Some(self)
    }

    fn build_obsidian_robot(mut self, blueprint: &Blueprint) -> Option<Self> {
        self.ore = self.ore.checked_sub(blueprint.obsidian_cost.0)?;
        self.clay = self.clay.checked_sub(blueprint.obsidian_cost.1)?;
        self.tick();
        self.obsidian_robots += 1;
        Some(self)
    }

    fn build_geode_robot(mut self, blueprint: &Blueprint) -> Option<Self> {
        self.ore = self.ore.checked_sub(blueprint.geode_cost.0)?;
        self.obsidian = self.obsidian.checked_sub(blueprint.geode_cost.1)?;
        self.tick();
        self.geode_robots += 1;
        Some(self)
    }

    fn choices(mut self, blueprint: &Blueprint) -> Vec<Self> {
        let mut selves = Vec::new();

        let mut foo = 0;

        if self.ore_robots < blueprint.max_ore_cost() {
            selves.extend(self.build_ore_robot(blueprint));
            foo += 1;
        }
        if self.clay_robots < blueprint.obsidian_cost.1 {
            selves.extend(self.build_clay_robot(blueprint));
            foo += 1;
        }
        if self.obsidian_robots < blueprint.geode_cost.1 {
            selves.extend(self.build_obsidian_robot(blueprint));
            foo += 1;
        }
        selves.extend(self.build_geode_robot(blueprint));
        foo += 1;

        if selves.len() < foo {
            self.tick();
            selves.push(self);
        }

        selves
    }
}

fn part1(n: &In) -> Out {
    n.par_iter().map(|bp| bp.quality_level(24)).sum()
}

fn part2(n: &In) -> Out {
    n.par_iter().take(3).map(|bp| bp.max_output(32)).product()
}

util::register!(parse, part1, part2, @alt);
