use std::{
    collections::{HashSet, VecDeque},
    io,
};

use point::Point3;
use world::World;

mod point;
mod world;

fn main() {
    let points: HashSet<Point3> = io::stdin()
        .lines()
        .map(|line| Point3::parse(&line.unwrap()))
        .collect();

    println!("Answer (p1): {}", solve_p1(&points));
    println!("Answer (p2): {}", solve_p2(&points));
}

fn solve_p1(points: &HashSet<Point3>) -> usize {
    let mut ans = 0;
    for p in points {
        ans += p
            .neighbors()
            .filter(|n| !points.contains(n))
            .count();
    }
    ans
}

fn solve_p2(points: &HashSet<Point3>) -> usize {
    let world = World::encase(points);
    let outside = bfs(points, &world, world.top_left());
    
    let mut ans = 0;
    for p in points {
        ans += p
            .neighbors()
            .filter(|n| outside.contains(n))
            .count();
    }
    ans
}

fn bfs(points: &HashSet<Point3>, world: &World, start: Point3) -> HashSet<Point3> {
    let mut component = HashSet::new();
    component.insert(start);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(cur) = queue.pop_front() {
        for p in cur.neighbors() {
            if world.contains(&p) && !points.contains(&p) && !component.contains(&p) {
                component.insert(p);
                queue.push_back(p);
            }
        }
    }

    component
}
