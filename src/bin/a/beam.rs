use std::cmp::Reverse;

use crate::{
    arm::Arm,
    coord::{Coord, DIJ5},
    hash::CalcHash,
    input::Input,
    state::{move_action_to_directon, Direction, FingerAction, FingerHas, MoveAction, State},
};

#[derive(Debug, Clone)]
pub struct Node {
    pub track_id: usize,
    pub score: usize,
    pub hash: usize,
    pub state: State,
}
impl Node {
    fn new_node(&self, cand: &Cand) -> Node {
        let mut ret = self.clone();
        ret.apply(cand);
        ret
    }
    fn apply(&mut self, cand: &Cand) {
        self.state.root =
            self.state.root + DIJ5[move_action_to_directon(cand.op.move_actions[0].0) as usize];
        self.state.arm_direction = cand
            .op
            .move_actions
            .iter()
            .cloned()
            .skip(1)
            .map(|x| x.1)
            .collect::<Vec<Direction>>();
        self.state.finger_status = cand
            .op
            .finger_actions
            .iter()
            .cloned()
            .map(|x| (x.0, x.1))
            .collect::<Vec<(FingerAction, FingerHas)>>();
        for (finger_action, _, coord) in cand.op.finger_actions.iter() {
            if *finger_action == FingerAction::Grab {
                self.state.S[coord.i][coord.j] = '0';
            } else if *finger_action == FingerAction::Release {
                self.state.S[coord.i][coord.j] = '1';
            }
        }
        self.score = cand.eval_score;
        self.hash = cand.hash;
    }
}

#[derive(Debug, Clone)]
pub struct Op {
    pub move_actions: Vec<(MoveAction, Direction)>,
    pub finger_actions: Vec<(FingerAction, FingerHas, Coord)>,
}

#[derive(Debug, Clone)]
struct Cand {
    op: Op,
    parent: usize,
    eval_score: usize,
    hash: usize,
}
impl Cand {
    fn raw_score(&self, _input: &Input) -> usize {
        self.eval_score
    }
}

#[derive(Debug)]
pub struct BeamSearch {
    track: Vec<(usize, Op)>,
    nodes: Vec<Node>,
    next_nodes: Vec<Node>,
}
impl BeamSearch {
    pub fn new(node: Node) -> BeamSearch {
        BeamSearch {
            nodes: vec![node],
            track: vec![],
            next_nodes: vec![],
        }
    }

    fn enum_cands(
        &self,
        input: &Input,
        cands: &mut Vec<Cand>,
        _rng: &mut rand_pcg::Pcg64Mcg,
        arm: &Arm,
        calc_hash: &CalcHash,
    ) {
        for i in 0..self.nodes.len() {
            self.append_cands(input, i, cands, _rng, arm, calc_hash);
        }
    }

    fn append_cands(
        &self,
        input: &Input,
        parent_idx: usize,
        cands: &mut Vec<Cand>,
        _rng: &mut rand_pcg::Pcg64Mcg,
        arm: &Arm,
        calc_hash: &CalcHash,
    ) {
        let parent_node = &self.nodes[parent_idx];
        let parent_hash = parent_node.hash;

        for (score, arm, finger) in parent_node.state.cand(arm, &input.T) {
            let parent_root_pos = parent_node.state.root;
            let child_root_pos = parent_root_pos + DIJ5[move_action_to_directon(arm[0].0) as usize];
            let field_change_coords: Vec<Coord> = finger
                .iter()
                .filter(|x| x.0 == FingerAction::Grab || x.0 == FingerAction::Release)
                .map(|x| x.2)
                .collect();
            let arm_direction_changes: Vec<(Direction, Direction)> = arm
                .iter()
                .skip(1)
                .map(|x| x.1)
                .zip(parent_node.state.arm_direction.iter().cloned())
                .collect();
            let hash = calc_hash.calc(
                parent_hash,
                &field_change_coords,
                parent_root_pos,
                child_root_pos,
                &arm_direction_changes,
            );

            let cand = Cand {
                op: Op {
                    move_actions: arm,
                    finger_actions: finger,
                },
                parent: parent_idx,
                eval_score: parent_node.score + score,
                hash,
            };
            cands.push(cand);
        }
    }

    fn update<I: Iterator<Item = Cand>>(&mut self, cands: I) {
        self.next_nodes.clear();
        for cand in cands {
            let parent_node = &self.nodes[cand.parent];
            let mut new_node = parent_node.new_node(&cand);
            self.track.push((parent_node.track_id, cand.op));
            new_node.track_id = self.track.len() - 1;
            self.next_nodes.push(new_node);
        }
        std::mem::swap(&mut self.nodes, &mut self.next_nodes);
    }

    fn restore(&self, mut idx: usize) -> Vec<Op> {
        idx = self.nodes[idx].track_id;
        let mut ret = vec![];
        while idx != !0 {
            ret.push(self.track[idx].1.clone());
            idx = self.track[idx].0;
        }
        ret.reverse();
        ret
    }

    pub fn solve(
        &mut self,
        width: usize,
        depth: usize,
        input: &Input,
        _rng: &mut rand_pcg::Pcg64Mcg,
        arm: &Arm,
        calc_hash: &CalcHash,
        necessary_score: usize,
    ) -> Vec<Op> {
        let mut cands = Vec::<Cand>::new();
        let mut set = rustc_hash::FxHashSet::default();
        let mut before_score = 0;
        for t in 0..depth {
            if t != 0 {
                cands.sort_unstable_by_key(|a| Reverse(a.eval_score));
                let best_score = cands[0].eval_score;
                if before_score == necessary_score {
                    break;
                }
                set.clear();
                if best_score == before_score {
                    self.update(
                        cands
                            .iter()
                            .filter(|cand| set.insert(cand.hash))
                            .take(input.N / 2 * input.N / 2)
                            .cloned(),
                    );
                } else {
                    self.update(
                        cands
                            .iter()
                            .filter(|cand| set.insert(cand.hash))
                            .take(width)
                            .cloned(),
                    );
                }
                before_score = best_score;
            }
            cands.clear();
            self.enum_cands(input, &mut cands, _rng, arm, calc_hash);
        }

        let best = cands.iter().max_by_key(|a| a.raw_score(input)).unwrap();
        let mut ret = self.restore(best.parent);
        ret.push(best.op.clone());
        ret
    }
}
