#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::VecDeque;

use input::Input;
use itertools::Itertools;

use crate::{common::get_time, input::read_input};

mod common;
mod input;
mod state;
mod test;

fn solve(input: &Input) {
    let mut state = state::State::new(input.N);
    state.greedy(input);
    state.output();
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
