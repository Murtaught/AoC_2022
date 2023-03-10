use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, io::*};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Loc {
    x: i64,
    y: i64,
}

impl Loc {
    fn dist(&self, o: &Loc) -> i64 {
        (self.x - o.x).abs() + (self.y - o.y).abs()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Sensor {
    loc: Loc,
    radius: i64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Line {
    l: i64,
    r: i64,
}

impl Sensor {
    fn from_two_points(loc: Loc, beacon: &Loc) -> Sensor {
        let radius = loc.dist(beacon);
        Sensor { loc, radius }
    }

    fn contains_y(&self, y: i64) -> bool {
        (self.loc.y - y).abs() <= self.radius
    }

    fn line_at(&self, y: i64) -> Line {
        let radius = self.radius - (self.loc.y - y).abs();
        assert!(radius >= 0);

        Line {
            l: self.loc.x - radius,
            r: self.loc.x + radius,
        }
    }
}

fn main() {
    let mut sensors = Vec::new();
    let mut blocked = HashSet::new();

    while let Some(line) = gets() {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"
            )
            .unwrap();
        }

        let caps = RE.captures(&line).unwrap();
        let sensor = Loc {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
        };
        let beacon = Loc {
            x: caps[3].parse().unwrap(),
            y: caps[4].parse().unwrap(),
        };
        // eprintln!("S: {sensor:?}, B: {beacon:?}");

        blocked.insert(sensor.clone());
        sensors.push(Sensor::from_two_points(sensor, &beacon));
        blocked.insert(beacon);
    }

    // eprintln!("Sensors: {sensors:#?}");

    const LINE_Y: i64 = 2000000;

    let min_x = sensors.iter().map(|s| s.loc.x - s.radius).min().unwrap();
    let max_x = sensors.iter().map(|s| s.loc.x + s.radius).max().unwrap();
    // eprintln!("min_x: {min_x}");
    // eprintln!("max_x: {max_x}");

    let mut ans = 0_usize;
    for x in min_x..=max_x {
        let cur = Loc { x, y: LINE_Y };

        if blocked.contains(&cur) {
            continue;
        }

        if sensors.iter().any(|s| s.loc.dist(&cur) <= s.radius) {
            ans += 1;
        }
    }

    println!("Answer to the first part: {ans}");

    const MAX_COORD: i64 = 4_000_000;

    // Let's visualize this stuff:
    // for y in 0..21 {
    //     for x in 0..21 {
    //         let cur = Loc { x, y };
    //
    //         if blocked.contains(&cur) {
    //             eprint!("B");
    //             continue;
    //         }
    //
    //         if sensors.iter().any(|s| s.loc.dist(&cur) <= s.radius) {
    //             eprint!("#");
    //         } else {
    //             eprint!(".");
    //         }
    //     }
    //     eprintln!();
    // }

    for y in 0..MAX_COORD {
        // eprintln!("y = {y} ...");

        let mut lines: Vec<Line> = sensors
            .iter()
            .filter(|s| s.contains_y(y))
            .filter_map(|s| {
                let mut line = s.line_at(y);
                line.l = line.l.max(0);
                if line.r >= 0 {
                    Some(line)
                } else {
                    None
                }
            })
            .collect();

        for loc in &blocked {
            if loc.y == y {
                lines.push(Line { l: loc.x, r: loc.x });
            }
        }

        lines.sort_by_key(|line| line.l);

        // eprintln!("lines: {lines:?}");

        let mut front = 0;
        for line in lines {
            if line.l > front {
                let x = front;
                if !(0..=MAX_COORD).contains(&x) {
                    continue;
                }

                // println!("Found a gap! line = {line:?}, front = {front}");
                println!("Found it! x = {x}, y = {y}");
                println!("Answer to the second part: {}", x * 4_000_000 + y);
                break;
            }

            front = front.max(line.r + 1);
        }
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
