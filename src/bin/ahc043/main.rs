#![allow(non_snake_case)]
#![allow(dead_code)]

use input::Input;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{common::get_time, input::read_input};

mod bfs;
mod common;
mod coord;
mod dsu;
mod input;
mod state;
mod test;

const TLE: f64 = 1.0;

fn solve(input: &Input) {
    let mut rng = Pcg64Mcg::new(100);
    let mut best_score = input.K as i64;
    let mut best_state = state::State::new(input);
    while get_time() < TLE {
        let i = rng.gen_range(0..input.N);
        let j = rng.gen_range(0..input.N);
        let start = coord::Coord::new(i, j);
        let mut state = state::State::new(input);
        let score = state.greedy(start, input);
        if score > best_score {
            best_score = score;
            best_state = state;
        }
    }
    best_state.output();
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
