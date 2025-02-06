#![allow(non_snake_case)]
#![allow(dead_code)]

use input::Input;

use crate::{common::get_time, input::read_input};

mod common;
mod input;
mod state;
mod test;

fn solve(input: &Input) {
    let mut state = state::State::new(input);
    state.greedy();
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
