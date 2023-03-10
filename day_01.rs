use std::io;

fn main() {
    let content = io::read_to_string(io::stdin()).unwrap();
    let mut elves: Vec<_> = content
        .split("\n\n")
        .map(|block| block.lines().map(parse_i64).sum::<i64>())
        .collect();

    elves.sort_by_key(|elf| -elf);
    assert!(elves.len() >= 3);

    println!("ans (p1): {}", elves[0]);
    println!("ans (p2): {}", elves[0] + elves[1] + elves[2]);
}

fn parse_i64(s: &str) -> i64 {
    s.parse().unwrap()
}
