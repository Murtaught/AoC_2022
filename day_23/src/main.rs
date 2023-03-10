use simulation::{Loc, Simulation};

mod simulation;

fn main() {
    let content = std::fs::read_to_string("input2").unwrap();
    let elf_locs: Vec<Loc> = content
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line
                .chars()
                .enumerate()
                .filter_map(move |(j, c)| {
                    match c {
                        '#' => Some(Loc { i: i as i64, j: j as i64 }),
                        _ => None,
                    }
                })
        })
        .collect();

    let mut simulation = Simulation::new(elf_locs);
    println!("ans (p1): {}", solve_p1(simulation.clone()));
    println!("ans (p2): {}", solve_p2(simulation));
}

fn solve_p1(mut simulation: Simulation) -> u64 {
    for _ in 0..10 {
        simulation.step();
    }

    simulation.bound_empty_space()
}

fn solve_p2(mut simulation: Simulation) -> u64 {
    let mut step_no = 0;

    loop {
        step_no += 1;
        let finished = simulation.step();

        if finished { 
            break;
        }
    }

    step_no
}

#[allow(dead_code)]
fn simulate(mut simulation: Simulation) {
    let mut step_no = 0;

    loop {
        eprintln!("After {step_no} steps:");
        eprintln!("{simulation}");

        step_no += 1;
        let finished = simulation.step();

        if finished { 
            break;
        }
    }

    eprintln!("Simulation finished.");
}
