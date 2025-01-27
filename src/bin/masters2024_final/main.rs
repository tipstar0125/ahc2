#![allow(non_snake_case)]
#![allow(dead_code)]

use input::Input;

use crate::{common::get_time, input::read_input};

mod common;
mod coord;
mod input;
mod vis;

fn solve(input: &Input) -> Output {
    Output {}
}

fn main() {
    get_time();
    let input = read_input();
    let output = solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
    #[cfg(feature = "local")]
    {
        let max_turn = input.max_turn;
        vis::visualizer(input, output, max_turn);
    }
}

#[derive(Debug, Clone)]
pub struct Output {}
