pub mod part1;
pub mod part2;

use crate::N;
use fnv::FnvHashSet as HashSet;
use std::cmp::Ordering;

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
