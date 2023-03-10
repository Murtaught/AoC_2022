use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
};
use Dir::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Loc {
    pub i: i64,
    pub j: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Elf {
    pub start_idx: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Dir {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
}

#[derive(Debug, Clone)]
pub struct Simulation {
    pub move_order: Vec<Dir>,
    pub map: HashMap<Loc, Elf>,
}

impl Simulation {
    const DEFAULT_MOVE_ORDER: &[Dir] = &[N, S, W, E];

    pub fn new(elf_locs: Vec<Loc>) -> Self {
        let move_order = Self::DEFAULT_MOVE_ORDER.to_owned();
        let map = elf_locs
            .into_iter()
            .map(|l| (l, Elf { start_idx: 0 }))
            .collect();

        Self { move_order, map }
    }

    pub fn step(&mut self) -> bool {
        let mut next = HashMap::<Loc, Elf>::new();
        let mut propositions = HashMap::<Loc, Vec<Loc>>::new();

        // First half: propose moves:
        for (loc, elf) in &self.map {
            if Dir::ALL
                .iter()
                .all(|&d| !self.map.contains_key(&loc.moved(d)))
            {
                next.insert(*loc, *elf);
                continue;
            }

            let mut found = false;
            for k in 0..4 {
                let dir4 = self.move_order[(elf.start_idx + k) % self.move_order.len()];
                if !dir4
                    .triplet()
                    .into_iter()
                    .any(|d8| self.map.contains_key(&loc.moved(d8)))
                {
                    propositions.entry(loc.moved(dir4)).or_default().push(*loc);
                    found = true;
                    break;
                }
            }

            if !found {
                next.insert(*loc, *elf);
            }
        }

        if propositions.is_empty() {
            // Simulation finished.
            return true;
        }

        for (loc, props) in propositions {
            match props.len() {
                0 => {}
                1 => {
                    next.insert(loc, self.map[&props[0]]);
                }
                _ => {
                    for loc in props {
                        next.insert(loc, self.map[&loc]);
                    }
                }
            }
        }

        for elf in next.values_mut() {
            elf.start_idx = (elf.start_idx + 1) % self.move_order.len();
        }

        self.map = next;
        false
    }

    pub fn boundaries(&self) -> (Loc, Loc) {
        let mut min_i = i64::MAX;
        let mut min_j = i64::MAX;
        let mut max_i = i64::MIN;
        let mut max_j = i64::MIN;

        for loc in self.map.keys() {
            min_i = min(min_i, loc.i);
            min_j = min(min_j, loc.j);
            max_i = max(max_i, loc.i);
            max_j = max(max_j, loc.j);
        }

        (Loc { i: min_i, j: min_j }, Loc { i: max_i, j: max_j })
    }

    pub fn bound_empty_space(&self) -> u64 {
        let mut ret = 0;
        let (min_loc, max_loc) = self.boundaries();

        for i in min_loc.i..=max_loc.i {
            for j in min_loc.j..=max_loc.j {
                if !self.map.contains_key(&Loc { i, j }) {
                    ret += 1;
                }
            }
        }

        ret
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_loc, max_loc) = self.boundaries();
        for i in min_loc.i..=max_loc.i {
            for j in min_loc.j..=max_loc.j {
                write!(
                    f,
                    "{}",
                    if self.map.contains_key(&Loc { i, j }) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TryFrom<usize> for Dir {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            x if x == Self::N as usize => Ok(Self::N),
            x if x == Self::NE as usize => Ok(Self::NE),
            x if x == Self::E as usize => Ok(Self::E),
            x if x == Self::SE as usize => Ok(Self::SE),
            x if x == Self::S as usize => Ok(Self::S),
            x if x == Self::SW as usize => Ok(Self::SW),
            x if x == Self::W as usize => Ok(Self::W),
            x if x == Self::NW as usize => Ok(Self::NW),
            _ => Err(()),
        }
    }
}

impl Dir {
    pub const COUNT: usize = 8;
    pub const ALL: &[Dir] = &[N, NE, E, SE, S, SW, W, NW];

    pub fn triplet(self) -> [Dir; 3] {
        let n = self as usize + Self::COUNT;
        [
            ((n - 1) % Self::COUNT).try_into().unwrap(),
            (n % Self::COUNT).try_into().unwrap(),
            ((n + 1) % Self::COUNT).try_into().unwrap(),
        ]
    }
}

impl Loc {
    pub fn moved(mut self, dir: Dir) -> Loc {
        self.i += match dir {
            S | SE | SW => 1,
            N | NE | NW => -1,
            _ => 0,
        };
        self.j += match dir {
            E | NE | SE => 1,
            W | NW | SW => -1,
            _ => 0,
        };
        self
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}
