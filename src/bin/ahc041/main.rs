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

    while get_time() < 1.95 {
        let mut used = vec![false; input.N];
        let mut used_cnt = 0;
        let mut ans = vec![-1; input.N];
        let mut score = 1;
        while used_cnt < input.N {
            let mut best_part_score = 0;
            let mut best_part_used = vec![];
            let mut best_part_ans = vec![];

            for _ in 0..500 {
                let mut used_ = used.clone();
                let mut part_score = 0;
                let mut part_used = vec![];
                let mut part_ans = vec![];

                let mut root = !0;
                loop {
                    let i = rng.gen_range(0..input.N);
                    if used[i] {
                        continue;
                    }
                    root = i;
                    break;
                }

                part_used.push(root);
                used_[root] = true;

                let mut Q = VecDeque::new();
                let mut route = vec![root];
                let remain = 5;
                Q.push_back(root);
                while let Some(pos) = Q.pop_front() {
                    let mut next_cands = vec![];
                    for nxt in input.G[pos].iter() {
                        if used_[*nxt] {
                            continue;
                        }
                        next_cands.push((input.A[*nxt], *nxt));
                    }
                    if next_cands.is_empty() {
                        break;
                    }
                    next_cands.sort();
                    let (_, next) = next_cands.pop().unwrap();
                    used_[next] = true;
                    part_used.push(next);
                    route.push(next);
                    Q.push_back(next);
                    if route.len() == input.H - remain {
                        break;
                    }
                }

                part_score += input.A[root];
                for i in 1..route.len() {
                    part_ans.push((route[i - 1], route[i]));
                    part_score += input.A[route[i]] * (i + 1);
                }

                let mut now = vec![route[route.len() - 1]];
                for r in 0..remain {
                    let mut next = vec![];
                    for i in now.iter() {
                        for nxt in input.G[*i].iter() {
                            if used_[*nxt] {
                                continue;
                            }
                            used_[*nxt] = true;
                            part_used.push(*nxt);
                            part_score += input.A[*nxt] * (route.len() + r + 1);
                            part_ans.push((*i, *nxt));
                            next.push(*nxt);
                        }
                    }
                    now = next;
                }
                if part_score > best_part_score {
                    best_part_score = part_score;
                    best_part_used = part_used;
                    best_part_ans = part_ans;
                }
            }

            for i in best_part_used.iter() {
                used[*i] = true;
            }
            used_cnt += best_part_used.len();
            score += best_part_score;
            for (a, b) in best_part_ans {
                ans[b] = a as i32;
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
