#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{common::get_time, input::read_input};
use input::Input;

mod common;
mod input;
mod test;

const TLE: f64 = 1.9; // 時間制限

fn solve(input: &Input) {}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
