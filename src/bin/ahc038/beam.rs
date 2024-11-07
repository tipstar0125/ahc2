use std::cmp::Reverse;

use rustc_hash::FxHashSet;

use crate::{
    input::Input,
    state::{Op, State},
};

#[derive(Debug, Clone)]
pub struct Node {
    pub track_id: usize,
    pub state: State,
}
impl Node {
    fn new_node(&self, cand: &Cand) -> Node {
        let mut ret = self.clone();
        ret.apply(cand);
        ret
    }
    fn apply(&mut self, cand: &Cand) {
        self.state.apply(cand.eval_score, cand.hash, &cand.op);
    }
}

#[derive(Debug, Clone)]
struct Cand {
    op: Op,
    parent: usize,
    eval_score: usize,
    hash: usize,
    is_done: bool,
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

    fn append_cands(&self, input: &Input, cands: &mut Vec<Cand>, _rng: &mut rand_pcg::Pcg64Mcg) {
        let mut mp = rustc_hash::FxHashMap::default(); // added from lib
        for parent_idx in 0..self.nodes.len() {
            let parent_node = &self.nodes[parent_idx];
            for (delta_score, hash, op, is_done) in parent_node.state.cand(input) {
                // added from lib start
                // スコアに変動がなく、スコアと根の座標が同じ場合は、腕の方向に依らず同一視
                if delta_score == 0 {
                    let next_root = parent_node.state.root
                        + crate::coord::DIJ5
                            [crate::state::move_action_to_direction(op.move_actions[0].0) as usize];
                    if mp.contains_key(&parent_node.state.score)
                        && mp[&parent_node.state.score] == next_root
                    {
                        continue;
                    }
                    mp.insert(parent_node.state.score, next_root);
                }
                // added from lib end
                let cand = Cand {
                    op,
                    parent: parent_idx,
                    eval_score: parent_node.state.score + delta_score,
                    hash,
                    is_done,
                };
                cands.push(cand);
            }
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
        is_ascending: bool,
    ) -> Vec<Op> {
        let mut cands = Vec::<Cand>::new();
        let mut set = FxHashSet::default();
        let mut before_score = 0;
        for t in 0..depth {
            if t != 0 {
                if is_ascending {
                    cands.sort_unstable_by_key(|a| a.eval_score);
                } else {
                    cands.sort_unstable_by_key(|a| Reverse(a.eval_score));
                }
                let best_cand = &cands[0];
                if best_cand.is_done {
                    break;
                }
                set.clear();
                if best_cand.eval_score == before_score {
                    self.update(
                        cands
                            .iter()
                            .filter(|cand| set.insert(cand.hash))
                            .take((input.N / 2 * input.N / 2).max(width))
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
                before_score = best_cand.eval_score;
            }
            cands.clear();
            self.append_cands(input, &mut cands, _rng);
        }

        let best = if is_ascending {
            cands.iter().min_by_key(|a| a.raw_score(input)).unwrap()
        } else {
            cands.iter().max_by_key(|a| a.raw_score(input)).unwrap()
        };
        let mut ret = self.restore(best.parent);
        ret.push(best.op.clone());
        ret
    }
}
