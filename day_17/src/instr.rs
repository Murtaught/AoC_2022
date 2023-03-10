use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instr {
    Left,
    Right,
}

impl Instr {
    pub fn parse(c: u8) -> Instr {
        match c {
            b'<' => Instr::Left,
            b'>' => Instr::Right,
            _ => panic!("Unexpected instruction: '{}'!", c as char),
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Instr::Left => '<',
                Instr::Right => '>',
            }
        )
    }
}
