#![allow(non_snake_case)]
#![allow(dead_code)]

use std::collections::VecDeque;

use input::Input;
use itertools::Itertools;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{common::get_time, input::read_input};

mod common;
mod input;
mod state;
mod test;

fn solve(input: &Input) {
    let mut best_ans = vec![-1; input.N];
    let mut best_score = 0;
    let mut rng = Pcg64Mcg::new(100);

    while get_time() < 0.5 {
        let mut used = vec![false; input.N];
        let mut used_cnt = 0;
        let mut ans = vec![-1; input.N];
        let mut score = 1;
        while used_cnt < input.N {
            let mut root = !0;
            loop {
                let i = rng.gen_range(0..input.N);
                if used[i] {
                    continue;
                }
                root = i;
                break;
            }
            used_cnt += 1;
            used[root] = true;

            let mut Q = VecDeque::new();
            let mut route = vec![root];
            Q.push_back(root);
            while let Some(pos) = Q.pop_front() {
                let mut next_cands = vec![];
                for nxt in input.G[pos].iter() {
                    if used[*nxt] {
                        continue;
                    }
                    next_cands.push((input.A[*nxt], *nxt));
                }
                if next_cands.is_empty() {
                    break;
                }
                next_cands.sort();
                let (_, next) = next_cands.pop().unwrap();
                used_cnt += 1;
                used[next] = true;
                route.push(next);
                Q.push_back(next);
                if route.len() == input.H - 2 {
                    break;
                }
            }
            let mut before_leafs = vec![];
            let last_node = route[route.len() - 1];
            for nxt in input.G[last_node].iter() {
                if used[*nxt] {
                    continue;
                }
                used[*nxt] = true;
                used_cnt += 1;
                before_leafs.push(*nxt);
            }
            score += input.A[root];
            for i in 1..route.len() {
                ans[route[i]] = route[i - 1] as i32;
                score += input.A[route[i]] * (i + 1);
            }
            for i in 0..before_leafs.len() {
                ans[before_leafs[i]] = last_node as i32;
                score += input.A[before_leafs[i]] * (route.len() + 1);
            }
            for i in before_leafs.iter() {
                for nxt in input.G[*i].iter() {
                    if used[*nxt] {
                        continue;
                    }
                    used[*nxt] = true;
                    used_cnt += 1;
                    ans[*nxt] = *i as i32;
                    score += input.A[*nxt] * (route.len() + 2);
                }
            }
        }
        if score > best_score {
            best_score = score;
            best_ans = ans.clone();
        }
    }
    eprintln!("best_score = {}", best_score);
    println!("{}", best_ans.iter().join(" "));
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
