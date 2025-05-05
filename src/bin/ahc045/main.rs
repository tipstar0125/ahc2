#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{common::get_time, input::read_input};
use cut::CutTree;
use estimator::Estimator;
use input::Input;

mod common;
mod coord;
mod cut;
mod dsu;
mod estimator;
mod input;
mod rectangle;
mod test;

const TLE: f64 = 1.9;

fn solve(input: &Input) {
    let mut estimator = Estimator::new(&input);
    if input.L <= 4 {
        estimator.triangle_query();
    } else {
        estimator.neighbor_query();
    }
    estimator.get_inequality();
    estimator.climbing(0.5);
    let dist = estimator.gibbs_sampling(1.8);
    let mut cut_tree = CutTree::new(input, &dist, &estimator);
    cut_tree.cut(input);
    cut_tree.make_rest(input, &dist);
    cut_tree.annealing(input, &dist, TLE);
    cut_tree.output(&dist);
}

fn main() {
    let is_local: bool = std::env::var("ATCODER").and(Ok(false)).unwrap_or(true);
    get_time();
    let input = read_input(is_local);
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
