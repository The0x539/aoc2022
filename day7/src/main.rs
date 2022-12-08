#![cfg_attr(test, feature(test))]

use std::collections::HashMap;

#[derive(Default)]
struct Dir {
    entries: HashMap<&'static str, Entry>,
}

impl Dir {
    pub fn size(&self) -> u64 {
        self.entries.values().map(|e| e.size()).sum()
    }

    pub fn traverse_mut(&mut self, path: &[&'static str]) -> Option<&mut Self> {
        let mut cwd = self;
        for seg in path {
            let ent = cwd.entries.get_mut(seg)?;
            if let Entry::Dir(d) = ent {
                cwd = d;
            } else {
                return None;
            }
        }
        Some(cwd)
    }
}

enum Entry {
    File(u64),
    Dir(Dir),
}

impl Entry {
    pub fn size(&self) -> u64 {
        match self {
            Entry::File(size) => *size,
            Entry::Dir(d) => d.size(),
        }
    }
}

fn make_tree(session: &'static str) -> Dir {
    let mut cwd = vec![];
    let mut root = Dir::default();
    let mut lines = session.lines().map(|s| s.trim()).peekable();
    while let Some(cmd) = lines.next() {
        let cmd = cmd.strip_prefix("$ ").expect("unexpected non-command");

        if let Some(arg) = cmd.strip_prefix("cd ") {
            match arg {
                "/" => cwd.clear(),
                ".." => {
                    cwd.pop();
                }
                x => cwd.push(x),
            }
        } else if cmd == "ls" {
            let cwd_dir = root.traverse_mut(&cwd).unwrap();

            while let Some(entry) = lines.peek() {
                if entry.starts_with('$') {
                    break;
                }

                let (foo, name) = entry.split_once(' ').unwrap();
                if foo == "dir" {
                    cwd_dir.entries.insert(name, Entry::Dir(Default::default()));
                } else {
                    let size = foo.parse::<u64>().unwrap();
                    cwd_dir.entries.insert(name, Entry::File(size));
                }

                lines.next();
            }
        } else {
            panic!("unknown command");
        }
    }
    root
}

fn part1(root: &Dir) -> u64 {
    let mut stack = vec![root];
    let mut size_sum = 0;
    while let Some(dir) = stack.pop() {
        for entry in dir.entries.values() {
            if let Entry::Dir(d) = entry {
                stack.push(d);
            }
        }
        let sz = dir.size();
        if sz <= 100000 {
            size_sum += sz;
        }
    }

    size_sum
}

fn part2(root: &Dir) -> u64 {
    const TOTAL: u64 = 70000000;
    const NEEDED: u64 = 30000000;

    let already_avail: u64 = TOTAL - root.size();
    let additional_needed: u64 = NEEDED - already_avail;

    let mut stack = vec![root];

    let mut result = u64::MAX;

    while let Some(dir) = stack.pop() {
        for entry in dir.entries.values() {
            if let Entry::Dir(d) = entry {
                stack.push(d);
            }
        }
        let sz = dir.size();
        if sz >= additional_needed {
            result = u64::min(result, sz);
        }
    }

    result
}

util::register!(make_tree, part1, part2, @alt);
