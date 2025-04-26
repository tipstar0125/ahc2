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
    let block = vec![vec![false; input.N]; input.N];

    for dest_idx in 0..input.M - 1 {
        if dest_idx != 0 {
            current = input.destinations[dest_idx - 1];
        }
        let dest = input.destinations[dest_idx];
        let mut Q = VecDeque::new();
        Q.push_back((current, vec![current]));
        let mut dist = vec![vec![INF; input.N]; input.N];
        dist[current.i][current.j] = 0;
        let mut route = vec![];

        while let Some((pos, r)) = Q.pop_front() {
            if pos == dest {
                route = r;
                break;
            }
            for dij in DIJ4.iter() {
                let next = pos + *dij;
                if next.in_map(input.N) && dist[pos.i][pos.j] + 1 < dist[next.i][next.j] {
                    dist[next.i][next.j] = dist[pos.i][pos.j] + 1;
                    let mut next_route = r.clone();
                    next_route.push(next);
                    Q.push_back((next, next_route));
                }
            }
            for dij in DIJ4.iter() {
                let mut before = pos;
                let mut next = pos + *dij;
                loop {
                    if !next.in_map(input.N) || block[next.i][next.j] {
                        next = before;
                        break;
                    }
                    before = next;
                    next = next + *dij;
                }
                if next.in_map(input.N) && dist[pos.i][pos.j] + 1 < dist[next.i][next.j] {
                    dist[next.i][next.j] = dist[pos.i][pos.j] + 1;
                    let mut next_route = r.clone();
                    next_route.push(next);
                    Q.push_back((next, next_route));
                }
            }
        }

        let mut actions = vec![];
        for i in 0..route.len() - 1 {
            actions.push(compute_action(route[i], route[i + 1]));
        }
        ans.push(actions);
        routes.push(route);
    }
    let T = ans.iter().flatten().count();
    let score = input.M + 2 * input.M * input.N - T;
    eprintln!("Score = {}", score);
    output(&ans);
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}

fn output(ans: &Vec<Vec<String>>) {
    for actions in ans.iter() {
        for action in actions.iter() {
            println!("{}", action);
        }
    }
}

fn compute_action(pos0: Coord, pos1: Coord) -> String {
    let mut res = String::new();
    if pos0.i == pos1.i {
        let d = (pos1.j as i32 - pos0.j as i32).abs();
        if d > 1 {
            res += "S ";
        } else {
            res += "M ";
        }
        if pos0.j < pos1.j {
            res += "R";
        } else {
            res += "L";
        }
    } else {
        let d = (pos1.i as i32 - pos0.i as i32).abs();
        if d > 1 {
            res += "S ";
        } else {
            res += "M ";
        }
        if pos0.i < pos1.i {
            res += "D";
        } else {
            res += "U";
        }
    }
    res
}
