use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::Display,
    io::*,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Air,
    Wall,
    Sand,
    Spawner,
}

use Cell::*;

impl Cell {
    fn symbol(&self) -> char {
        match self {
            Air => '.',
            Wall => '#',
            Sand => 'o',
            Spawner => '+',
        }
    }

    fn is_blocking(&self) -> bool {
        matches!(self, Wall | Sand)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Loc {
    x: i32,
    y: i32,
}

impl Loc {
    fn parse(s: &str) -> Loc {
        let mut it = s.split(',').map(|s| s.trim());
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        assert_eq!(it.next(), None);
        Loc { x, y }
    }
}

type World = HashMap<Loc, Cell>;

#[allow(dead_code)]
fn show_world(world: &World) {
    let x_min = world.keys().map(|k| k.x).min().unwrap();
    let x_max = world.keys().map(|k| k.x).max().unwrap();
    let y_min = world.keys().map(|k| k.y).min().unwrap();
    let y_max = world.keys().map(|k| k.y).max().unwrap();

    eprintln!("X from {x_min} to {x_max};");
    eprintln!("Y from {y_min} to {y_max};");

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let cell = world.get(&Loc { x, y }).unwrap_or(&Air);
            eprint!("{}", cell);
        }
        eprintln!();
    }
}

const SPAWNER_LOC: Loc = Loc { x: 500, y: 0 };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SimulationResult {
    Stabilized,
    FallenOff,
    BlockedSpawner,
}

fn simulate_sand_fall(world: &mut World) -> SimulationResult {
    // Grossly inefficient, but I don't care.
    let wall_y_max = world
        .iter()
        .filter_map(|(loc, &cell)| if cell == Wall { Some(loc.y) } else { None })
        .max()
        .unwrap();

    let mut cur = SPAWNER_LOC;

    if world.get(&cur).unwrap_or(&Air) == &Sand {
        return SimulationResult::BlockedSpawner;
    }

    let try_cell = |world: &World, cur: Loc, dx: i32, dy: i32| {
        let next = Loc {
            x: cur.x + dx,
            y: cur.y + dy,
        };
        let next_cell = world.get(&next).unwrap_or(&Air);
        if !next_cell.is_blocking() {
            Some(next)
        } else {
            None
        }
    };

    while cur.y <= wall_y_max {
        if let Some(next) = try_cell(world, cur, 0, 1) {
            cur = next;
            continue;
        }

        if let Some(next) = try_cell(world, cur, -1, 1) {
            cur = next;
            continue;
        }

        if let Some(next) = try_cell(world, cur, 1, 1) {
            cur = next;
            continue;
        }

        world.insert(cur, Sand);
        return SimulationResult::Stabilized;
    }

    SimulationResult::FallenOff
}

fn main() {
    let mut world = World::new();
    world.insert(SPAWNER_LOC, Spawner);

    while let Some(line) = gets() {
        // eprintln!("Processing {line:?} ...");
        let points: Vec<Loc> = line.split(" -> ").map(Loc::parse).collect();

        for (&pa, &pb) in points.iter().zip(points.iter().skip(1)) {
            // eprintln!("{pa:?} --> {pb:?}");

            if pa.x == pb.x {
                let x = pa.x;
                let y_min = min(pa.y, pb.y);
                let y_max = max(pa.y, pb.y);
                for y in y_min..=y_max {
                    world.insert(Loc { x, y }, Wall);
                }
            } else {
                assert_eq!(pa.y, pb.y);
                let y = pa.y;
                let x_min = min(pa.x, pb.x);
                let x_max = max(pa.x, pb.x);
                for x in x_min..=x_max {
                    world.insert(Loc { x, y }, Wall);
                }
            }
        }
    }

    // show_world(&world);

    let mut ans = 0_usize;

    while simulate_sand_fall(&mut world) != SimulationResult::FallenOff {
        ans += 1;
        // show_world(&world);
    }

    println!("Answer to the first part: {ans}");

    world = world
        .into_iter()
        .filter(|(_, cell)| cell == &Wall || cell == &Spawner)
        .collect();

    let floor_y = world
        .iter()
        .filter_map(|(loc, &cell)| if cell == Wall { Some(loc.y) } else { None })
        .max()
        .unwrap()
        + 2;

    for x in -10_000..=10_000 {
        world.insert(Loc { x, y: floor_y }, Wall);
    }

    eprintln!("Made \"infinite\" floor. Starting another set of simulations.");
    eprintln!("Be patient. It may take a few minutes of wall clock time.");

    ans = 0;
    while simulate_sand_fall(&mut world) == SimulationResult::Stabilized {
        ans += 1;
    }

    if simulate_sand_fall(&mut world) == SimulationResult::FallenOff {
        println!("Error! Floor is not long enough! :o(");
    } else {
        println!("Answer to the second part: {ans}");
    }
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
