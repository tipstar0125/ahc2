use itertools::Itertools;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{common::get_time, dsu::UnionFind, input::Input};

pub struct Forest {
    pub rng: Pcg64Mcg,
    pub score: f64,
    pub lengths: Vec<f64>,
    pub group: Vec<Vec<usize>>,
}

impl Forest {
    pub fn new(input: &Input, dist: &Vec<Vec<f64>>, TLE: f64) -> Self {
        // グループに含まれるべき点の個数で降順ソートし、大きいものから順にグループを構成する
        // グループを構成する最初の頂点は、使用していない頂点の中からランダムに選択する
        // 次の頂点は、使用していない頂点の中から、最も近い頂点を選択し、グループに含め木構造を構築する

        let mut G_with_idx = input
            .G
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, g)| (g, i))
            .collect::<Vec<(usize, usize)>>();
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

        let mut rng = Pcg64Mcg::new(10);
        let mut best_score = 1e18;
        let mut best_group = vec![];
        let mut best_lengths = vec![];

        while get_time() < TLE {
            let mut used = vec![false; input.N];
            let mut proceed_idx = vec![0; input.N];
            let mut group = vec![vec![]; input.M];
            let mut lengths = vec![0.0; input.M];
            let mut score = 0.0;

            for &(num, g_idx) in G_with_idx.iter() {
                let mut node_idx = rng.gen_range(0..input.N);
                while used[node_idx] {
                    node_idx = rng.gen_range(0..input.N);
                }
                used[node_idx] = true;
                let mut nodes = vec![node_idx];
                group[g_idx].push(node_idx);

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
                    group[g_idx].push(next_node);
                    lengths[g_idx] += min_dist;
                    score += min_dist;
                }
            }
            if score < best_score {
                best_score = score;
                best_group = group;
                best_lengths = lengths;
            }
        }

        Self {
            rng,
            score: best_score,
            lengths: best_lengths,
            group: best_group,
        }
    }

    pub fn annealing(&mut self, input: &Input, dist: &Vec<Vec<f64>>, TLE: f64) {
        let mut iter = 0;
        let mut updated_cnt = 0;
        let T0 = 200.0;
        let T1 = 10.0;
        while get_time() < TLE {
            iter += 1;
            let ga = self.rng.gen_range(0..input.M);
            let gb = self.rng.gen_range(0..input.M);
            if ga == gb {
                continue;
            }
            let before_length = self.lengths[ga] + self.lengths[gb];

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
            let temp = T0 + (T1 - T0) * get_time() / TLE;
            if diff_score <= 0.0 || self.rng.gen_bool((-diff_score / temp).exp()) {
                self.score += diff_score;
                self.lengths[ga] = score_a;
                self.lengths[gb] = score_b;
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

        eprintln!("Score = {}", self.score as usize);
    }
}
