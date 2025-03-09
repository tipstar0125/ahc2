#![allow(non_snake_case)]
#![allow(dead_code)]

use input::Input;

use crate::{common::get_time, input::read_input};

mod bfs;
mod chokudai;
mod chokudai_search;
mod common;
mod coord;
mod input;
mod state;
mod test;

fn solve(input: &Input) {
    let mut chokudai_search = chokudai_search::ChokudaiSearch::new(input, 1, 100);
    chokudai_search.shoot(input);

    let wait_num = input.T - chokudai_search.best_ops.len();
    for op in chokudai_search.best_ops {
        op.output();
    }
    for _ in 0..wait_num {
        println!("-1");
    }
    eprintln!("Elapsed time = {:.3}", get_time());
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
}
