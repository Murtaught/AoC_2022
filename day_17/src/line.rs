use std::fmt::Display;

use crate::pixel::Pixel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Line(u8);

impl Line {
    pub const WIDTH: usize = 7;
    const FULL_MASK: u8 = (1 << Self::WIDTH) - 1;

    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, i: usize) -> Pixel {
        let bit = ((self.0 >> i) & 1) == 1;
        bit.into()
    }

    pub fn set(&mut self, i: usize, value: Pixel) {
        self.0 = match value {
            Pixel::Block => self.0 | (1 << i),
            Pixel::Air => self.0 & !(1 << i),
        };
    }

    pub fn merge(mut self, o: &Line) -> Line {
        self.0 |= o.0;
        self
    }

    pub fn is_full(&self) -> bool {
        self.0 == Self::FULL_MASK
    }
}

impl From<u8> for Line {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|")?;
        for i in 0..Line::WIDTH {
            write!(f, "{}", self.get(i))?;
        }
        writeln!(f, "|")
    }
}
