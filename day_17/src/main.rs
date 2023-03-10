use figure::Figure;
use instr::Instr;
use std::io::*;
use tetris::Tetris;

mod figure;
mod instr;
mod line;
mod loc;
mod pixel;
mod tetris;

fn main() {
    let figures = vec![
        Figure::new("####"),
        Figure::new(".#. ### .#."),
        Figure::new("..# ..# ###"),
        Figure::new("# # # #"),
        Figure::new("## ##"),
    ];

    let instructions: Vec<Instr> = gets().unwrap().bytes().map(Instr::parse).collect();

    let tetris = Tetris {
        figures,
        instructions,
    };

    println!("solve (p1): {}", tetris.simulate(2022));
    println!("solve (p2): {}", tetris.simulate(1000000000000));
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
