use std::{cmp::Ordering, fmt::Display, io::*};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Pac {
    List(Vec<Pac>),
    Int(i32),
}

impl Display for Pac {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pac::Int(value) => write!(f, "{}", value),
            Pac::List(vec) => {
                write!(f, "[")?;
                let mut first = true;
                for sub in vec {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", sub)?;
                    first = false;
                }
                write!(f, "]")
            }
        }
    }
}

impl Pac {
    fn parse(s: &str) -> Pac {
        if let Ok(value) = s.parse() {
            return Pac::Int(value);
        }

        assert!(s.len() >= 2);
        assert_eq!(s.chars().next(), Some('['));
        assert_eq!(s.chars().last(), Some(']'));

        let mut vec = Vec::new();

        let mut balance = 0_usize;
        let mut prev = 1_usize;

        for (i, c) in s.chars().enumerate() {
            match c {
                '[' => balance += 1,
                ']' => {
                    balance -= 1;
                    if balance == 0 && prev < i {
                        vec.push(Pac::parse(&s[prev..i]));
                    }
                }
                ',' => {
                    if balance == 1 {
                        vec.push(Pac::parse(&s[prev..i]));
                        prev = i + 1;
                    }
                }
                _ => {}
            }
        }

        Pac::List(vec)
    }
}

impl PartialOrd for Pac {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Pac::*;
        match self {
            Int(self_val) => match other {
                Int(other_val) => self_val.partial_cmp(other_val),
                List(_) => List(vec![self.clone()]).partial_cmp(other),
            },
            List(self_vec) => match other {
                Int(_) => self.partial_cmp(&List(vec![other.clone()])),
                List(other_vec) => self_vec.partial_cmp(other_vec),
            },
        }
    }
}

impl Ord for Pac {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let mut index = 1_usize;
    let mut ans = 0;

    let dividers = vec![Pac::parse("[[2]]"), Pac::parse("[[6]]")];
    let mut packets = dividers.clone();

    loop {
        let pa = Pac::parse(&gets().unwrap());
        let pb = Pac::parse(&gets().unwrap());
        // eprintln!("pa: {pa}");
        // eprintln!("pb: {pb}");
        // eprintln!("{:?}", pa.partial_cmp(&pb));
        // eprintln!("----------");

        if pa < pb {
            ans += index;
        }

        index += 1;

        packets.push(pa);
        packets.push(pb);

        if gets().is_none() {
            break;
        }
    }

    println!("Answer to the first part: {ans}");

    packets.sort();

    let mut ans_2 = 1;
    for divider in &dividers {
        let pos = packets.iter().position(|x| x == divider).unwrap() + 1;
        ans_2 *= pos;
    }

    println!("Answer to the second part: {ans_2}");
}

fn gets() -> Option<String> {
    let mut line = String::new();
    let count = stdin().read_line(&mut line).unwrap();
    if count > 0 {
        Some(line.trim().to_string())
    } else {
        None
    }
}
