use std::io::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instr {
    Add(i64),
    Nop,
}

impl Instr {
    fn parse(s: &str) -> Instr {
        if s == "noop" {
            return Instr::Nop;
        }

        if let Some(s) = s.strip_prefix("addx ") {
            return Instr::Add(s.parse().unwrap());
        }

        panic!("Can't parse {s:?} as Instr!");
    }
}

fn main() {
    let program: Vec<Instr> = stdin().lines().map(|rs| Instr::parse(&rs.unwrap())).collect();

    let mut tick = 0_u64;
    let mut register = 1_i64;

    const INTERESTING_TICKS: &[u64] = &[20, 60, 100, 140, 180, 220];
    let mut ans = 0_i64;

    let mut x = 0_i64;

    let mut on_tick = |tick: u64, reg: i64| {
        // eprintln!("tick: {tick:3}, reg: {reg:3}");
        if INTERESTING_TICKS.contains(&tick) {
            // eprintln!("+ {}", (tick as i64) * reg);
            ans += (tick as i64) * reg;
        }

        if (x - reg).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        x += 1;
        if x == 40 {
            x = 0;
            println!();
        }
    };


    for instr in program {
        tick += 1;
        on_tick(tick, register);

        if let Instr::Add(v) = instr {
            tick += 1;
            on_tick(tick, register);
            register += v;
        }
    }

    println!("{ans}");
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

fn scan<T: std::str::FromStr>() -> T {
    static mut BUFFER: Vec<String> = vec![];
    loop {
        if let Some(token) = unsafe { BUFFER.pop() } {
            return token.parse().ok().unwrap();
        }
        let mut line = String::new();
        stdin().read_line(&mut line).ok();
        unsafe {
            BUFFER = line.split_whitespace().rev().map(String::from).collect();
        }
    }
}
