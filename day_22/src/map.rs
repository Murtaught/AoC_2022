use std::{fmt::Display, ops::Index, usize};
use Tile::*;

use crate::{
    instr::Instr,
    jump_table::{verify_jump_table, JumpTable},
    pawn::{Direction, Loc, Pawn},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Void,
    Floor,
    Wall,
}

#[derive(Debug)]
pub struct Map {
    pub n: usize,
    pub m: usize,
    pub ss: usize,
    pub d: Vec<Vec<Tile>>,
    pub jt: Option<JumpTable>,
}

impl Tile {
    pub fn to_char(self) -> char {
        match self {
            Void => '█',
            Floor => '.',
            Wall => '#',
        }
    }

    pub fn from_char(c: char) -> Option<Tile> {
        match c {
            ' ' | '█' => Some(Void),
            '.' => Some(Floor),
            '#' => Some(Wall),
            _ => None,
        }
    }
}

impl Map {
    pub fn from_lines(lines: &[&str]) -> Map {
        let n = lines.len();
        let m = lines.iter().map(|line| line.len()).max().unwrap();
        let ss = Self::guess_cube_face_size(n, m).unwrap();
        let mut d = vec![vec![Void; m]; n];

        for (i, &line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                d[i][j] = Tile::from_char(c).unwrap();
            }
        }

        Map {
            n,
            m,
            ss,
            d,
            jt: None,
        }
    }

    fn guess_cube_face_size(mut n: usize, mut m: usize) -> Option<usize> {
        if n < m {
            std::mem::swap(&mut n, &mut m);
        }

        if n % 4 == 0 && m % 3 == 0 && n / 4 == m / 3 {
            return Some(n / 4);
        }

        None
    }

    pub fn set_jump_table(&mut self, table: JumpTable) {
        assert_eq!(self.jt, None);
        verify_jump_table(&table);
        self.jt = Some(table);
    }

    pub fn within_bounds(&self, (i, j): Loc) -> bool {
        let n = self.n as i64;
        let m = self.m as i64;
        (0..n).contains(&i) && (0..m).contains(&j)
    }

    pub fn walk(&self, instrs: &[Instr]) -> Pawn {
        let mut pawn = self.start();
        for &instr in instrs {
            use Instr::*;
            match instr {
                GoForward(steps) => {
                    for _ in 0..steps {
                        self.step(&mut pawn);
                    }
                }

                TurnLeft => pawn.dir.turn_left(),
                TurnRight => pawn.dir.turn_right(),
            }
        }
        pawn
    }

    fn start(&self) -> Pawn {
        for i in 0..self.n {
            for j in 0..self.m {
                if self.d[i][j] == Floor {
                    return Pawn {
                        loc: (i as i64, j as i64),
                        dir: Direction::E,
                    };
                }
            }
        }
        panic!("Starting position not found!");
    }

    fn step(&self, pawn: &mut Pawn) {
        assert_eq!(self[pawn.loc], Floor);
        if self.jt.is_some() {
            self.step_jump_table(pawn);
        } else {
            self.step_trace(pawn);
        }
        // eprintln!("after step: {pawn:?}");
    }

    fn step_jump_table(&self, pawn: &mut Pawn) {
        let iss = self.ss as i64 - 1;
        use Direction::*;
        let (zone_index, (mut li, mut lj)) = self.get_zone(pawn);
        let nav_array = self.jt.as_ref().and_then(|jt| jt.get(&zone_index)).unwrap();
        if li == 0 && pawn.dir == Direction::N {
            let (next_zone, next_dir) = nav_array[pawn.dir as usize];
            match next_dir {
                E => {
                    li = lj;
                    lj = 0;
                }
                S => {
                    lj = iss - lj;
                    li = 0;
                }
                W => {
                    li = iss - lj;
                    lj = iss;
                },
                N => {
                    li = iss; 
                }
            }
            let next_pawn = self.pawn_from_zone(next_zone, (li, lj), next_dir);
            if self[next_pawn.loc] == Floor {
                *pawn = next_pawn;
            }
        } else if li as usize == self.ss - 1 && pawn.dir == Direction::S {
            let (next_zone, next_dir) = nav_array[pawn.dir as usize];
            match next_dir {
                E => {
                    li = iss - lj;
                    lj = 0;
                }
                S => {
                    li = 0;
                }
                W => {
                    li = lj;
                    lj = iss;
                },
                N => {
                    lj = iss - lj;
                    li = iss;
                }
            }
            let next_pawn = self.pawn_from_zone(next_zone, (li, lj), next_dir);
            if self[next_pawn.loc] == Floor {
                *pawn = next_pawn;
            }
        } else if lj == 0 && pawn.dir == Direction::W {
            let (next_zone, next_dir) = nav_array[pawn.dir as usize];
            match next_dir {
                E => {
                    li = iss - li;
                    lj = 0;
                }
                S => {
                    lj = li;
                    li = 0;
                }
                W => {
                    lj = iss;
                },
                N => {
                    lj = iss - li;
                    li = iss;
                }
            }
            let next_pawn = self.pawn_from_zone(next_zone, (li, lj), next_dir);
            if self[next_pawn.loc] == Floor {
                *pawn = next_pawn;
            }
        } else if lj as usize == self.ss - 1 && pawn.dir == Direction::E {
            let (next_zone, next_dir) = nav_array[pawn.dir as usize];
            match next_dir {
                E => {
                    lj = 0;
                }
                S => {
                    lj = iss - li;
                    li = 0;
                }
                W => {
                    li = iss - li;
                    lj = iss;
                }, 
                N => {
                    lj = li;
                    li = iss;
                }
            }
            let next_pawn = self.pawn_from_zone(next_zone, (li, lj), next_dir);
            if self[next_pawn.loc] == Floor {
                *pawn = next_pawn;
            }
        } else {
            let next = pawn.dir.direct(pawn.loc);
            assert_ne!(self[next], Void);

            if self[next] == Floor {
                pawn.loc = next;
            }
        }
    }

    /// Get zone index and local zone coordinates of given Pawn.
    fn get_zone(&self, pawn: &Pawn) -> (usize, Loc) {
        let iss = self.ss as i64;
        let zw = self.m as i64 / iss;
        let (gi, gj) = pawn.loc;
        let zone_index = (gi / iss) * zw + gj / iss;
        let (li, lj) = (gi % iss, gj % iss);
        (zone_index as usize, (li, lj))
    }

    fn pawn_from_zone(&self, zone_index: usize, (mut i, mut j): Loc, dir: Direction) -> Pawn {
        let iss = self.ss as i64;
        let zw = self.m as i64 / iss;
        let zi = zone_index as i64 / zw;
        let zj = zone_index as i64 % zw;
        i += zi * iss;
        j += zj * iss;
        Pawn { loc: (i, j), dir }
    }

    // fn jump(pawn: &Pawn, )

    fn step_trace(&self, pawn: &mut Pawn) {
        let next = pawn.dir.direct(pawn.loc);
        match self[next] {
            Void => {
                let next = self.trace(pawn.reverse());
                match self[next] {
                    Floor => {
                        pawn.loc = next;
                    }
                    Wall => {}
                    Void => unreachable!(),
                }
            }

            Floor => {
                pawn.loc = next;
            }
            Wall => {}
        }
    }

    fn trace(&self, mut pawn: Pawn) -> Loc {
        loop {
            let next = pawn.dir.direct(pawn.loc);
            if self[next] == Void {
                break pawn.loc;
            }
            pawn.loc = next;
        }
    }
}

impl<'a> FromIterator<&'a str> for Map {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let lines: Vec<&'a str> = iter.into_iter().collect();
        Map::from_lines(&lines)
    }
}

impl Index<Loc> for Map {
    type Output = Tile;

    fn index(&self, (i, j): Loc) -> &Self::Output {
        if self.within_bounds((i, j)) {
            &self.d[i as usize][j as usize]
        } else {
            &Void
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Map [{}x{}]", self.n, self.m)?;
        for row in &self.d {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
