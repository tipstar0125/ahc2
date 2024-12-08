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
    fn new_node(&self, cand: &Cand, input: &Input) -> Node {
        let mut ret = self.clone();
        ret.apply(cand, input);
        ret
    }
    fn apply(&mut self, cand: &Cand, input: &Input) {
        self.state
            .apply(cand.eval_score, cand.hash, &cand.op, input);
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
        for parent_idx in 0..self.nodes.len() {
            let parent_node = &self.nodes[parent_idx];
            for (delta_score, hash, op, is_done) in parent_node.state.cand(input) {
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

    fn update<I: Iterator<Item = Cand>>(&mut self, cands: I, input: &Input) {
        self.next_nodes.clear();
        for cand in cands {
            let parent_node = &self.nodes[cand.parent];
            let mut new_node = parent_node.new_node(&cand, input);
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
                self.update(
                    cands
                        .iter()
                        .filter(|cand| set.insert(cand.hash))
                        .take(width)
                        .cloned(),
                    input,
                );
            }
            cands.clear();
            self.append_cands(input, &mut cands, _rng);
        }

        cands.sort_unstable_by_key(|a| a.eval_score);
        for cand in cands.iter().take(input.T) {
            let mut ops = self.restore(cand.parent);
            ops.push(cand.op.clone());
            println!("{}", ops.len());
            for Op {
                p,
                r,
                d,
                b,
                pos: _,
                row: _,
            } in ops.iter()
            {
                println!("{} {} {} {}", p, if *r { 1 } else { 0 }, d, b,);
            }
        }

        let best = if is_ascending {
            cands.iter().min_by_key(|a| a.raw_score(input)).unwrap()
        } else {
            cands.iter().max_by_key(|a| a.raw_score(input)).unwrap()
        };
        eprintln!("Ideal = {}", best.eval_score);
        let mut ret = self.restore(best.parent);
        ret.push(best.op.clone());
        ret
    }
}
