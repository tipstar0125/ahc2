use crate::{
    common::get_time,
    input::Input,
    state::{Op, State},
};

#[derive(Debug, Default)]
pub struct Best {
    pub score: i64,
    pub ops: Vec<Op>,
    pub station_num: usize,
}

pub struct ChokudaiSearch {
    pub depth: usize,
    pub width: usize,
    pub max_size: usize,
    pub beam: Vec<Vec<State>>,
    pub beam_num: usize,
    pub best: Best,
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
            best: Best::default(),
        }
    }
    pub fn apply_cand(&mut self, turn: usize, input: &Input) {
        for _ in 0..self.width {
            if self.beam[turn].is_empty() {
                return;
            }
            let state = self.beam[turn].pop().unwrap();
            if state.turn > self.best.ops.len() && state.score < self.best.score {
                continue;
            }
            if state.score > self.best.score {
                self.best.score = state.score;
                self.best.ops = state.ops.clone();
                self.best.station_num = state.stations.len();
            }
            let cands = state.cand(input);
            for c in cands {
                let new_state = state.apply(c, input);
                if new_state.score > self.best.score {
                    self.best.score = new_state.score;
                    self.best.ops = new_state.ops.clone();
                    self.best.station_num = new_state.stations.len();
                }
                if new_state.turn > self.best.ops.len() && new_state.score < self.best.score {
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
