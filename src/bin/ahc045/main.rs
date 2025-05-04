#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{common::get_time, input::read_input};
use common::eprint_yellow;
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
    if input.L == 3 {
        estimator.three_node_query();
    } else {
        estimator.neighbor_query();
    }
    estimator.get_inequality();
    eprint_yellow(format!("estimator init elapsed = {:.3}", get_time()).as_str());
    estimator.climbing(0.5);
    eprint_yellow(format!("climbing elapsed = {:.3}", get_time()).as_str());
    let dist = estimator.gibbs_sampling(TLE);
    eprint_yellow(format!("gibbs sampling elapsed = {:.3}", get_time()).as_str());
    let mut cut_tree = CutTree::new(input, &dist);
    cut_tree.cut(input);
    eprint_yellow(format!("cut elapsed = {:.3}", get_time()).as_str());
    cut_tree.make_rest(input, &dist);
    eprint_yellow(format!("make rest elapsed = {:.3}", get_time()).as_str());
    cut_tree.annealing(input, &dist, TLE);
    eprint_yellow(format!("annealing elapsed = {:.3}", get_time()).as_str());
    cut_tree.output(&dist);
}

fn main() {
    let is_local: bool = std::env::var("ATCODER").and(Ok(false)).unwrap_or(true);
    get_time();
    let input = read_input(is_local);
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
