use std::collections::{HashMap, VecDeque};

use state::State;

use crate::{
    dir::Dir,
    loc::{IndexedLoc, Loc},
};

mod dir;
mod loc;
mod state;

fn main() {
    let states = get_states("input2");
    assert!(!states.is_empty());
    assert_eq!(states.len(), states[0].period());

    let start = IndexedLoc {
        index: 0,
        loc: Loc { i: -1, j: 0 },
    };

    let n = states[0].height();
    let m = states[0].width();
    let goal = Loc {
        i: n as i16,
        j: m as i16 - 1,
    };

    let p1 = bfs(&states, start, goal);
    println!("ans (p1): {}", p1.steps);

    let p2 = bfs(&states, IndexedLoc { loc: goal, index: p1.index }, start.loc);
    println!("p2 (return to start): {p2:?}");

    let p3 = bfs(&states, IndexedLoc { index: p2.index, ..start }, goal);
    println!("p3 (to goal again): {p3:?}");

    let ans_2 = p1.steps + p2.steps + p3.steps;
    println!("ans (p2): {ans_2}");
}

fn get_states(path: &str) -> Vec<State> {
    let content = std::fs::read_to_string(path).unwrap();
    let mut state = State::parse(&content);
    let mut ret = Vec::with_capacity(state.period());
    loop {
        ret.push(state.clone());
        if !state.advance() {
            break;
        }
    }
    ret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Answer {
    steps: u64,
    index: u32,
}

fn bfs(states: &[State], start: IndexedLoc, goal: Loc) -> Answer {
    let mut dist = HashMap::<IndexedLoc, u64>::new();
    dist.insert(start, 0);

    let mut queue = VecDeque::<IndexedLoc>::new();
    queue.push_back(start);

    while let Some(cur) = queue.pop_front() {
        use Dir::*;
        for action in [None, Some(N), Some(E), Some(S), Some(W)] {
            if let Some(next) = states[cur.index as usize].nudge_pawn(cur, action) {
                if states[next.index as usize].is_empty(next.loc) && !dist.contains_key(&next) {
                    dist.insert(next, dist[&cur] + 1);
                    queue.push_back(next);
                }
            }
        }
    }

    let period = states.len() as u32;
    (0..period)
        .into_iter()
        .filter_map(|index| {
            let loc = IndexedLoc { index, loc: goal };
            dist.get(&loc).copied().map(|d| Answer { steps: d, index })
        })
        .min_by_key(|ans| ans.steps)
        .expect("Failed to find a path through state graph")
}
