use std::collections::VecDeque;

use itertools::Itertools;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{common::get_time, dsu::UnionFind, input::Input};

pub struct CutTree {
    rng: Pcg64Mcg,
    edges: Vec<Vec<usize>>,
    group: Vec<Vec<usize>>,
}

impl CutTree {
    pub fn new(input: &Input, dist: &Vec<Vec<f64>>) -> Self {
        let mut edges = vec![vec![]; input.N];
        let mut cand = vec![];
        for i in 0..input.N {
            for j in i + 1..input.N {
                if i == j {
                    continue;
                }

                cand.push((dist[i][j], i, j));
            }
        }
        cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut uf = UnionFind::new(input.N);

        for (_, i, j) in cand.iter() {
            if uf.is_same(*i, *j) {
                continue;
            }
            uf.unite(*i, *j);
            edges[*i].push(*j);
            edges[*j].push(*i);
        }

        Self {
            rng: Pcg64Mcg::new(200),
            edges,
            group: vec![],
        }
    }
    pub fn cut(&mut self, input: &Input) {
        let mut group = vec![vec![]; input.M];
        let mut removed = vec![false; input.N];
        let mut made = vec![false; input.M];

        loop {
            let mut degrees = vec![0; input.N];
            for i in 0..input.N {
                degrees[i] = self.edges[i].len();
            }

            let mut Q = VecDeque::new();
            let mut size = vec![1; input.N];
            let mut used = vec![false; input.N];
            let mut parents = vec![!0; input.N];
            let mut children = vec![vec![]; input.N];
            for i in 0..input.N {
                if degrees[i] == 1 {
                    Q.push_back(i);
                }
            }

            while let Some(v) = Q.pop_front() {
                degrees[v] -= 1;
                used[v] = true;
                for &u in self.edges[v].iter() {
                    if used[u] {
                        continue;
                    }
                    size[u] += size[v];
                    parents[v] = u;
                    children[u].push(v);
                    degrees[u] -= 1;
                    if degrees[u] == 1 {
                        Q.push_back(u);
                    }
                }
            }

            let mut G_with_idx = input
                .G
                .iter()
                .cloned()
                .enumerate()
                .map(|(i, g)| (g, i))
                .collect::<Vec<(usize, usize)>>();
            G_with_idx.sort();
            G_with_idx.reverse();
            let mut removed_nodes = vec![];
            let mut made_cnt = 0;

            for (g, i) in G_with_idx.iter() {
                let mut indexes = size.iter().positions(|&s| s == *g).collect_vec();
                while let Some(idx) = indexes.pop() {
                    if removed[idx] {
                        continue;
                    }
                    if made[*i] {
                        continue;
                    }
                    made_cnt += 1;
                    let mut nodes = vec![idx];
                    let mut Q = VecDeque::new();
                    removed[idx] = true;
                    Q.push_back(idx);
                    while let Some(v) = Q.pop_front() {
                        for &u in children[v].iter() {
                            removed[u] = true;
                            nodes.push(u);
                            Q.push_back(u);
                        }
                    }
                    assert!(nodes.len() == *g);
                    removed_nodes.extend(nodes.clone());
                    group[*i] = nodes;
                    made[*i] = true;
                    break;
                }
            }

            for node in removed_nodes.iter() {
                self.edges[*node].clear();
                for i in 0..input.N {
                    self.edges[i].retain(|&u| u != *node);
                }
            }
            if made_cnt == input.M || made_cnt == 0 {
                break;
            }
        }

        self.group = group;
    }
    pub fn make_rest(&mut self, input: &Input, dist: &Vec<Vec<f64>>) {
        let finished = self.group.iter().all(|group| group.len() > 0);
        if finished {
            return;
        }
        eprintln!("not finished");

        let mut G_with_idx = vec![];
        for (i, group) in self.group.iter().enumerate() {
            if group.len() == 0 {
                G_with_idx.push((input.G[i], i));
            }
        }
        G_with_idx.sort();
        G_with_idx.reverse();

        let mut dist_idx = vec![vec![]; input.N];

        for i in 0..input.N {
            for j in 0..input.N {
                if i == j {
                    continue;
                }
                dist_idx[i].push((dist[i][j], j));
            }
            dist_idx[i].sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        }

        let mut used = vec![false; input.N];
        let mut proceed_idx = vec![0; input.N];

        let used_nodes = self.group.iter().flatten().collect_vec();
        for idx in used_nodes.iter() {
            used[**idx] = true;
        }

        for &(num, g_idx) in G_with_idx.iter() {
            let mut node_idx = self.rng.gen_range(0..input.N);
            while used[node_idx] {
                node_idx = self.rng.gen_range(0..input.N);
            }
            used[node_idx] = true;
            let mut nodes = vec![node_idx];
            self.group[g_idx].push(node_idx);

            while nodes.len() < num {
                let mut target_node = !0;
                let mut next_node = !0;
                let mut min_dist = 1e18;
                for &node_idx in nodes.iter() {
                    loop {
                        let (dist, next_idx) = dist_idx[node_idx][proceed_idx[node_idx]];
                        if used[next_idx] {
                            proceed_idx[node_idx] += 1;
                            continue;
                        }
                        if dist < min_dist {
                            min_dist = dist;
                            next_node = next_idx;
                            target_node = node_idx;
                        }
                        break;
                    }
                }
                assert!(target_node != !0);
                assert!(next_node != !0);
                assert!(min_dist != 1e18);
                used[next_node] = true;
                nodes.push(next_node);
                self.group[g_idx].push(next_node);
            }
        }
    }
    pub fn climbing(&mut self, input: &Input, dist: &Vec<Vec<f64>>, TLE: f64) {
        let mut lengths = vec![0.0; input.M];

        for (idx, group) in self.group.iter().enumerate() {
            let mut uf = UnionFind::new(group.len());
            let mut cand = vec![];
            for i in 0..group.len() {
                for j in i + 1..group.len() {
                    cand.push((dist[group[i]][group[j]], i, j));
                }
            }
            cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            for (_, i, j) in cand.iter() {
                if uf.is_same(*i, *j) {
                    continue;
                }
                uf.unite(*i, *j);
                lengths[idx] += dist[group[*i]][group[*j]];
            }
        }

        let mut iter = 0;
        let mut updated_cnt = 0;

        while get_time() < TLE {
            iter += 1;
            let ga = self.rng.gen_range(0..input.M);
            let gb = self.rng.gen_range(0..input.M);
            if ga == gb {
                continue;
            }
            let before_length = lengths[ga] + lengths[gb];

            let na_idx = self.rng.gen_range(0..self.group[ga].len());
            let nb_idx = self.rng.gen_range(0..self.group[gb].len());
            let na = self.group[ga].remove(na_idx);
            let nb = self.group[gb].remove(nb_idx);
            self.group[ga].push(nb);
            self.group[gb].push(na);

            // aのグループについて、最小全域木を構成
            let mut score_a = 0.0;
            let mut uf = UnionFind::new(self.group[ga].len());
            let mut cand = vec![];
            for i in 0..self.group[ga].len() {
                for j in i + 1..self.group[ga].len() {
                    cand.push((dist[self.group[ga][i]][self.group[ga][j]], i, j));
                }
            }
            cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            for (_, i, j) in cand.iter() {
                if uf.is_same(*i, *j) {
                    continue;
                }
                uf.unite(*i, *j);
                score_a += dist[self.group[ga][*i]][self.group[ga][*j]];
            }

            // bのグループについて、最小全域木を構成
            let mut score_b = 0.0;
            let mut uf = UnionFind::new(self.group[gb].len());
            let mut cand = vec![];
            for i in 0..self.group[gb].len() {
                for j in i + 1..self.group[gb].len() {
                    cand.push((dist[self.group[gb][i]][self.group[gb][j]], i, j));
                }
            }
            cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            for (_, i, j) in cand.iter() {
                if uf.is_same(*i, *j) {
                    continue;
                }
                uf.unite(*i, *j);
                score_b += dist[self.group[gb][*i]][self.group[gb][*j]];
            }

            let after_length = score_a + score_b;
            let diff_score = after_length - before_length;
            if diff_score < 0.0 {
                lengths[ga] = score_a;
                lengths[gb] = score_b;
                updated_cnt += 1;
            } else {
                let na = self.group[ga].pop().unwrap();
                let nb = self.group[gb].pop().unwrap();
                self.group[ga].push(nb);
                self.group[gb].push(na);
            }
        }
        eprintln!("updated_cnt = {}", updated_cnt);
        eprintln!("iter = {}", iter);
    }
    pub fn output(&self, dist: &Vec<Vec<f64>>) {
        println!("!");
        for group in self.group.iter() {
            println!("{}", group.iter().join(" "));
            let mut uf = UnionFind::new(group.len());
            let mut cand = vec![];
            for i in 0..group.len() {
                for j in i + 1..group.len() {
                    let d = dist[group[i]][group[j]];
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
