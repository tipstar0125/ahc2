#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::VecDeque;

use input::Input;
use itertools::Itertools;

use crate::{common::get_time, input::read_input};

mod common;
mod input;
mod state;
mod test;

fn solve(input: &Input) {
    let mut used = vec![false; input.N];
    let mut used_cnt = 0;
    let mut ans = vec![-1; input.N];
    let mut score = 1;
    let mut order = input
        .A
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, a)| (a, i))
        .collect_vec();
    order.sort();

    while used_cnt < input.N {
        let root = {
            let mut ret = !0;
            for (_, i) in order.iter() {
                if !used[*i] {
                    ret = *i;
                    break;
                }
            }
            ret
        };

        let mut Q = VecDeque::new();
        let mut routes = vec![];
        let route = vec![root];
        let remain = 5;
        Q.push_back(route);
        while let Some(route) = Q.pop_front() {
            if route.len() == input.H + 1 - remain {
                routes.push(route);
                continue;
            }
            let pos = *route.last().unwrap();
            let mut exists = false;
            for nxt in input.G[pos].iter() {
                if used[*nxt] || route.contains(nxt) {
                    continue;
                }
                exists = true;
                let mut next_route = route.clone();
                next_route.push(*nxt);
                Q.push_back(next_route);
            }
            if !exists {
                routes.push(route);
            }
        }

        let mut best_part_score = 0;
        let mut best_part_ans = vec![];
        let mut best_used_cnt = 0;

        for route in routes.iter() {
            let mut part_used = used.clone();
            let mut part_score = input.A[root];
            let mut part_ans = vec![(root, -1)];
            let mut part_used_cnt = 1;
            part_used[root] = true;
            for i in 1..route.len() {
                part_score += input.A[route[i]] * (i + 1);
                part_used[route[i]] = true;
                part_ans.push((route[i], route[i - 1] as i32));
                part_used_cnt += 1;
            }

            let mut now = vec![route[route.len() - 1]];
            for r in 0..remain {
                let mut next = vec![];
                for i in now.iter() {
                    for nxt in input.G[*i].iter() {
                        if part_used[*nxt] {
                            continue;
                        }
                        part_used[*nxt] = true;
                        part_used_cnt += 1;
                        part_score += input.A[*nxt] * (route.len() + r + 1);
                        part_ans.push((*nxt, *i as i32));
                        next.push(*nxt);
                    }
                }
                now = next;
            }
            if part_score > best_part_score {
                best_part_score = part_score;
                best_part_ans = part_ans;
                best_used_cnt = part_used_cnt;
            }
        }
        score += best_part_score;
        used_cnt += best_used_cnt;
        for (c, p) in best_part_ans.iter() {
            used[*c] = true;
            ans[*c] = *p;
        }
    }
    eprintln!("Score = {}", score);
    println!("{}", ans.iter().join(" "));
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
