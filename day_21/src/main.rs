use num_rational::BigRational;
use num_traits::FromPrimitive;

use crate::monkey::{Monkey, Operation::*};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::process::Command;

mod monkey;

fn main() {
    let content = std::fs::read_to_string("input2").unwrap();
    let monkeys: HashMap<String, Monkey> = content
        .lines()
        .map(Monkey::parse)
        .map(|m| (m.name.clone(), m))
        .collect();

    println!("ans (p1): {}", solve_p1(monkeys.clone()));
    println!("ans (p2): {}", solve_p2(monkeys));
}

fn solve_p1(monkeys: HashMap<String, Monkey>) -> BigRational {
    let mut dfs = Dfs { monkeys };
    dfs.optimize("root").unwrap()
}

fn solve_p2(mut monkeys: HashMap<String, Monkey>) -> BigRational {
    monkeys.get_mut("root").unwrap().convert_to_equals();
    monkeys.get_mut("humn").unwrap().convert_to_human();

    let mut dfs = Dfs { monkeys };

    let mut original = dfs.clone();

    let _ = dfs.optimize("root");
    dfs.remove_unused();

    let mut _i = 0;
    // dfs.write_file(&format!("graphs/{_i:03}.dot"));

    let ans = loop {
        if let Some(ans) = dfs.unwrap_equals() {
            break ans;
        }

        _i += 1;
        dfs.remove_unused();
        // dfs.write_file(&format!("graphs/{_i:03}.dot"));
    };

    // Let's check ourselves:
    original.monkeys.get_mut("humn").unwrap().op = Constant(ans.clone());
    assert_eq!(
        original.optimize("root"),
        Some(BigRational::from_i64(1).unwrap())
    );

    ans
}

#[derive(Clone)]
struct Dfs {
    monkeys: HashMap<String, Monkey>,
}

impl Dfs {
    pub fn optimize(&mut self, name: &str) -> Option<BigRational> {
        match self.monkeys.get(name).unwrap().op.clone() {
            Human => None,
            Constant(c) => Some(c),

            Add(a, b) => self.do_two(&a, &b, |a, b| a + b).map(|x| {
                self.monkeys.get_mut(name).unwrap().op = Constant(x.clone());
                x
            }),

            Subtract(a, b) => self.do_two(&a, &b, |a, b| a - b).map(|x| {
                self.monkeys.get_mut(name).unwrap().op = Constant(x.clone());
                x
            }),

            Multiply(a, b) => self.do_two(&a, &b, |a, b| a * b).map(|x| {
                self.monkeys.get_mut(name).unwrap().op = Constant(x.clone());
                x
            }),

            Divide(a, b) => self.do_two(&a, &b, |a, b| a / b).map(|x| {
                self.monkeys.get_mut(name).unwrap().op = Constant(x.clone());
                x
            }),

            Equals(a, b) => self.do_two(&a, &b, |a, b| {
                BigRational::from_i64((a == b) as i64).unwrap()
            }),
        }
    }

    fn do_two(
        &mut self,
        a: &str,
        b: &str,
        f: fn(a: BigRational, b: BigRational) -> BigRational,
    ) -> Option<BigRational> {
        let a = self.optimize(a);
        let b = self.optimize(b);
        if let Some(a) = a {
            if let Some(b) = b {
                return Some(f(a, b));
            }
        }
        None
    }

    pub fn remove_unused(&mut self) {
        let reach = self.reachable("root");
        self.monkeys = std::mem::take(&mut self.monkeys)
            .into_iter()
            .filter(|(name, _)| reach.contains(name))
            .collect();
    }

    pub fn reachable(&self, name: &str) -> HashSet<String> {
        let monkey = self.monkeys.get(name).unwrap();
        let mut ret = if let Some((a, b)) = monkey.children() {
            let mut ret = self.reachable(a);
            for s in self.reachable(b) {
                ret.insert(s);
            }
            ret
        } else {
            HashSet::new()
        };
        ret.insert(name.to_string());
        ret
    }

    pub fn unwrap_equals(&mut self) -> Option<BigRational> {
        if let Equals(a, b) = self.monkeys.get("root").unwrap().op.clone() {
            let (c1, b, _) = self.sorted_children(&a, &b);

            match &b.op {
                Constant(_) => panic!("Static Equals?"),
                Human => return Some(c1),

                Add(c, d) => {
                    let (c2, d, _) = self.sorted_children(c, d);
                    self.monkeys.get_mut(&b.name).unwrap().op = Constant(c1 - c2);
                    self.monkeys.get_mut("root").unwrap().op = Equals(b.name, d.name);
                }

                Subtract(c, d) => {
                    let (c2, d, right) = self.sorted_children(c, d);
                    self.monkeys.get_mut(&b.name).unwrap().op = Constant(if right {
                        // d - c2 = c1
                        c1 + c2
                    } else {
                        // c2 - d = c1
                        c2 - c1
                    });
                    self.monkeys.get_mut("root").unwrap().op = Equals(b.name, d.name);
                }

                Multiply(c, d) => {
                    let (c2, d, _) = self.sorted_children(c, d);
                    self.monkeys.get_mut(&b.name).unwrap().op = Constant(c1 / c2);
                    self.monkeys.get_mut("root").unwrap().op = Equals(b.name, d.name);
                }

                Divide(c, d) => {
                    let (c2, d, right) = self.sorted_children(c, d);
                    self.monkeys.get_mut(&b.name).unwrap().op = Constant(if right {
                        // d / c2 = c1
                        c1 * c2
                    } else {
                        // c2 / d = c1 | * d
                        // c2 = c1 * d
                        // d = c2 / c1
                        c2 / c1
                    });
                    self.monkeys.get_mut("root").unwrap().op = Equals(b.name, d.name);
                }

                Equals(_, _) => unreachable!(),
            }

            None
        } else {
            panic!("Root node should always be an Equals Monkey.");
        }
    }

    fn sorted_children(&self, a: &str, b: &str) -> (BigRational, Monkey, bool) {
        let mut a = self.monkeys.get(a).unwrap();
        let mut b = self.monkeys.get(b).unwrap();
        let mut right = false;
        if !matches!(a.op, Constant(_)) {
            right = true;
            std::mem::swap(&mut a, &mut b);
        }

        let c = match &a.op {
            Constant(c) => c.clone(),
            _ => panic!("sorted_children: constant expected"),
        };

        (c, b.clone(), right)
    }

    #[allow(dead_code)]
    pub fn write_file(&self, file_path: &str) {
        {
            let mut file = std::fs::File::create(file_path).unwrap();
            self.write_dot(&mut file).unwrap();
        }

        Command::new("dot")
            .arg("-Tpng")
            .arg("-O")
            .arg(file_path)
            .output()
            .expect("Failed to run \"dot\".");
    }

    #[allow(dead_code)]
    pub fn write_dot(&self, f: &mut impl Write) -> std::io::Result<()> {
        writeln!(f, "digraph G {{")?;
        // writeln!(f, "    rankdir=\"LR\";")?;

        for (name, monkey) in &self.monkeys {
            write!(f, "    {name} [label=\"")?;
            match &monkey.op {
                Constant(c) => write!(f, "{c}")?,
                Human => write!(f, "Human")?,
                Add(_, _) => write!(f, "+")?,
                Subtract(_, _) => write!(f, "-")?,
                Multiply(_, _) => write!(f, "*")?,
                Divide(_, _) => write!(f, "/")?,
                Equals(_, _) => write!(f, "==")?,
            }
            writeln!(f, "\"];")?;

            if let Some((a, b)) = monkey.children() {
                writeln!(f, "    {name} -> {a}")?;
                writeln!(f, "    {name} -> {b}")?;
            }
        }

        writeln!(f, "}}")
    }
}
