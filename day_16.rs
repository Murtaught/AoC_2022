#![allow(dead_code)]
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::*,
    process::Command,
};

#[derive(Debug, Eq, PartialEq)]
struct Valve {
    index: usize,
    flow: u64,
    tunnels: Vec<usize>,
}

#[derive(Debug, Default)]
struct Graph {
    valves: Vec<Valve>,
    indices: HashMap<String, usize>,
    dist: Vec<Vec<usize>>,
    non_zero: Vec<usize>,
}

impl Graph {
    fn new() -> Self {
        Self::default()
    }

    fn add_valve(&mut self, name: &str) -> usize {
        if let Some(&index) = self.indices.get(name) {
            index
        } else {
            let index = self.valves.len();
            self.indices.insert(name.to_string(), index);
            self.valves.push(Valve {
                index,
                flow: 0,
                tunnels: vec![],
            });
            index
        }
    }

    fn index_of(&self, name: &str) -> Option<usize> {
        self.indices.get(name).copied()
    }

    fn get(&self, name: &str) -> Option<&Valve> {
        self.index_of(name).map(|i| &self.valves[i])
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Valve> {
        self.index_of(name).map(|i| &mut self.valves[i])
    }

    fn set_flow(&mut self, name: &str, flow: u64) {
        let valve = self
            .get_mut(name)
            .unwrap_or_else(|| panic!("Valve {name:?} not found!"));

        valve.flow = flow;
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        let a = self
            .index_of(from)
            .unwrap_or_else(|| panic!("Valve {from:?} not found!"));

        let b = self
            .index_of(to)
            .unwrap_or_else(|| panic!("Valve {to:?} not found!"));

        self.valves[a].tunnels.push(b);
    }

    fn parse() -> Graph {
        let mut g = Graph::new();

        while let Some(line) = gets() {
            lazy_static! {
                static ref RE: Regex = Regex::new(
                    r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([\w\s,]+)$"
                )
                .unwrap();
            }

            let caps = RE.captures(&line).unwrap();

            let name = &caps[1];
            let flow: u64 = caps[2].parse().unwrap();
            let mut neighbors: Vec<&str> = caps[3].split(',').map(|s| s.trim()).collect();
            neighbors.sort();

            g.add_valve(name);
            g.set_flow(name, flow);

            for nbr in neighbors {
                g.add_valve(nbr);
                g.add_edge(name, nbr);
            }
        }

        g.compute_dist();

        g.non_zero = g
            .valves
            .iter()
            .enumerate()
            .filter(|&(_, v)| v.flow > 0)
            .map(|(i, _)| i)
            .collect();

        g
    }

    fn write_dot(&self, f: &mut impl Write) -> std::io::Result<()> {
        writeln!(f, "strict graph G {{")?;
        // writeln!(f, "    rankdir=\"LR\";")?;

        let mut indices: Vec<(&str, usize)> =
            self.indices.iter().map(|(n, i)| (n.as_str(), *i)).collect();

        indices.sort_by_key(|&(name, _)| name);

        for (name, i) in indices {
            let valve = &self.valves[i];
            let flow = valve.flow;
            write!(f, "    valve_{i} [label=\"{i}: {name}\\n{flow}\"")?;

            if flow > 0 {
                write!(f, ", fontcolor=\"red\"")?;
            }
            writeln!(f, "];")?;

            for &j in &valve.tunnels {
                writeln!(f, "    valve_{i} -- valve_{j};")?;
            }

            writeln!(f)?;
        }
        writeln!(f, "}}")
    }

    fn bfs(&self, start: usize) -> Vec<usize> {
        let n = self.valves.len();

        let mut dist = vec![usize::MAX; n];
        dist[start] = 0;

        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(cur) = queue.pop_front() {
            for &index in &self.valves[cur].tunnels {
                if dist[index] == usize::MAX {
                    dist[index] = dist[cur] + 1;
                    queue.push_back(index);
                }
            }
        }

        dist
    }

    fn compute_dist(&mut self) {
        let n = self.valves.len();

        self.dist = vec![Vec::new(); n];
        for i in 0..n {
            self.dist[i] = self.bfs(i);
        }
    }
}

fn show_graph(g: &Graph) {
    const PATH_DOT: &str = "/tmp/graph.dot";
    const PATH_PNG: &str = "/tmp/graph.dot.png";

    let mut f = File::create(PATH_DOT).unwrap();
    g.write_dot(&mut f).unwrap();
    drop(f);

    Command::new("dot")
        .arg("-Tpng")
        .arg("-O")
        .arg(PATH_DOT)
        .output()
        .expect("Failed to run \"dot\".");

    // Command::new("sxiv")
    //     .arg(PATH_PNG)
    //     .output()
    //     .expect("Failed to run sxiv.");
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
struct BitMask(u64);

impl BitMask {
    const BIT_COUNT: usize = 64;

    fn all_set(bit_count: usize) -> Self {
        Self((1 << bit_count) - 1)
    }

    fn is_set(&self, bit: usize) -> bool {
        ((self.0 >> bit) & 1) == 1
    }

    fn set(&mut self, bit: usize, value: bool) {
        self.0 = if value {
            self.0 | (1 << bit)
        } else {
            self.0 & !(1 << bit)
        }
    }

    fn split<T: Clone>(&self, slice: &[T]) -> (Vec<T>, Vec<T>) {
        let mut v0 = Vec::new();
        let mut v1 = Vec::new();

        for (i, x) in slice.iter().enumerate() {
            if self.is_set(i) {
                v1.push(x.clone());
            } else {
                v0.push(x.clone());
            }
        }

        (v0, v1)
    }
}

struct BitMaskIter {
    mask: BitMask,
    bit: usize,
}

impl BitMaskIter {
    fn new(mask: BitMask) -> Self {
        Self { mask, bit: 0 }
    }
}

impl Iterator for BitMaskIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.bit < BitMask::BIT_COUNT {
            self.bit += 1;
            if self.mask.is_set(self.bit - 1) {
                return Some(self.bit - 1);
            }
        }

        None
    }
}

impl IntoIterator for BitMask {
    type Item = usize;
    type IntoIter = BitMaskIter;

    fn into_iter(self) -> Self::IntoIter {
        BitMaskIter::new(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Key {
    start_index: usize,
    remaining: BitMask,
    time_left: u64,
}

type Cache = HashMap<Key, u64>;

#[derive(Debug, Clone, Default)]
struct Stats {
    cache_hits: usize,
    total: usize,
}

fn solve(g: &Graph, key: Key, cache: &mut Cache, stats: &mut Stats) -> u64 {
    stats.total += 1;
    if let Some(&ans) = cache.get(&key) {
        stats.cache_hits += 1;
        return ans;
    }

    let mut ans = 0;
    let i = key.start_index;

    // Let's try each non-zero valves.
    for j_index in key.remaining.into_iter() {
        let j = g.non_zero[j_index];

        // Time to travel to valve `j` and to open it.
        let need_time = (g.dist[i][j] + 1) as u64;

        if key.time_left > need_time {
            let mut next_remaining = key.remaining;
            next_remaining.set(j_index, false);

            let next_key = Key {
                start_index: j,
                remaining: next_remaining,
                time_left: key.time_left - need_time,
            };

            let mut next_ans = next_key.time_left * g.valves[j].flow;
            next_ans += solve(g, next_key, cache, stats);

            ans = ans.max(next_ans);
        }
    }

    cache.insert(key, ans);
    ans
}

fn main() {
    let g = Graph::parse();

    let start_index = g.index_of("AA").unwrap();
    // assert_eq!(start_index, 0);

    let mut cache: Cache = HashMap::new();
    let mut stats = Stats::default();
    let ans_1 = solve(
        &g,
        Key {
            start_index,
            remaining: BitMask::all_set(g.non_zero.len()),
            time_left: 30,
        },
        &mut cache,
        &mut stats,
    );
    println!("ans_1: {ans_1}");

    let m = g.non_zero.len();
    let masks_count = 1 << m;

    let mut ans_2 = 0;
    let mut processed_masks = HashSet::new();

    for mask in 0..masks_count {
        let inverse_mask = !mask & (masks_count - 1);
        if processed_masks.contains(&inverse_mask) {
            continue;
        }

        let sol_1 = solve(
            &g,
            Key {
                start_index,
                remaining: BitMask(mask),
                time_left: 26,
            },
            &mut cache,
            &mut stats,
        );
        let sol_2 = solve(
            &g,
            Key {
                start_index,
                remaining: BitMask(inverse_mask),
                time_left: 26,
            },
            &mut cache,
            &mut stats,
        );

        ans_2 = ans_2.max(sol_1 + sol_2);

        processed_masks.insert(mask);
    }

    println!("ans_2: {ans_2}");
}

fn gets() -> Option<String> {
    let mut line = String::new();
    let count = stdin().read_line(&mut line).unwrap();
    if count > 0 {
        Some(line.trim().to_string())
    } else {
        None
    }
}
