use map::Map;

use crate::{instr::Instr, jump_table::JumpTable};

mod instr;
mod jump_table;
mod map;
mod pawn;
mod zones;

fn main() {
    let content = std::fs::read_to_string("input2").unwrap();
    let map: Map = content
        .lines()
        .take_while(|line| !line.is_empty())
        .collect();

    let instrs = Instr::parse(
        content
            .lines()
            .skip_while(|line| !line.is_empty())
            .nth(1)
            .unwrap(),
    );

    // eprintln!("{map}");
    // eprintln!("{instrs:?}");
    // eprintln!("pawn: {pawn:?}");

    println!("{}", solve_p1(&map, &instrs));
    println!("{}", solve_p2(map, &instrs));
}

fn solve_p1(map: &Map, instrs: &[Instr]) -> i64 {
    map.walk(instrs).password()
}

fn solve_p2(mut map: Map, instrs: &[Instr]) -> i64 {
    // eprintln!("Zones diagram:\n{}", zones::zones_diagram(&map));
    map.set_jump_table(if map.ss == 4 {
        jump_table_1()
    } else {
        jump_table_2()
    });
    map.walk(instrs).password()
}

#[allow(dead_code)]
fn jump_table_1() -> JumpTable {
    use pawn::Direction::*;
    let mut t = JumpTable::new();
    t.insert(2, [(11, W), (6, S), (5, S), (4, S)]);
    t.insert(4, [(5, E), (10, N), (11, N), (2, S)]);
    t.insert(5, [(6, E), (10, E), (4, W), (2, E)]);
    t.insert(6, [(11, S), (10, S), (5, W), (2, N)]);
    t.insert(10, [(11, E), (4, N), (5, N), (6, N)]);
    t.insert(11, [(2, W), (4, E), (10, W), (6, W)]);
    t
}

#[allow(dead_code)]
fn jump_table_2() -> JumpTable {
    use pawn::Direction::*;
    let mut t = JumpTable::new();
    t.insert(1, [(2, E), (4, S), (6, E), (9, E)]);
    t.insert(2, [(7, W), (4, W), (1, W), (9, N)]);
    t.insert(4, [(2, N), (7, S), (6, S), (1, N)]);
    t.insert(6, [(7, E), (9, S), (1, E), (4, E)]);
    t.insert(7, [(2, W), (9, W), (6, W), (4, N)]);
    t.insert(9, [(7, N), (2, S), (1, S), (6, N)]);
    t
}
