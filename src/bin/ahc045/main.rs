#![allow(non_snake_case)]
#![allow(dead_code)]

use common::get_time;
use estimator::Estimator;
use forest::Forest;
use input::{read_input, Input};

mod common;
mod coord;
mod dsu;
mod estimator;
mod forest;
mod input;
mod rectangle;
mod test;

const TLE: f64 = 1.9;

fn solve(input: &Input) {
    let mut estimator = Estimator::new(input);
    estimator.measure(input);
    estimator.get_ineqs(input);
    estimator.filter_ineqs(input);
    estimator.climbing(input, 1.0);
    let mut forest = Forest::new(input, &estimator);
    forest.greedy(input);
    forest.output();
}

fn main() {
    let is_local: bool = std::env::var("ATCODER").and(Ok(false)).unwrap_or(true);
    get_time();
    let input = read_input(is_local);
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
