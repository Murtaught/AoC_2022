use lazy_static::lazy_static;
use regex::Regex;
use Instr::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Instr {
    GoForward(usize),
    TurnLeft,
    TurnRight,
}

impl Instr {
    pub fn parse(s: &str) -> Vec<Instr> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(\d+|L|R)").unwrap();
        }

        RE.find_iter(s)
            .map(|m| {
                match m.as_str() {
                    "L" => TurnLeft,
                    "R" => TurnRight,
                    s => GoForward(s.parse().unwrap()),
                }
            })
            .collect()
    }
}
