#![allow(non_snake_case)]
#![allow(dead_code)]

mod common;
mod coord;
mod input;
mod state;

use common::get_time;
use input::read_input;
use state::{Direction, State};

fn main() {
    get_time();
    let input = read_input();

    let state = State::new(input.N, input.V, &input.S);

    eprintln!("Elapsed: {}", get_time());
}
