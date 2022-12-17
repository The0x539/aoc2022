use crate::N;
use fnv::FnvHashMap as HashMap;

#[derive(Clone, PartialEq)]
pub struct Node {
    pub name: &'static str,
    pub flow: N,
    pub adjacencies: HashMap<&'static str, N>,
}

impl Node {
    pub fn parse(s: &'static str) -> Self {
        let tokens = s.split_whitespace().collect::<Vec<_>>();

        let name = tokens[1];
        let flow = tokens[4][5..].strip_suffix(';').unwrap().parse().unwrap();

        let adjacencies = tokens[9..]
            .iter()
            .map(|s| s.strip_suffix(',').unwrap_or(s))
            .map(|n| (n, 1))
            .collect();

        Node {
            name,
            flow,
            adjacencies,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.adjacencies.len() == 1
    }

    pub fn is_removable(&self) -> bool {
        self.adjacencies.len() == 2 && self.flow == 0
    }
}
