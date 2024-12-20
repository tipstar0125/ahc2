#![allow(non_snake_case)]
#![allow(dead_code)]

mod beam;
mod common;
mod hash;
mod input;
mod measure;
mod state;
mod test;

use beam::{BeamSearch, Node};
use common::get_time;
use input::{read_input, Input};
use rand_pcg::Pcg64Mcg;
use state::State;

fn solve(input: &Input) {
    let mut rng = Pcg64Mcg::new(0);
    let init_state = State::new(input);
    let init_node = Node {
        track_id: !0,
        state: init_state,
    };
    let mut beam = BeamSearch::new(init_node);
    let width = if input.N <= 70 { 15000 } else { 10000 };
    beam.solve(width, input.N, &input, &mut rng, true);
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
