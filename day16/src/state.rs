pub mod part1;
pub mod part2;

use crate::graph::Keyer;
use crate::N;
use std::cmp::Ordering;

pub trait State: PartialOrd + Sized {
    type Key: Ord;
    fn key(&self) -> Self::Key;
    fn time_elapsed(&self) -> N;
    fn pressure_released(&self) -> N;
    fn choices(&self, max_time: N) -> Vec<Self>;
}

impl State for part1::State1<'_> {
    type Key = (N, &'static str);

    fn key(&self) -> Self::Key {
        (self.time_elapsed, self.location)
    }

    fn time_elapsed(&self) -> N {
        self.time_elapsed
    }

    fn pressure_released(&self) -> N {
        self.pressure_released
    }

    fn choices(&self, max_time: N) -> Vec<Self> {
        self.choices(max_time)
    }
}

impl State for part2::State2<'_> {
    type Key = (part2::Status, part2::Status);

    fn key(&self) -> Self::Key {
        (self.human, self.elephant)
    }

    fn time_elapsed(&self) -> N {
        self.time_elapsed
    }

    fn pressure_released(&self) -> N {
        self.pressure_released
    }

    fn choices(&self, max_time: N) -> Vec<Self> {
        self.choices(max_time)
    }
}

#[derive(Copy, Clone)]
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

impl PartialEq for SillySet<'_> {
    fn eq(&self, other: &Self) -> bool {
        debug_assert!(std::ptr::eq(self.keyer, other.keyer));
        self.bits == other.bits
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

fn silly_comparison(sp: N, op: N, sv: &SillySet, ov: &SillySet) -> Option<Ordering> {
    debug_assert!(std::ptr::eq(sv, ov));

    match (sv.partial_cmp(&ov)?, sp.cmp(&op)) {
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
