#![allow(non_snake_case)]
#![allow(dead_code)]

mod common;
mod input;
mod test;

use common::get_time;
use input::{read_input, Input};
use rand_pcg::Pcg64Mcg;

fn solve(input: &Input) {
    let mut rng = Pcg64Mcg::new(0);
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
