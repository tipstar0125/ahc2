use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;

use crate::{dsu::UnionFind, estimator::Estimator, input::Input};

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
        let mut Q = BinaryHeap::new();

        for gi in order_by_group_size {
            Q.clear();
            let start = *(0..input.N)
                .filter(|&i| !used[i])
                .collect_vec()
                .choose(&mut self.rng)
                .unwrap();

            self.group[gi].push(start);
            used[start] = true;
            for i in 0..input.N {
                if !used[i] {
                    Q.push((Reverse(self.dist[start][i]), i));
                }
            }

            while let Some((_, u)) = Q.pop() {
                if self.group[gi].len() == input.G[gi] {
                    break;
                }
                if used[u] {
                    continue;
                }
                used[u] = true;
                self.group[gi].push(u);
                for v in 0..input.N {
                    if !used[v] {
                        Q.push((Reverse(self.dist[u][v]), v));
                    }
                }
            }
        }
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
