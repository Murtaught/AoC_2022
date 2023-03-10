use std::{fmt::Display, hash::Hash};

use crate::resources::Resources;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    pub have: Resources,
    pub inc: Resources,
    pub time_left: u8,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] ore: {} (+{}), cly: {} (+{}), obs: {} (+{}), gds: {} (+{})",
            self.time_left,
            self.have.a,
            self.inc.a,
            self.have.b,
            self.inc.b,
            self.have.c,
            self.inc.c,
            self.have.d,
            self.inc.d,
        )
    }
}

impl State {
    pub fn new(time_left: u8) -> Self {
        Self {
            have: Resources::default(),
            inc: Resources {
                a: 1,
                ..Resources::default()
            },
            time_left,
        }
    }

    pub fn best_possible_ans(&self) -> u64 {
        let mut ret = self.have.d as u64;
        let mut robots = self.inc.d as u64;
        for _ in 0..self.time_left {
            ret += robots;
            robots += 1;
        }
        ret
    }

    pub fn priority(&self) -> u16 {
        self.have.d
    }

    pub fn idle_ans(&self) -> u64 {
        self.have.d as u64 + self.inc.d as u64 * self.time_left as u64
    }

    pub fn advance(&self, ticks: u8) -> State {
        if ticks > self.time_left {
            panic!("Invalid call to State::tick() at zero time left!");
        }

        let mut next = self.clone();
        next.have += self.inc * ticks;
        next.time_left -= ticks;
        next
    }

    pub fn buy(&self, cost: Resources, dinc: Resources) -> Option<State> {
        let shortage = cost - self.have;
        let ticks = shortage.ticks_needed(&self.inc)? + 1;
        if ticks > self.time_left {
            return None;
        }

        let mut next = self.advance(ticks);

        assert!(next.have.can_afford(&cost));
        next.have -= cost;
        next.inc += dinc;

        Some(next)
    }
}
