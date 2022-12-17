use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};

use super::*;

#[derive(PartialEq, Clone)]
pub struct Keyer {
    pub(crate) inner: HashMap<&'static str, u64>,
}

impl Keyer {
    pub fn get(&self, name: &str) -> u64 {
        let v = self.inner[name];
        debug_assert_eq!(v.count_ones(), 1);
        v
    }

    fn from_names(names: impl IntoIterator<Item = &'static str>) -> Self {
        let mut names = Vec::from_iter(names);
        names.sort();
        let mut k = 1;
        let mut inner = HashMap::default();
        for name in names {
            inner.insert(name, k);
            k <<= 1;
        }
        Self { inner }
    }
}

#[derive(PartialEq)]
pub struct Graph {
    pub map: HashMap<&'static str, Node>,
    pub keyer: Keyer,
}

impl Graph {
    pub fn from_nodes(nodes: &[Node]) -> Self {
        let map = nodes.iter().cloned().map(|n| (n.name, n)).collect();
        let keyer = Keyer::from_names(nodes.iter().map(|n| n.name));
        Self { map, keyer }
    }

    fn find_node_to_remove(&self) -> Option<&'static str> {
        for (name, node) in &self.map {
            if node.is_removable() {
                return Some(name);
            }
        }
        None
    }

    fn remove_node(&mut self, name: &str) {
        let node = self.map.remove(name).unwrap();

        assert_eq!(node.adjacencies.len(), 2);

        let mut iter = node.adjacencies.into_iter();
        let (name_a, dist_a) = iter.next().unwrap();
        let (name_b, dist_b) = iter.next().unwrap();

        let dist_ab = dist_a + dist_b;

        let neighbor_a = self.map.get_mut(name_a).unwrap();
        assert_eq!(neighbor_a.adjacencies.remove(name).unwrap(), dist_a);
        neighbor_a.adjacencies.insert(name_b, dist_ab);

        let neighbor_b = self.map.get_mut(name_b).unwrap();
        assert_eq!(neighbor_b.adjacencies.remove(name).unwrap(), dist_b);
        neighbor_b.adjacencies.insert(name_a, dist_ab);
    }

    pub fn collapse_edges(&mut self) {
        while let Some(name) = self.find_node_to_remove() {
            self.remove_node(name);
        }
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        self.map.values()
    }

    pub fn edges(&self) -> impl Iterator<Item = (&'static str, &'static str, N)> + '_ {
        self.nodes()
            .flat_map(|node| std::iter::repeat(node.name).zip(&node.adjacencies))
            .map(|(src, (dst, len))| (src, *dst, *len))
    }

    pub fn flow(&self, name: &str) -> N {
        self.map[name].flow
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "graph G {{")?;

        for node in self.map.values() {
            let (name, flow) = (node.name, node.flow);
            writeln!(f, "\t{name} [label=\"{name}\\n{flow}\"];")?;
        }

        let mut done_edges = HashSet::default();

        for (src, dst, len) in self.edges() {
            if done_edges.contains(&(dst, src)) {
                continue;
            }

            writeln!(f, "\t{src} -- {dst} [len={len},label={len}];")?;
            done_edges.insert((src, dst));
        }

        writeln!(f, "}}")?;

        Ok(())
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
