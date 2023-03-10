use std::collections::HashMap;

use crate::line::Line;
use crate::{figure::Figure, instr::Instr, loc::Loc, pixel::Pixel};

pub struct Tetris {
    pub figures: Vec<Figure>,
    pub instructions: Vec<Instr>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Hash)]
struct State {
    fig_index: usize,
    instr_index: usize,
    field: Vec<Line>,
}

impl Tetris {
    const WIDTH: u64 = Line::WIDTH as u64;
    const START_X: u64 = 2;
    const START_Y: u64 = 3;

    pub fn simulate(&self, mut figures_count: usize) -> u64 {
        let cycle_len = self.optimal_step_count();

        let mut ans: u64 = 0;
        let mut state = State::default();
        let mut cache = HashMap::<State, (u64, State)>::new();

        while figures_count > 0 {
            if figures_count >= cycle_len {
                ans += if let Some((increment, next_state)) = cache.get(&state) {
                    state = next_state.clone();
                    *increment
                } else {
                    let initial_state = state.clone();
                    let increment = self.simulate_direct(&mut state, cycle_len);
                    cache.insert(initial_state, (increment, state.clone()));
                    increment
                };

                figures_count -= cycle_len;
            } else {
                ans += self.simulate_direct(&mut state, figures_count);
                figures_count = 0;
            }
        }

        ans
    }

    fn optimal_step_count(&self) -> usize {
        let f = self.figures.len();
        let i = self.instructions.len();

        let lcm = num::integer::lcm(f, i);

        let mut ret = lcm;
        while ret < 100_000 {
            ret += lcm;
        }

        ret
    }

    fn simulate_direct(&self, state: &mut State, figures_count: usize) -> u64 {
        let initial_height = state.height();
        let mut removed_lines = 0_u64;

        for _ in 0..figures_count {
            let figure = &self.figures[state.fig_index];
            state.fig_index = (state.fig_index + 1) % self.figures.len();

            let mut pos = Loc {
                i: state.height() + Self::START_Y,
                j: Self::START_X,
            };

            loop {
                let instr = self.instructions[state.instr_index];
                state.instr_index = (state.instr_index + 1) % self.instructions.len();

                let next = pos.step_left_or_right(instr);

                if state.fits(next, figure) {
                    pos = next;
                }

                match pos.step_down() {
                    Some(next) if state.fits(next, figure) => {
                        pos = next;
                    }
                    _ => break,
                }
            }

            state.petrify(pos, figure);
            state.optimize(figure.n as u64, &mut removed_lines);
        }

        // eprintln!("End of sim.: removed_lines = {removed_lines}, height = {}", state.height());
        removed_lines + state.height() - initial_height
    }
}

impl State {
    fn height(&self) -> u64 {
        self.field.len() as u64
    }

    fn fits(&self, pos: Loc, figure: &Figure) -> bool {
        for i in 0..figure.n {
            for j in 0..figure.m {
                if figure.body[i].get(j) == Pixel::Block {
                    let loc = Loc::new(i, j) + pos;
                    if loc.j >= Tetris::WIDTH || self.contains(&loc) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn contains(&self, loc: &Loc) -> bool {
        self.field
            .get(loc.i as usize)
            .map(|line| line.get(loc.j as usize))
            .unwrap_or(Pixel::Air)
            .into()
    }

    fn set(&mut self, loc: Loc) {
        let i = loc.i as usize;
        if self.field.len() <= i {
            self.field.resize(i + 1, Line::default());
        }

        self.field[i].set(loc.j as usize, Pixel::Block);
    }

    fn petrify(&mut self, pos: Loc, figure: &Figure) {
        for i in 0..figure.n {
            for j in 0..figure.m {
                if figure.body[i].get(j) == Pixel::Block {
                    let loc = Loc::new(i, j) + pos;
                    self.set(loc);
                }
            }
        }
    }

    fn optimize(&mut self, n: u64, removed_lines: &mut u64) {
        let start_i = self.height().saturating_sub(n + 1);
        for i in start_i..self.height() {
            if self.check_line_pair(i as usize) {
                *removed_lines += i + 1;
                self.cut_off(i as usize + 1);
                break;
            }
        }
    }

    fn check_line_pair(&self, i: usize) -> bool {
        let mut line = self.field[i];
        if let Some(other_line) = self.field.get(i + 1) {
            line = line.merge(other_line);
        }
        line.is_full()
    }

    fn cut_off(&mut self, line_count: usize) {
        self.field.drain(0..line_count);
    }
}
