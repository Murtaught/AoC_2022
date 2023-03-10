use std::{collections::HashSet, io::*};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
struct Loc {
    i: i32,
    j: i32,
}

impl Loc {
    fn is_adjacent(self, o: Loc) -> bool {
        (self.i - o.i).abs() <= 1 && (self.j - o.j).abs() <= 1
    }
}

fn main() {
    const ROPE_LEN: usize = 10;
    let mut visited = HashSet::<Loc>::new();
    let mut rope = vec![Loc::default(); ROPE_LEN];

    while let Some(line) = gets() {
        let line: Vec<_> = line.split_whitespace().collect();
        let direction: char = line[0].parse().unwrap();
        let step_count: usize = line[1].parse().unwrap();
        // eprintln!("> {direction:?} {step_count}");

        let (dj, di) = match direction {
            'R' => (1, 0),
            'U' => (0, 1),
            'L' => (-1, 0),
            'D' => (0, -1),
            _ => panic!("Unexpected direction {direction:?}!"),
        };

        for _ in 0..step_count {
            rope[0].i += di;
            rope[0].j += dj;

            for k in 1..ROPE_LEN {
                rope[k] = move_knot(rope[k], rope[k - 1]);
            }

            visited.insert(rope[ROPE_LEN - 1]);

            // print_state(&rope);
        }
    }

    println!("{}", visited.len());
}

fn move_knot(tail: Loc, head: Loc) -> Loc {
    if tail.is_adjacent(head) {
        // Nothing needs to be done.
        return tail;
    }

    let di = head.i - tail.i;
    let dj = head.j - tail.j;

    if di == 0 {
        assert_eq!(dj.abs(), 2);
        return Loc { j: tail.j + dj / 2, ..tail };
    }

    if dj == 0 {
        assert_eq!(di.abs(), 2);
        return Loc { i: tail.i + di / 2, ..tail };
    }

    let diag = Loc { i: tail.i + 1, j: tail.j + 1 };
    if diag.is_adjacent(head) {
        return diag;
    }

    let diag = Loc { i: tail.i + 1, j: tail.j - 1 };
    if diag.is_adjacent(head) {
        return diag;
    }

    let diag = Loc { i: tail.i - 1, j: tail.j + 1 };
    if diag.is_adjacent(head) {
        return diag;
    }

    let diag = Loc { i: tail.i - 1, j: tail.j - 1 };
    if diag.is_adjacent(head) {
        return diag;
    }

    panic!("Failed to move knot!");
}

fn print_state(rope: &[Loc]) {
    for i in (-5..=15).into_iter().rev() {
        for j in -11..=14 {
            let loc = Loc { i, j };

            if let Some(k) = rope.iter().position(|&el| el == loc) {
                if k == 0 {
                    eprint!("H");
                } else {
                    eprint!("{}", k);
                }
            } else if i == 0 && j == 0 {
                eprint!("s");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }
    eprintln!();
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
