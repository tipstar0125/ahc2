#![allow(non_snake_case)]
#![allow(dead_code)]

use coord::{Coord, ADJ};
use input::Input;

use crate::{common::get_time, input::read_input};

mod bfs;
mod common;
mod coord;
mod dsu;
mod input;
mod state;
mod test;

const TLE: f64 = 1.0;

fn solve(input: &Input) {
    let mut best_score = input.K as i64;
    let mut best_state = state::State::new(input);

    let mut cnt = 0;
    let mut used_pos = vec![vec![false; input.N]; input.N];
    let mut used_home_workspace = vec![false; input.M * 2];
    let mut stations = vec![];
    while cnt < input.M * 2 {
        let mut cand = vec![];
        for i in 1..input.N - 1 {
            for j in 1..input.N - 1 {
                if used_pos[i][j] {
                    continue;
                }
                let pos = Coord::new(i, j);
                let mut added = 0;
                for &dij in ADJ.iter() {
                    let next = pos + dij;
                    if next.in_map(input.N) {
                        for &idx in input.home_workspace_field[next.i][next.j].iter() {
                            if !used_home_workspace[idx] {
                                added += 1;
                            }
                        }
                    }
                }
                cand.push((added, pos));
            }
        }
        cand.sort();
        cand.reverse();
        assert!(!cand.is_empty());
        let (added, pos) = cand[0];
        stations.push(pos);
        used_pos[pos.i][pos.j] = true;
        for &dij in ADJ.iter() {
            let next = pos + dij;
            if next.in_map(input.N) {
                for &idx in input.home_workspace_field[next.i][next.j].iter() {
                    used_home_workspace[idx] = true;
                }
            }
        }
        cnt += added;
    }

    for &station in stations.iter() {
        if get_time() > TLE {
            break;
        }
        let mut state = state::State::new(input);
        let score = state.greedy(station, input);
        if score > best_score {
            best_score = score;
            best_state = state;
        }
    }
    best_state.output();
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
