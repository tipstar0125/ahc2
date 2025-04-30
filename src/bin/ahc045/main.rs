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
    let estimator = Estimator::new(&input).query();

    let mut dist = vec![vec![0.0; input.N]; input.N];
    for i in 0..input.N {
        for j in 0..input.N {
            dist[i][j] = estimator.dist[i][j] as f64;
            dist[j][i] = dist[i][j];
        }
    }
    let mut cut_tree = CutTree::new(input, &dist);
    cut_tree.cut(input);
    cut_tree.make_rest(input, &dist);
    cut_tree.output(&dist);
}

fn main() {
    let is_local: bool = std::env::var("ATCODER").and(Ok(false)).unwrap_or(true);
    get_time();
    let input = read_input(is_local);
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
