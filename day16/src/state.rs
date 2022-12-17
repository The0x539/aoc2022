pub mod part1;
pub mod part2;

use crate::graph::Keyer;
use crate::N;
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Copy, Clone, PartialEq)]
pub struct SillySet<'a> {
    bits: u64,
    keyer: &'a Keyer,
}

impl<'a> SillySet<'a> {
    pub fn new(keyer: &'a Keyer) -> Self {
        Self { bits: 0, keyer }
    }

    pub fn contains(&self, key: &str) -> bool {
        let k = self.keyer.get(key);
        self.bits & k != 0
    }

    pub fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }

    pub fn insert(&mut self, key: &str) {
        let k = self.keyer.get(key);
        self.bits |= k;
    }

    pub fn iter(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.keyer
            .inner
            .iter()
            .filter(|(_, k)| self.bits & *k != 0)
            .map(|(x, _)| *x)
    }
}

impl PartialOrd for SillySet<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.bits == other.bits {
            Some(Ordering::Equal)
        } else if self.bits & other.bits == other.bits {
            // self is a strict superset of other
            Some(Ordering::Greater)
        } else if self.bits & other.bits == self.bits {
            // self is a strict subset of other
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

fn silly_comparison(sp: N, op: N, sv: &BTreeSet<&str>, ov: &BTreeSet<&str>) -> Option<Ordering> {
    if (sv.len() > ov.len() && sp < op) || (sv.len() < ov.len() && sp > op) {
        return None;
    }

    let mut foo = false;
    let mut bar = false;
    for item in sv.symmetric_difference(ov) {
        match (sv.contains(item), ov.contains(item)) {
            (true, false) => foo = true,
            (false, true) => bar = true,
            (true, true) | (false, false) => panic!(),
        }
        if foo && bar {
            return None;
        }
    }

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
