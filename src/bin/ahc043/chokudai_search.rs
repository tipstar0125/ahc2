use crate::{
    common::get_time,
    input::Input,
    state::{Cand, State},
};

pub struct ChokudaiSearch {
    pub depth: usize,
    pub width: usize,
    pub max_size: usize,
    pub beam: Vec<Vec<State>>,
    pub beam_num: usize,
    pub best: Cand,
}

impl ChokudaiSearch {
    pub fn new(input: &Input, width: usize, max_size: usize) -> Self {
        let mut beam = vec![vec![]; input.T + 1];
        let initial_state = State::new(input);
        beam[0].push(initial_state);

        Self {
            depth: input.T,
            width,
            max_size,
            beam,
            beam_num: 0,
            best: Cand::default(),
        }
    }
    pub fn apply_cand(&mut self, turn: usize, input: &Input) {
        for _ in 0..self.width {
            if self.beam[turn].is_empty() {
                return;
            }
            let state = self.beam[turn].pop().unwrap();
            let cands = state.cand(input);
            for c in cands {
                if state.turn + c.ops.len() > self.best.ops.len() && c.score < self.best.score {
                    continue;
                }
                let new_state = state.apply(c.ops, input);
                if new_state.score > self.best.score {
                    self.best.score = new_state.score;
                    self.best.ops = new_state.ops.clone();
                }
                self.beam[new_state.turn].push(new_state);
            }
        }
    }
    pub fn shoot(&mut self, input: &Input) {
        'a: loop {
            self.beam_num += 1;
            for turn in 0..self.depth {
                self.beam[turn].sort_unstable_by_key(|state| -state.score);
                self.beam[turn].truncate(self.max_size);
                self.beam[turn].reverse();
                self.apply_cand(turn, input);
                if get_time() > input.TLE {
                    break 'a;
                }
            }
        }

        eprintln!("beam_num = {}", self.beam_num);
        eprintln!("Score = {}", self.best.score);
    }
}
