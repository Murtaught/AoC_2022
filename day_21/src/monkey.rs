use lazy_static::lazy_static;
use num_rational::BigRational;
use num_traits::cast::FromPrimitive;
use regex::Regex;
use Operation::*;

#[derive(Debug, Clone)]
pub struct Monkey {
    pub name: String,
    pub op: Operation,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Constant(BigRational),
    Human,
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
    Equals(String, String),
}

impl Monkey {
    pub fn parse(line: &str) -> Monkey {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(\w+): (?:(\d+)|(\w+) (\+|\-|\*|/) (\w+))$").unwrap();
        }

        let caps = RE.captures(line).unwrap();
        let name = caps[1].to_string();

        let op = if let Some(constant) = caps.get(2) {
            let constant = constant.as_str().parse::<i64>().unwrap();
            Constant(BigRational::from_i64(constant).unwrap())
        } else {
            let name_a = caps[3].to_string();
            let name_b = caps[5].to_string();

            match &caps[4] {
                "+" => Add(name_a, name_b),
                "-" => Subtract(name_a, name_b),
                "*" => Multiply(name_a, name_b),
                "/" => Divide(name_a, name_b),
                _ => unreachable!(),
            }
        };

        Monkey { name, op }
    }

    pub fn children(&self) -> Option<(&str, &str)> {
        match &self.op {
            Add(a, b) => Some((a, b)),
            Subtract(a, b) => Some((a, b)),
            Multiply(a, b) => Some((a, b)),
            Divide(a, b) => Some((a, b)),
            Equals(a, b) => Some((a, b)),
            _ => None,
        }
    }

    pub fn convert_to_equals(&mut self) {
        let mut op = Constant(BigRational::default());
        std::mem::swap(&mut self.op, &mut op);

        let (a, b) = match op {
            Constant(_) => panic!("Can't turn a Constant Monkey into an Equals Monkey."),
            Human => panic!("Can't turn a Human Monkey into an Equals Monkey."),
            Add(a, b) => (a, b),
            Subtract(a, b) => (a, b),
            Multiply(a, b) => (a, b),
            Divide(a, b) => (a, b),
            Equals(a, b) => (a, b),
        };

        self.op = Equals(a, b);
    }

    pub fn convert_to_human(&mut self) {
        self.op = Human;
    }
}
