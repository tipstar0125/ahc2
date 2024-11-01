#![allow(non_snake_case)]
#![allow(dead_code)]

mod arm;
mod beam;
mod common;
mod coord;
mod hash;
mod input;
mod state;

use arm::Arm;
use beam::{BeamSearch, Node};
use common::get_time;
use coord::Coord;
use hash::CalcHash;
use input::read_input;
use rand_pcg::Pcg64Mcg;
use state::{move_action_to_directon, FingerAction, State};

const DIRS: [char; 5] = ['R', 'D', 'L', 'U', '.'];

fn main() {
    get_time();
    let mut rng = Pcg64Mcg::new(0);
    let input = read_input();
    let arm = Arm::new(&input);
    let init_state = State::new(&arm, &input);
    let state_hash = CalcHash::new(&input, &mut rng);
    let start = Coord::new(input.N / 2, input.N / 2);
    let init_hash = state_hash.init(&input, start);
    let init_node = Node {
        track_id: !0,
        score: 0,
        hash: init_hash,
        state: init_state,
    };
    let mut beam = BeamSearch::new(init_node);
    let ops = beam.solve(1, 2, &input, &mut rng, &arm, &state_hash);

    arm.output();

    for op in ops.iter() {
        let mut output = "".to_string();
        for &(action, _) in op.move_actions.iter() {
            output += DIRS[move_action_to_directon(action) as usize]
                .to_string()
                .as_str();
        }
        // 根は何もしない
        output += ".";
        for &(action, _, _) in op.finger_actions.iter() {
            if action == FingerAction::Grab || action == FingerAction::Release {
                output += "P";
            } else {
                output += ".";
            }
        }
        eprintln!("{}", output);
        println!("{}", output);
    }

    eprintln!("Elapsed: {}", get_time());
}
