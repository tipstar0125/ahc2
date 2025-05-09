#![allow(non_snake_case)]
#![allow(dead_code)]

use common::get_time;
use input::{read_input, Input};

mod common;
mod coord;
mod input;
mod rectangle;
mod test;

const TLE: f64 = 1.9;

fn solve(input: &Input) {}

fn main() {
    let is_local: bool = std::env::var("ATCODER").and(Ok(false)).unwrap_or(true);
    get_time();
    let input = read_input(is_local);
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
