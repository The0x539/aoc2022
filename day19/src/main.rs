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

    lines
        .enumerate()
        .map(|(i, line)| parse_bp(i as N + 1, line))
        .collect()
}

fn parse_bp(id: N, s: &str) -> Blueprint {
    let i = ints(s);
    Blueprint {
        id,
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
            /*
            println!("{_i}: {}", states.len());
            let mut vstates = states
                .into_par_iter()
                .flat_map(|s| s.choices(self))
                .collect::<Vec<_>>();

            let foo = vstates.len();
            prune(&mut vstates);
            let bar = vstates.len();
            println!("{foo} -> {bar}");
            states = vstates.into_iter().collect();
            */
            states = states.into_iter().flat_map(|s| s.choices(self)).collect();
        }

        states.into_iter().map(|s| s.geodes).max().unwrap()
    }
}

// doesn't work
pub fn prune(states: &mut Vec<State>) {
    if states.iter().all(|s| s.geode_robots == 0) {
        println!("no geodes yet");
        return;
    }

    let mut to_remove = BTreeSet::new();
    for i in 0..states.len() {
        if to_remove.contains(&i) {
            continue;
        }
        for j in (i + 1)..states.len() {
            if to_remove.contains(&j) {
                continue;
            }

            if gt(&states[i], &states[j]) {
                to_remove.insert(j);
            } else if gt(&states[j], &states[i]) {
                to_remove.insert(i);
            }
        }
    }

    for i in to_remove.into_iter().rev() {
        states.remove(i);
    }
}

// doesn't work
fn gt(a: &State, b: &State) -> bool {
    match (a.geodes.cmp(&b.geodes), a.geode_robots.cmp(&b.geode_robots)) {
        (Ordering::Greater, Ordering::Greater)
        | (Ordering::Greater, Ordering::Equal)
        | (Ordering::Equal, Ordering::Greater) => true,
        _ => false,
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

        selves.extend(self.build_ore_robot(blueprint));
        selves.extend(self.build_clay_robot(blueprint));
        selves.extend(self.build_obsidian_robot(blueprint));
        selves.extend(self.build_geode_robot(blueprint));

        self.tick();
        selves.push(self);

        selves
    }
}

fn part1(n: &In) -> Out {
    n.par_iter().map(|bp| bp.quality_level(24)).sum()
}

fn part2(n: &In) -> Out {
    n.iter().take(3).map(|bp| bp.max_output(32)).product()
}

util::register!(parse, part1, part2, @alt);
