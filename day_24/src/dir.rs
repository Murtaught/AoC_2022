use std::fmt::Display;

use Dir::*;

use crate::loc::Loc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    pub fn parse(c: char) -> Option<Dir> {
        match c {
            '^' => Some(N),
            '>' => Some(E),
            'v' => Some(S),
            '<' => Some(W),
            _ => None,
        }
    }

    pub fn to_char(self) -> char {
        match self {
            N => '^',
            E => '>',
            S => 'v',
            W => '<',
        }
    }

    pub fn nudge(self, mut loc: Loc) -> Loc {
        match self {
            N => loc.i -= 1,
            E => loc.j += 1,
            S => loc.i += 1,
            W => loc.j -= 1,
        }
        loc
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
