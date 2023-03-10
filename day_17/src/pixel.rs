use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pixel {
    Block,
    Air,
}

impl Pixel {
    pub fn parse(c: u8) -> Pixel {
        match c {
            b'#' => Pixel::Block,
            b'.' => Pixel::Air,
            _ => panic!("Unexpected character '{}'!", c as char),
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pixel::Block => '#',
                Pixel::Air => '.',
            }
        )
    }
}

impl From<bool> for Pixel {
    fn from(value: bool) -> Self {
        match value {
            true => Pixel::Block,
            false => Pixel::Air,
        }
    }
}

impl From<Pixel> for bool {
    fn from(pixel: Pixel) -> Self {
        pixel == Pixel::Block
    }
}
