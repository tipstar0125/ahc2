use std::cmp::Reverse;

use rustc_hash::FxHashSet;

use crate::{
    input::Input,
    state::{Op, RailTree, State},
};

#[derive(Debug, PartialEq, Eq)]
pub enum ScoreOrder {
    Ascending,  // Lower is better
    Descending, // Higher is better
}

#[derive(Debug, Clone)]
pub struct Node {
    pub track_id: usize,
    pub state: State,
}
impl Node {
    fn new_node(&self, cand: &Cand, input: &Input, rail_tree: &RailTree) -> Node {
        let mut ret = self.clone();
        ret.apply(cand, input, rail_tree);
        ret
    }
    fn apply(&mut self, cand: &Cand, input: &Input, rail_tree: &RailTree) {
        self.state
            .apply(cand.eval_score, cand.hash, &cand.op, input, rail_tree);
    }
}

#[derive(Debug, Clone)]
struct Cand {
    op: Op,
    parent: usize,
    eval_score: i64,
    hash: usize,
    is_done: bool,
}
impl Cand {
    fn raw_score(&self, _input: &Input) -> i64 {
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
    pub fn new(input: &Input, rail_tree: &RailTree) -> BeamSearch {
        let node = Node {
            track_id: !0,
            state: State::new(input, rail_tree),
        };
        BeamSearch {
            nodes: vec![node],
            track: vec![],
            next_nodes: vec![],
        }
    }

    #[allow(unused_variables)]
    fn append_cands(
        &self,
        input: &Input,
        rail_tree: &RailTree,
        cands: &mut Vec<Cand>,
        rng: &mut rand_pcg::Pcg64Mcg,
    ) {
        for parent_idx in 0..self.nodes.len() {
            let parent_node = &self.nodes[parent_idx];
            for (score, hash, op, is_done) in parent_node.state.cand(input, rail_tree) {
                let cand = Cand {
                    op,
                    parent: parent_idx,
                    eval_score: score,
                    hash,
                    is_done,
                };
                cands.push(cand);
            }
        }
    }

    fn update<I: Iterator<Item = Cand>>(&mut self, cands: I, input: &Input, rail_tree: &RailTree) {
        self.next_nodes.clear();
        for cand in cands {
            let parent_node = &self.nodes[cand.parent];
            let mut new_node = parent_node.new_node(&cand, input, rail_tree);
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
        rail_tree: &RailTree,
        score_order: ScoreOrder,
    ) -> Vec<Op> {
        let mut cands = Vec::<Cand>::new();
        let mut best_score = 0;
        let mut best_ops = vec![];
        let mut set = FxHashSet::default();
        let mut rng = rand_pcg::Pcg64Mcg::new(0);
        for t in 0..depth {
            if t != 0 {
                if score_order == ScoreOrder::Ascending {
                    cands.sort_unstable_by_key(|a| a.eval_score);
                } else {
                    cands.sort_unstable_by_key(|a| Reverse(a.op.score));
                }
                if !cands.is_empty() {
                    let best_cand = &cands[0];
                    if best_cand.op.score > best_score {
                        best_score = best_cand.op.score;
                        let mut ops = self.restore(best_cand.parent);
                        ops.push(best_cand.op.clone());
                        best_ops = ops;
                    }
                }
                set.clear();
                self.update(
                    cands
                        .iter()
                        .filter(|cand| !cand.is_done)
                        .filter(|cand| set.insert(cand.hash))
                        .take(width)
                        .cloned(),
                    input,
                    rail_tree,
                );
            }
            cands.clear();
            self.append_cands(input, rail_tree, &mut cands, &mut rng);
        }
        eprintln!("Score = {}", best_score);
        best_ops
    }
}
