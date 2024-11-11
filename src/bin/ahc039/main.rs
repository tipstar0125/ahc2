#![allow(non_snake_case)]
#![allow(dead_code)]

mod common;
mod coord;
mod input;
mod state;
mod test;

use common::get_time;
use coord::Coord;
use input::{read_input, Input};
use state::State;

fn solve(input: &Input) {
    let mut best_score = 0;
    let mut best_delta = 1;
    let mut best_length = 0;
    let mut best_polygon = vec![
        Coord::new(0, 0),
        Coord::new(1e5 as usize, 0),
        Coord::new(1e5 as usize, 1e5 as usize),
        Coord::new(0, 1e5 as usize),
    ];
    for grid_num in 2..=250 {
        if input.size % grid_num != 0 {
            continue;
        }
        let mut state = State::new(grid_num, input);
        let mut group = state.grouping_saba_area();
        if group.is_empty() {
            continue;
        }
        group.sort();
        group.reverse();
        let (score, g) = group[0].clone();
        let (polygon, length) = state.make_polygon(&g);
        if polygon.len() > 0 && score > best_score {
            best_score = score;
            best_delta = input.size / grid_num;
            best_polygon = polygon;
            best_length = length;
        }
    }
    println!("{}", best_polygon.len());
    for p in best_polygon.iter() {
        println!("{} {}", p.x * best_delta, p.y * best_delta);
    }
    eprintln!("Length = {}", best_length);
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
