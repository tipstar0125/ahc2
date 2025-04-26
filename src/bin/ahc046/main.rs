#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::VecDeque;

use crate::{common::get_time, input::read_input};
use coord::{Coord, DIJ4};
use input::Input;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

mod common;
mod coord;
mod input;
mod test;

const TLE: f64 = 1.9; // 時間制限
const INF: usize = 1 << 30;

fn solve(input: &Input) {
    let mut rng = Pcg64Mcg::new(100);
    let mut best_ans = vec![];
    let mut best_score = 0;
    'outer: while get_time() < TLE {
        let mut ans = vec![];
        let mut current = input.start;
        let mut block = vec![vec![false; input.N]; input.N];
        let mut map = vec![vec![-1; input.N]; input.N];
        for (i, pos) in input.destinations.iter().enumerate() {
            map[pos.i][pos.j] = i as i32;
        }

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
                    if next.in_map(input.N)
                        && dist[pos.i][pos.j] + 1 < dist[next.i][next.j]
                        && !block[next.i][next.j]
                    {
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
            if route.len() < 2 {
                continue 'outer;
            }
            for i in 0..route.len() - 1 {
                let action = compute_action(route[i], route[i + 1]);
                if rng.gen_bool(0.1) {
                    let dir = rng.gen_range(0..4);
                    let next = route[i] + DIJ4[dir];
                    if next.in_map(input.N)
                        && !block[next.i][next.j]
                        && map[next.i][next.j] == -1
                        && action.1 != DIR[dir]
                    {
                        actions.push(("A".to_string(), DIR[dir].to_string()));
                        block[next.i][next.j] = true;
                    }
                }
                actions.push(action);
            }
            ans.push(actions);
            map[dest.i][dest.j] = -1;
        }
        let T = ans.iter().flatten().count();
        let score = input.M + 2 * input.M * input.N - T;
        if score > best_score {
            best_score = score;
            best_ans = ans;
        }
    }
    output(&best_ans);
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}

const DIR: [&str; 4] = ["R", "D", "L", "U"];

fn output(ans: &Vec<Vec<(String, String)>>) {
    for actions in ans.iter() {
        for (action, dir) in actions.iter() {
            println!("{} {}", action, dir);
        }
    }
}

fn compute_action(pos0: Coord, pos1: Coord) -> (String, String) {
    let mut action = String::new();
    let mut dir = String::new();
    if pos0.i == pos1.i {
        let d = (pos1.j as i32 - pos0.j as i32).abs();
        if d > 1 {
            action += "S";
        } else {
            action += "M";
        }
        if pos0.j < pos1.j {
            dir += "R";
        } else {
            dir += "L";
        }
    } else {
        let d = (pos1.i as i32 - pos0.i as i32).abs();
        if d > 1 {
            action += "S";
        } else {
            action += "M";
        }
        if pos0.i < pos1.i {
            dir += "D";
        } else {
            dir += "U";
        }
    }
    (action, dir)
}
