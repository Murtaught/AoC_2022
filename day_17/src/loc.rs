use crate::instr::Instr;
use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Loc {
    pub i: u64,
    pub j: u64,
}

impl Loc {
    pub fn new<T>(i: T, j: T) -> Self
    where
        T: TryInto<u64>,
        T::Error: Debug,
    {
        Self {
            i: i.try_into().unwrap(),
            j: j.try_into().unwrap(),
        }
    }

    pub fn step_left_or_right(mut self, instr: Instr) -> Self {
        self.j = match instr {
            Instr::Left => self.j.saturating_sub(1),
            Instr::Right => self.j.saturating_add(1),
        };
        self
    }

    pub fn step_down(mut self) -> Option<Self> {
        if self.i == 0 {
            return None;
        }

        self.i = self.i.saturating_sub(1);
        Some(self)
    }
}

impl Add for Loc {
    type Output = Loc;

    fn add(mut self, rhs: Loc) -> Loc {
        self.i += rhs.i;
        self.j += rhs.j;
        self
    }
}
