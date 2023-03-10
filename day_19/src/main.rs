use blueprint::Blueprint;
use priority_queue::PriorityQueue;
use state::State;

mod blueprint;
mod resources;
mod state;

fn main() {
    let content = std::fs::read_to_string("input").unwrap();
    let blueprints: Vec<_> = content.lines().map(Blueprint::parse).collect();

    println!("Answer (p1): {}", solve_p1(&blueprints));
    println!("Answer (p2): {}", solve_p2(&blueprints));
}

#[allow(dead_code)]
fn solve_p1(blueprints: &[Blueprint]) -> u64 {
    const TIME_LEFT: u8 = 24; // minutes

    let mut ans = 0;
    for blueprint in blueprints {
        let geodes = solve(blueprint, TIME_LEFT);
        ans += geodes * blueprint.number as u64;
    }
    ans
}

#[allow(dead_code)]
fn solve_p2(blueprints: &[Blueprint]) -> u64 {
    const TIME_LEFT: u8 = 32; // minutes

    assert!(blueprints.len() >= 3);
    let blueprints = &blueprints[0..3];

    let mut ans = 1;
    for blueprint in blueprints {
        let geodes = solve(blueprint, TIME_LEFT);
        ans *= geodes;
    }
    ans
}

fn solve(bp: &Blueprint, time_left: u8) -> u64 {
    let mut queue = PriorityQueue::<State, u16>::new();
    queue.push(State::new(time_left), 0);

    let mut ans = 0;
    while let Some((cur, _)) = queue.pop() {
        if cur.best_possible_ans() > ans {
            if cur.inc.a < bp.max_a {
                if let Some(next) = cur.buy(bp.ar, Blueprint::A_INC) {
                    let p = next.priority();
                    queue.push(next, p);
                }
            }

            if cur.inc.b < bp.max_b {
                if let Some(next) = cur.buy(bp.br, Blueprint::B_INC) {
                    let p = next.priority();
                    queue.push(next, p);
                }
            }

            if cur.inc.c < bp.max_c {
                if let Some(next) = cur.buy(bp.cr, Blueprint::C_INC) {
                    let p = next.priority();
                    queue.push(next, p);
                }
            }

            if let Some(next) = cur.buy(bp.dr, Blueprint::D_INC) {
                let p = next.priority();
                queue.push(next, p);
            }

            ans = ans.max(cur.idle_ans());
        }
    }
    ans
}
