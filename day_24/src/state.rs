use crate::{dir::Dir, loc::{Loc, IndexedLoc}};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    n: i16,
    m: i16,
    p: usize,
    index: usize,
    map: HashMap<Loc, Vec<Dir>>,
}

impl State {
    pub fn parse(content: &str) -> Self {
        let mut map = HashMap::<Loc, Vec<Dir>>::new();

        let field: Vec<&str> = content
            .lines()
            .skip(1)
            .map(|line| &line[1..(line.len() - 1)])
            .take_while(|line| !line.contains('#'))
            .collect();

        for (i, line) in field.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if let Some(dir) = Dir::parse(c) {
                    let loc = Loc {
                        i: i as i16,
                        j: j as i16,
                    };
                    map.entry(loc).or_default().push(dir);
                }
            }
        }

        let n = field.len();
        assert!(n > 0);

        let m = field[0].len();
        assert!(field.iter().all(|line| line.len() == m));

        Self::initial(n, m, map)
    }

    pub fn initial(n: usize, m: usize, map: HashMap<Loc, Vec<Dir>>) -> Self {
        Self {
            n: n as i16,
            m: m as i16,
            p: num::integer::lcm(n, m),
            index: 0,
            map,
        }
    }

    pub fn height(&self) -> usize {
        self.n as usize
    }

    pub fn width(&self) -> usize {
        self.m as usize
    }

    pub fn period(&self) -> usize {
        self.p
    }

    pub fn is_empty(&self, loc: Loc) -> bool {
        !self.map.contains_key(&loc)
    }

    pub fn advance(&mut self) -> bool {
        let mut next = HashMap::<Loc, Vec<Dir>>::new();

        for (&loc, vec) in &self.map {
            for &dir in vec {
                let loc = self.teleport(dir.nudge(loc));
                next.entry(loc).or_default().push(dir);
            }
        }

        self.map = next;

        self.index += 1;
        if self.index >= self.p {
            self.index = 0;
            return false;
        }

        true
    }

    fn teleport(&self, mut loc: Loc) -> Loc {
        loc.i = (loc.i + self.n) % self.n;
        loc.j = (loc.j + self.m) % self.m;
        loc
    }

    pub fn nudge_pawn(&self, mut loc: IndexedLoc, dir: Option<Dir>) -> Option<IndexedLoc> {
        assert!(self.is_valid(loc.loc));
        if let Some(dir) = dir {
            loc.loc = dir.nudge(loc.loc);
            if !self.is_valid(loc.loc) {
                return None;
            }
        }

        loc.index = (loc.index + 1) % (self.p as u32);
        Some(loc)
    }

    fn is_valid(&self, loc: Loc) -> bool {
        if loc.i == -1 && loc.j == 0 {
            return true;
        }

        if loc.i == self.n && loc.j == self.m - 1 {
            return true;
        }

        (0..self.n).contains(&loc.i) && (0..self.m).contains(&loc.j)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "State(n: {}, m: {}, i: {}/{})",
            self.n, self.m, self.index, self.p
        )?;

        write!(f, "#.")?;
        for _ in 0..self.m {
            write!(f, "#")?;
        }
        writeln!(f)?;

        for i in 0..self.n {
            write!(f, "#")?;
            for j in 0..self.m {
                let c = if let Some(vec) = self.map.get(&Loc { i, j }) {
                    if vec.len() > 1 {
                        (b'0' + vec.len() as u8) as char
                    } else {
                        vec[0].to_char()
                    }
                } else {
                    '.'
                };

                write!(f, "{c}")?;
            }
            writeln!(f, "#")?;
        }

        for _ in 0..self.m {
            write!(f, "#")?;
        }
        write!(f, ".#")
    }
}
