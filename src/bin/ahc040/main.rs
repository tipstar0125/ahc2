#![allow(non_snake_case)]
#![allow(dead_code)]

mod common;
mod input;
mod state;
mod test;

use std::collections::VecDeque;

use common::get_time;
use input::{read_input, Input};
use rand::Rng;
use rand_pcg::Pcg64Mcg;

const HOW_TO_PACK: [(bool, char); 4] = [(false, 'U'), (true, 'U'), (false, 'L'), (true, 'L')];

fn solve(input: &Input) {
    let mut rng = Pcg64Mcg::new(10);
    for _ in 0..input.T {
        let width_limit = rng.gen_range(5e4 as i32..7e5 as i32);
        let mut ans = "".to_string();
        ans += &format!("{}\n", input.N);
        let mut now = 0;
        while now < input.N {
            let mut Q = VecDeque::new();
            Q.push_back((now, input.wh2[now].0, vec![0], vec![input.wh2[now].1]));
            Q.push_back((now, input.wh2[now].1, vec![1], vec![input.wh2[now].0]));
            let mut cands = vec![];
            while let Some((pos, width, rotates, heights)) = Q.pop_front() {
                if width > width_limit || pos == input.N - 1 {
                    let mx = heights.iter().max().unwrap();
                    let mn = heights.iter().min().unwrap();
                    let diff_mx_mn = mx - mn;
                    cands.push((diff_mx_mn, pos, rotates));
                    continue;
                }
                let next = pos + 1;
                let mut next_rotates = rotates.clone();
                let mut next_heights = heights.clone();
                next_rotates.push(0);
                next_heights.push(input.wh2[next].1);
                Q.push_back((next, width + input.wh2[next].0, next_rotates, next_heights));

                let mut next_rotates = rotates.clone();
                let mut next_heights = heights.clone();
                next_rotates.push(1);
                next_heights.push(input.wh2[next].0);
                Q.push_back((next, width + input.wh2[next].1, next_rotates, next_heights));
            }
            cands.sort();
            let (_, pos, rotates) = cands[0].clone();
            for (i, r) in rotates.iter().enumerate() {
                if i == 0 {
                    ans += &format!("{} {} U {}\n", now + i, r, -1);
                } else {
                    ans += &format!("{} {} U {}\n", now + i, r, now + i - 1);
                }
            }
            now = pos + 1;
        }
        println!("{}", ans);
    }
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
