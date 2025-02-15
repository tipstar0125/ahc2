#![allow(non_snake_case)]
#![allow(dead_code)]

use input::Input;

use crate::{common::get_time, input::read_input};

mod bfs;
mod common;
mod coord;
mod dsu;
mod input;
mod state;
mod test;

fn solve(input: &Input) {
    for i in 0..input.N {
        for j in 0..input.N {
            let start_station = coord::Coord::new(i, j);
            let mut state = state::State::new(input);
            let score = state.greedy(start_station, input);
        }
    }
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
