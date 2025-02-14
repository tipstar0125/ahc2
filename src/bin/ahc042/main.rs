#![allow(non_snake_case)]
#![allow(dead_code)]

use beam::Node;
use input::Input;

use crate::{common::get_time, input::read_input};

mod beam;
mod common;
mod hash;
mod input;
mod state;
mod test;

fn solve(input: &Input) {
    let init_state = state::State::new(input);
    let init_node = Node {
        track_id: !0,
        state: init_state,
    };
    let mut beam = beam::BeamSearch::new(init_node);
    let width = 2000;
    let ops = beam.solve(width, 200, &input, beam::ScoreOrder::Ascending);
    for op in ops.iter() {
        println!("{} {}", op.dir, op.idx);
    }
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
