use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;

use crate::{
    common::{eprint_green, get_time},
    dsu::UnionFind,
    estimator::Estimator,
    input::Input,
};

pub struct Forest {
    rng: Pcg64Mcg,
    group: Vec<Vec<usize>>,
    dist: Vec<Vec<usize>>,
}

impl Forest {
    pub fn new(input: &Input, estimator: &Estimator) -> Self {
        let mut dist = vec![vec![0; input.N]; input.N];
        for i in 0..input.N {
            for j in i + 1..input.N {
                dist[i][j] = estimator.positions[i].euclidean_dist(estimator.positions[j]);
                dist[j][i] = dist[i][j];
            }
        }
        Self {
            rng: Pcg64Mcg::new(200),
            group: vec![vec![]; input.M],
            dist,
        }
    }
    pub fn prim(
        &mut self,
        gi: usize,
        g_size: usize,
        nodes: &Vec<usize>,
        used: &mut Vec<bool>,
    ) -> usize {
        let mut Q = BinaryHeap::new();
        let start = **nodes
            .iter()
            .filter(|&&i| !used[i])
            .collect_vec()
            .choose(&mut self.rng)
            .unwrap();
        used[start] = true;
        self.group[gi].push(start);
        for &i in nodes.iter() {
            if !used[i] {
                Q.push((Reverse(self.dist[start][i]), i));
            }
        }

        let mut score = 0;
        while let Some((Reverse(d), u)) = Q.pop() {
            if self.group[gi].len() == g_size {
                break;
            }
            if used[u] {
                continue;
            }
            used[u] = true;
            self.group[gi].push(u);
            score += d;
            for &v in nodes.iter() {
                if !used[v] {
                    Q.push((Reverse(self.dist[u][v]), v));
                }
            }
        }
        score
    }
    pub fn greedy(&mut self, input: &Input) {
        let order_by_group_size = input
            .G
            .iter()
            .enumerate()
            .sorted_by_key(|(_, &g)| g)
            .rev()
            .map(|(i, _)| i)
            .collect_vec();

        let mut used = vec![false; input.N];
        for gi in order_by_group_size {
            self.prim(gi, input.G[gi], &(0..input.N).collect_vec(), &mut used);
        }
    }
    pub fn random(&mut self, input: &Input, tle: f64) {
        let mut used = vec![false; input.N];
        let mut best_score = 1 << 60;
        let mut best_group = vec![];
        let mut iter = 0;
        loop {
            if get_time() > tle {
                break;
            }
            used.fill(false);
            self.group.fill(vec![]);
            let mut order = (0..input.M).collect_vec();
            order.shuffle(&mut self.rng);
            let mut score = 0;
            for gi in order {
                score += self.prim(gi, input.G[gi], &(0..input.N).collect_vec(), &mut used);
            }
            if score < best_score {
                best_score = score;
                best_group = self.group.clone();
            }
            iter += 1;
        }
        self.group = best_group;
        eprint_green(&format!("forest random iter = {}", iter));
    }
    pub fn output(&self) {
        println!("!");
        for group in self.group.iter() {
            println!("{}", group.iter().join(" "));
            let mut uf = UnionFind::new(group.len());
            let mut cand = vec![];
            for i in 0..group.len() {
                for j in i + 1..group.len() {
                    let d = self.dist[group[i]][group[j]];
                    cand.push((d, i, j, group[i], group[j]));
                }
            }
            cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            for (_, i, j, a, b) in cand.iter() {
                if uf.is_same(*i, *j) {
                    continue;
                }
                uf.unite(*i, *j);
                println!("{} {}", a, b);
            }
        }
    }
}
