use crate::{
    common::get_time,
    input::Input,
    state::{Op, State},
};

pub struct ChokudaiSearch {
    pub depth: usize,
    pub width: usize,
    pub max_size: usize,
    pub beam: Vec<Vec<State>>,
    pub beam_num: usize,
    pub best_score: i64,
    pub best_ops: Vec<Op>,
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
            best_score: 0,
            best_ops: vec![],
        }
    }
    pub fn apply_cand(&mut self, turn: usize, input: &Input) {
        for _ in 0..self.width {
            if self.beam[turn].is_empty() {
                return;
            }
            let state = self.beam[turn].pop().unwrap();
            if state.turn > self.best_ops.len() && state.score <= self.best_score {
                continue;
            }
            if state.score > self.best_score {
                self.best_score = state.score;
                self.best_ops = state.ops.clone();
            }
            let cands = state.cand(input);
            for c in cands {
                let new_state = state.apply(c, input);
                if new_state.score > self.best_score {
                    self.best_score = new_state.score;
                    self.best_ops = new_state.ops.clone();
                }
                if new_state.score <= state.score {}
                if new_state.turn > self.best_ops.len() && new_state.score <= self.best_score {
                    continue;
                }
                self.beam[new_state.turn].push(new_state);
            }
        }
    }
    pub fn shoot(&mut self, input: &Input) {
        'a: loop {
            self.beam_num += 1;
            for turn in 0..self.depth {
                eprintln!("turn = {}", turn);
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
        eprintln!("Score = {}", self.best_score);
    }
}
