use Direction::*;

pub type Loc = (i64, i64);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    E = 0,
    S = 1,
    W = 2,
    N = 3,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pawn {
    pub loc: Loc,
    pub dir: Direction,
}

impl Direction {
    pub fn turn_left(&mut self) {
        *self = match self {
            E => N,
            S => E,
            W => S,
            N => W,
        };
    }

    pub fn turn_right(&mut self) {
        *self = match self {
            E => S,
            S => W,
            W => N,
            N => E,
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            E => W,
            S => N,
            W => E,
            N => S,
        }
    }

    pub fn direct(self, (mut i, mut j): Loc) -> Loc {
        match self {
            E => j += 1,
            S => i += 1,
            W => j -= 1,
            N => i -= 1,
        };
        (i, j)
    }
}

impl Pawn {
    pub fn reverse(&self) -> Pawn {
        Pawn {
            dir: self.dir.reverse(),
            loc: self.loc,
        }
    }

    pub fn password(&self) -> i64 {
        let (i, j) = self.loc;
        1000 * (i + 1) + 4 * (j + 1) + self.dir as i64
    }
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => E,
            1 => S,
            2 => W,
            3 => N,
            _ => panic!("Invalid numerical value"),
        }
    }
}
