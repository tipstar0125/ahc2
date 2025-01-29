#![allow(non_snake_case)]
#![allow(dead_code)]

use coord::Coord;
use estimator::Particle;
use input::Input;
use proconio::input_interactive;
use rand_pcg::Pcg64Mcg;

use crate::{common::get_time, input::read_input};

mod common;
mod coord;
mod estimator;
mod input;
mod normal;
mod vis;

fn solve(input: &Input) -> Output {
    let mut output = Output {
        actual_position: vec![],
        actual_velocity: vec![],
        particle: vec![],
    };

    #[cfg(feature = "local")]
    {
        input_interactive! {
            p: (i64, i64),
            v: (i64, i64),
        }
        output.actual_position.push(Coord { x: p.0, y: p.1 });
        output.actual_velocity.push(Coord { x: v.0, y: v.1 });
        // eprintln!("p = {:?}, v = {:?}", p, v);
    }

    let mut estimator = estimator::Estimator::new(input, 2000);
    output.particle.push(estimator.particles.clone());

    for t in 0..input.max_turn {
        let particles = estimator.action(input);
        output.particle.push(particles.clone());

        if t < input.max_turn - 1 {
            #[cfg(feature = "local")]
            {
                input_interactive! {
                    p: (i64, i64),
                    v: (i64, i64),
                }
                output.actual_position.push(Coord { x: p.0, y: p.1 });
                output.actual_velocity.push(Coord { x: v.0, y: v.1 });
                // eprintln!("p = {:?}, v = {:?}", p, v);
            }
        }
    }

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
    particle: Vec<Vec<Particle>>,
}
