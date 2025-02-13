#![allow(non_snake_case)]
#![allow(dead_code)]

use beam::Node;
use input::Input;
use rand_pcg::Pcg64Mcg;

use crate::{common::get_time, input::read_input};

mod beam;
mod common;
mod hash;
mod input;
mod state;
mod test;

fn solve(input: &Input) {
    let init_state = state::State::new(input);
    let init_node = Node {
        track_id: !0,
        state: init_state,
    };

    let mut beam = beam::BeamSearch::new(init_node);
    let width = 300;
    let ops = beam.solve(width, 200, &input, beam::ScoreOrder::Ascending);
    for op in ops.iter() {
        println!("{} {}", op.dir, op.idx);
    }
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}

fn playout(input: &Input) {
    let mut state = state::State::new(input);
    let mut cnt = 0;
    let mut ans = vec![];
    let mut best_score = 3000;
    let tle = 1.9;

    while cnt < input.N * 2 {
        let mut actions = state.get_greedy_dist_legal_action();
        actions.sort();
        let mut candidates = vec![];

        let remain_time = tle - get_time();
        let num = ((remain_time / tle) * 4.0).ceil() as usize + 1;

        for &(_, _, dir, idx, remove_x) in actions.iter().take(num) {
            let mut next_state = state.clone();
            match dir {
                'L' => next_state.shift_left(idx, 1),
                'R' => next_state.shift_right(idx, 1),
                'U' => next_state.shift_up(idx, 1),
                'D' => next_state.shift_down(idx, 1),
                _ => unreachable!(),
            };
            next_state.shift_cnt += 1;
            let next_cnt = cnt + if remove_x { 1 } else { 0 };
            if next_state.greedy_dist(false, best_score, next_cnt) {
                let s = next_state.get_score();
                candidates.push((s, dir, idx, remove_x));
            }
        }
        candidates.sort();
        candidates.reverse();
        let (s, dir, idx, remove_x) = candidates[0];
        if s > best_score {
            best_score = s;
        }
        if remove_x {
            cnt += 1;
        }

        match dir {
            'L' => ans.extend(state.shift_left(idx, 1)),
            'R' => ans.extend(state.shift_right(idx, 1)),
            'U' => ans.extend(state.shift_up(idx, 1)),
            'D' => ans.extend(state.shift_down(idx, 1)),
            _ => unreachable!(),
        };
        state.shift_cnt += 1;
    }
    for a in ans.iter() {
        println!("{}", a);
    }
}
