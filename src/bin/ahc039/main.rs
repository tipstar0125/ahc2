#![allow(non_snake_case)]
#![allow(dead_code)]

mod common;
mod coord;
mod input;
mod polygon;
mod state;
mod test;

use common::{connect9, get_time};
use input::{read_input, Input};
use polygon::polygon_grid_to_vertex_coords;
use rand_pcg::Pcg64Mcg;
use state::State;

fn solve(input: &Input) {
    let mut rng = Pcg64Mcg::new(10);
    let connect9 = connect9();
    let tle_list = vec![0.25, 0.5, 1.0, 1.5, 1.95];
    let grid_num_list = vec![25, 50, 100, 200, 400];
    let mut state = State::new(grid_num_list[0], input);
    state.annealing(&mut rng, &connect9, tle_list[0]);

    for (grid_num, tle) in grid_num_list.iter().skip(1).zip(tle_list.iter().skip(1)) {
        state.to_next_grid(*grid_num, input);
        state.annealing(&mut rng, &connect9, *tle);
    }
    let polygon = polygon_grid_to_vertex_coords(&state.grid);
    println!("{}", polygon.len());
    for p in polygon.iter() {
        println!("{} {}", p.x * state.dl as usize, p.y * state.dl as usize);
    }
    eprintln!("Length = {}", state.length);
    eprintln!("Score = {}", state.score);
}

fn vis(grid: &Vec<Vec<bool>>) {
    for y in (0..grid.len()).rev() {
        for x in 0..grid.len() {
            if grid[x][y] {
                eprint!("■ ");
            } else {
                eprint!("□ ");
            }
        }
        eprintln!();
    }
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
