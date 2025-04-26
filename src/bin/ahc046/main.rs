#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::VecDeque;

use crate::{common::get_time, input::read_input};
use coord::{Coord, DIJ4};
use input::Input;

mod common;
mod coord;
mod input;
mod test;

const TLE: f64 = 1.9; // 時間制限
const INF: usize = 1 << 30;

fn solve(input: &Input) {
    let mut ans = vec![];
    let mut routes = vec![];
    let mut current = input.start;

    for dest_idx in 0..input.M - 1 {
        if dest_idx != 0 {
            current = input.destinations[dest_idx - 1];
        }
        let dest = input.destinations[dest_idx];
        let mut Q = VecDeque::new();
        Q.push_back(current);
        let mut dist = vec![vec![INF; input.N]; input.N];
        dist[current.i][current.j] = 0;

        while let Some(pos) = Q.pop_front() {
            if pos == dest {
                break;
            }
            for dij in DIJ4.iter() {
                let next = pos + *dij;
                if next.in_map(input.N) && dist[pos.i][pos.j] + 1 < dist[next.i][next.j] {
                    dist[next.i][next.j] = dist[pos.i][pos.j] + 1;
                    Q.push_back(next);
                }
            }
        }
        let mut route = vec![];
        let mut actions = vec![];
        let mut pos = dest;
        route.push(pos);
        while pos != current {
            for dij in DIJ4.iter() {
                let next = pos + *dij;
                if next.in_map(input.N) && dist[pos.i][pos.j] == dist[next.i][next.j] + 1 {
                    route.push(next);
                    actions.push(compute_action(next, pos));
                    pos = next;
                    break;
                }
            }
        }
        route.reverse();
        actions.reverse();
        ans.push(actions);
        routes.push(route);
    }
    output(&ans, &routes);
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}

fn output(ans: &Vec<Vec<String>>, routes: &Vec<Vec<Coord>>) {
    for (actions, route) in ans.iter().zip(routes.iter()) {
        eprintln!("{:?}", route);
        for action in actions.iter() {
            println!("M {}", action);
        }
    }
}

fn compute_action(pos0: Coord, pos1: Coord) -> String {
    if pos0.i == pos1.i {
        if pos0.j < pos1.j {
            "R".to_string()
        } else {
            "L".to_string()
        }
    } else {
        if pos0.i < pos1.i {
            "D".to_string()
        } else {
            "U".to_string()
        }
    }
}
