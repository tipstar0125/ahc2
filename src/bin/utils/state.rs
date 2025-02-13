use crate::input::Input;

#[derive(Debug, Clone, Copy)]
pub struct Op {}

#[derive(Debug, Clone)]
pub struct State {}

impl State {
    pub fn new(input: &Input) -> Self {
        Self {}
    }
    pub fn cand(&self, input: &Input) -> Vec<(i64, usize, Op, bool)> {
        let mut cand = vec![];
        cand
    }
    pub fn apply(&mut self, score: i64, hash: usize, op: &Op, _input: &Input) {}
}
