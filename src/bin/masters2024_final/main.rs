#![allow(non_snake_case)]
#![allow(dead_code)]

use coord::Coord;
use input::Input;
use proconio::input_interactive;

use crate::{common::get_time, input::read_input};

mod common;
mod coord;
mod input;
mod state;
mod vis;

fn solve(input: &Input) -> Output {
    let mut output = Output {
        actual_position: vec![],
        actual_velocity: vec![],
    };

    #[cfg(feature = "local")]
    {
        input_interactive! {
            p: (i64, i64),
            v: (i64, i64),
        }
        output.actual_position.push(Coord { x: p.0, y: p.1 });
        output.actual_velocity.push(Coord { x: v.0, y: v.1 });
        eprintln!("p = {:?}, v = {:?}", p, v);
    }

    // input_interactive! {
    //     c: usize,
    //     h: usize,
    //     q: [usize; h]
    // }
    // #[cfg(feature = "local")]
    // input_interactive! {
    //     p: (f32, f32),
    //     v: (f32, f32),
    // }

    output
}

fn main() {
    get_time();
    let input = read_input();

    let _output = solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
    #[cfg(feature = "local")]
    {
        let max_turn = input.max_turn;
        vis::visualizer(input, _output, max_turn);
    }
}

#[derive(Debug, Clone)]
pub struct Output {
    actual_position: Vec<Coord>,
    actual_velocity: Vec<Coord>,
}
