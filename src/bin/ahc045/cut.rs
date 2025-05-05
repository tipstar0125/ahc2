use std::{cmp::Ordering, collections::VecDeque};

use itertools::Itertools;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use rustc_hash::FxHashMap;

use crate::{
    common::{eprint_blue, eprint_green, eprint_yellow, get_time},
    coord::Coord,
    dsu::UnionFind,
    estimator::Estimator,
    input::Input,
};

#[derive(Clone)]
pub struct CutTree {
    rng: Pcg64Mcg,
    edges: Vec<Vec<usize>>,
    group: Vec<Vec<usize>>,
    order_map: FxHashMap<(usize, usize), usize>,
}

impl CutTree {
    pub fn new(input: &Input, dist: &Vec<Vec<f64>>, estimator: &Estimator) -> Self {
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

        let mut order_map = FxHashMap::default();
        for (i, &(_, u, v)) in cand.iter().enumerate() {
            order_map.insert((u, v), i);
        }

        for _ in 0..2 {
            let mut error_cnt = 0;
            for ineq in estimator.inequalities.iter() {
                let mut short_u = ineq.short.0;
                let mut short_v = ineq.short.1;
                if short_u > short_v {
                    std::mem::swap(&mut short_u, &mut short_v);
                }
                let mut long_u = ineq.long.0;
                let mut long_v = ineq.long.1;
                if long_u > long_v {
                    std::mem::swap(&mut long_u, &mut long_v);
                }
                if order_map.contains_key(&(short_u, short_v))
                    && order_map.contains_key(&(long_u, long_v))
                {
                    let order_short = order_map[&(short_u, short_v)];
                    let order_long = order_map[&(long_u, long_v)];
                    if order_short > order_long {
                        eprint_green(&format!("swap: {} {}", order_short, order_long));
                        cand.swap(order_short, order_long);
                        order_map.remove(&(short_u, short_v));
                        order_map.remove(&(long_u, long_v));
                        order_map.insert((short_u, short_v), order_long);
                        order_map.insert((long_u, long_v), order_short);
                        eprint_yellow(&format!("{:?}", cand[order_short]));
                        eprint_yellow(&format!("{:?}", cand[order_long]));
                        error_cnt += 1;
                    }
                }
            }
            eprint_blue(&format!("error_cnt: {}", error_cnt));
            if error_cnt == 0 {
                break;
            }
        }
        let mut order_map_keys = order_map.keys().cloned().collect_vec();
        order_map_keys.sort();
        for &(u, v) in order_map_keys.iter() {
            let order = order_map[&(u, v)];
            order_map.insert((v, u), order);
        }

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
            order_map,
        }
    }
    pub fn cut(&mut self, input: &Input) {
        let xy_center = input
            .rects
            .iter()
            .map(|rect| rect.center())
            .collect::<Vec<Coord>>();
        let mut dist = vec![vec![0.0; input.N]; input.N];
        for i in 0..input.N {
            for j in i + 1..input.N {
                dist[i][j] = xy_center[i].euclidean_dist(xy_center[j]) as f64;
                dist[j][i] = dist[i][j];
            }
        }

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
                indexes.sort_by(|a, b| {
                    if parents[*a] == !0 || parents[*b] == !0 {
                        Ordering::Equal
                    } else {
                        dist[*a][parents[*a]]
                            .partial_cmp(&dist[*b][parents[*b]])
                            .unwrap()
                    }
                });
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
            if made_cnt == 0 {
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
    pub fn annealing(&mut self, input: &Input, dist: &Vec<Vec<f64>>, TLE: f64) {
        let mut lengths = vec![0.0; input.M];

        for (idx, group) in self.group.iter().enumerate() {
            let mut uf = UnionFind::new(group.len());
            let mut cand = vec![];
            for i in 0..group.len() {
                for j in i + 1..group.len() {
                    cand.push((self.order_map[&(group[i], group[j])], i, j));
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

        let mut group_map = vec![!0; input.N];
        for (group_idx, nodes) in self.group.iter().enumerate() {
            for &idx in nodes.iter() {
                group_map[idx] = group_idx;
            }
        }

        let mut dist_order_by = vec![vec![]; input.N];
        for i in 0..input.N {
            for j in 0..input.N {
                if i == j {
                    continue;
                }
                dist_order_by[i].push((dist[i][j], j));
            }
            dist_order_by[i].sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        }

        let mut neighbor_group = vec![vec![]; input.M];
        for i in 0..input.N {
            for &(_, j) in dist_order_by[i].iter() {
                if group_map[i] != group_map[j] {
                    neighbor_group[group_map[i]].push(group_map[j]);
                    break;
                }
            }
        }
        for i in 0..input.M {
            neighbor_group[i].sort();
            neighbor_group[i].dedup();
        }

        let mut iter = 0;
        let mut updated_cnt = 0;

        let T0 = 150.0;
        let T1 = 10.0;

        while input.M > 1 && get_time() < TLE {
            iter += 1;
            let mut ga = self.rng.gen_range(0..input.M);
            let mut gb = neighbor_group[ga][self.rng.gen_range(0..neighbor_group[ga].len())];

            if input.G[ga] > input.G[gb] {
                std::mem::swap(&mut ga, &mut gb);
            }

            let mut before_length = lengths[ga] + lengths[gb];

            let nodes = self.group[ga]
                .iter()
                .cloned()
                .chain(self.group[gb].iter().cloned())
                .collect_vec();

            // a, bのグループについて、最小全域木を構成
            let mut uf = UnionFind::new(nodes.len());
            let mut edges = vec![vec![]; input.N];
            let mut cand = vec![];
            for i in 0..nodes.len() {
                for j in i + 1..nodes.len() {
                    cand.push((self.order_map[&(nodes[i], nodes[j])], i, j));
                }
            }
            cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            let mut score_c = 0.0;
            for (_, i, j) in cand.iter() {
                if uf.is_same(*i, *j) {
                    continue;
                }
                uf.unite(*i, *j);
                edges[nodes[*i]].push(nodes[*j]);
                edges[nodes[*j]].push(nodes[*i]);
                score_c += dist[nodes[*i]][nodes[*j]];
            }

            let size_c = nodes.len();
            let cand_c = input
                .G
                .iter()
                .enumerate()
                .filter(|(_, g)| **g == size_c)
                .map(|(i, _)| i)
                .collect_vec();

            if cand_c.is_empty() {
                let mut size = vec![0; input.N];
                for i in 0..nodes.len() {
                    size[nodes[i]] = 1;
                }
                let start = nodes[0];
                let mut used = vec![false; input.N];
                let mut parents = vec![!0; input.N];
                let mut children = vec![vec![]; input.N];
                used[start] = true;
                dfs(
                    start,
                    &edges,
                    &mut used,
                    &mut size,
                    &mut parents,
                    &mut children,
                );
                assert!(*size.iter().max().unwrap() == nodes.len());

                let mut indexes = size.iter().positions(|&s| s == input.G[ga]).collect_vec();
                indexes.sort_by(|a, b| {
                    if parents[*a] == !0 || parents[*b] == !0 {
                        Ordering::Equal
                    } else {
                        dist[*a][parents[*a]]
                            .partial_cmp(&dist[*b][parents[*b]])
                            .unwrap()
                    }
                });
                let idx = {
                    if indexes.len() > 0 {
                        indexes.pop().unwrap()
                    } else {
                        let mut idx = 0;
                        let mut diff = 100i32;
                        for i in 0..input.N {
                            let s = size[i];
                            if (input.G[ga] as i32 - s as i32).abs() < diff {
                                diff = (input.G[ga] as i32 - s as i32).abs();
                                idx = i;
                            }
                        }
                        idx
                    }
                };

                let mut a_nodes = vec![idx];
                let mut Q = VecDeque::new();
                Q.push_back(idx);
                while let Some(v) = Q.pop_front() {
                    for &u in children[v].iter() {
                        a_nodes.push(u);
                        Q.push_back(u);
                    }
                }
                let mut b_nodes = vec![];
                for node in nodes.iter() {
                    if !a_nodes.contains(node) {
                        b_nodes.push(*node);
                    }
                }

                if a_nodes.len() > input.G[ga] {
                    std::mem::swap(&mut ga, &mut gb);
                    std::mem::swap(&mut a_nodes, &mut b_nodes);
                }
                let mut cand = vec![];
                for a in 0..a_nodes.len() {
                    for b in 0..b_nodes.len() {
                        cand.push((self.order_map[&(a_nodes[a], b_nodes[b])], b_nodes[b]));
                    }
                }
                cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                let mut added_nodes = vec![];
                for (_, idx) in cand.iter() {
                    if added_nodes.len() == input.G[ga] - a_nodes.len() {
                        break;
                    }
                    if !added_nodes.contains(idx) {
                        added_nodes.push(*idx);
                    }
                }
                a_nodes.extend(added_nodes);
                b_nodes.retain(|node| !a_nodes.contains(node));
                assert!(a_nodes.len() == input.G[ga]);
                assert!(b_nodes.len() == input.G[gb]);

                // aのグループについて、最小全域木を構成
                let mut score_a = 0.0;
                let mut uf = UnionFind::new(a_nodes.len());
                let mut cand = vec![];
                for i in 0..a_nodes.len() {
                    for j in i + 1..a_nodes.len() {
                        cand.push((self.order_map[&(a_nodes[i], a_nodes[j])], i, j));
                    }
                }
                cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                for (_, i, j) in cand.iter() {
                    if uf.is_same(*i, *j) {
                        continue;
                    }
                    uf.unite(*i, *j);
                    score_a += dist[a_nodes[*i]][a_nodes[*j]];
                }
                // bのグループについて、最小全域木を構成
                let mut score_b = 0.0;
                let mut uf = UnionFind::new(b_nodes.len());
                let mut cand = vec![];
                for i in 0..b_nodes.len() {
                    for j in i + 1..b_nodes.len() {
                        cand.push((self.order_map[&(b_nodes[i], b_nodes[j])], i, j));
                    }
                }
                cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                for (_, i, j) in cand.iter() {
                    if uf.is_same(*i, *j) {
                        continue;
                    }
                    uf.unite(*i, *j);
                    score_b += dist[b_nodes[*i]][b_nodes[*j]];
                }

                let after_length = score_a + score_b;
                let diff_score = after_length - before_length;
                let T = T0 + (T1 - T0) * (get_time() / TLE);
                if diff_score < 0.0 || self.rng.gen_bool((-diff_score / T).exp()) {
                    lengths[ga] = score_a;
                    lengths[gb] = score_b;
                    self.group[ga] = a_nodes.clone();
                    self.group[gb] = b_nodes.clone();
                    updated_cnt += 1;

                    for idx in self.group[ga].iter() {
                        group_map[*idx] = ga;
                    }

                    for &i in a_nodes.iter() {
                        for &(_, j) in dist_order_by[i].iter() {
                            if group_map[i] != group_map[j] {
                                neighbor_group[group_map[i]].push(group_map[j]);
                                break;
                            }
                        }
                    }
                    neighbor_group[ga].sort();
                    neighbor_group[ga].dedup();

                    for idx in self.group[gb].iter() {
                        group_map[*idx] = gb;
                    }

                    for &i in b_nodes.iter() {
                        for &(_, j) in dist_order_by[i].iter() {
                            if group_map[i] != group_map[j] {
                                neighbor_group[group_map[i]].push(group_map[j]);
                                break;
                            }
                        }
                    }
                    neighbor_group[gb].sort();
                    neighbor_group[gb].dedup();
                }
            } else {
                let gc = cand_c[self.rng.gen_range(0..cand_c.len())];
                before_length += lengths[gc];
                let c_nodes = nodes;
                let nodes = self.group[gc].clone();

                let mut uf = UnionFind::new(nodes.len());
                let mut edges = vec![vec![]; input.N];
                let mut cand = vec![];
                for i in 0..nodes.len() {
                    for j in i + 1..nodes.len() {
                        cand.push((self.order_map[&(nodes[i], nodes[j])], i, j));
                    }
                }
                cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                for (_, i, j) in cand.iter() {
                    if uf.is_same(*i, *j) {
                        continue;
                    }
                    uf.unite(*i, *j);
                    edges[nodes[*i]].push(nodes[*j]);
                    edges[nodes[*j]].push(nodes[*i]);
                }

                let mut size = vec![0; input.N];
                for i in 0..nodes.len() {
                    size[nodes[i]] = 1;
                }
                let start = nodes[0];
                let mut used = vec![false; input.N];
                let mut parents = vec![!0; input.N];
                let mut children = vec![vec![]; input.N];
                used[start] = true;
                dfs(
                    start,
                    &edges,
                    &mut used,
                    &mut size,
                    &mut parents,
                    &mut children,
                );
                assert!(*size.iter().max().unwrap() == nodes.len());

                let mut indexes = size.iter().positions(|&s| s == input.G[ga]).collect_vec();
                indexes.sort_by(|a, b| {
                    if parents[*a] == !0 || parents[*b] == !0 {
                        Ordering::Equal
                    } else {
                        dist[*a][parents[*a]]
                            .partial_cmp(&dist[*b][parents[*b]])
                            .unwrap()
                    }
                });
                let idx = {
                    if indexes.len() > 0 {
                        indexes.pop().unwrap()
                    } else {
                        let mut idx = 0;
                        let mut diff = 100i32;
                        for i in 0..input.N {
                            let s = size[i];
                            if (input.G[ga] as i32 - s as i32).abs() < diff {
                                diff = (input.G[ga] as i32 - s as i32).abs();
                                idx = i;
                            }
                        }
                        idx
                    }
                };

                let mut a_nodes = vec![idx];
                let mut Q = VecDeque::new();
                Q.push_back(idx);
                while let Some(v) = Q.pop_front() {
                    for &u in children[v].iter() {
                        a_nodes.push(u);
                        Q.push_back(u);
                    }
                }
                let mut b_nodes = vec![];
                for node in nodes.iter() {
                    if !a_nodes.contains(node) {
                        b_nodes.push(*node);
                    }
                }

                if a_nodes.len() > input.G[ga] {
                    std::mem::swap(&mut ga, &mut gb);
                    std::mem::swap(&mut a_nodes, &mut b_nodes);
                }
                let mut cand = vec![];
                for a in 0..a_nodes.len() {
                    for b in 0..b_nodes.len() {
                        cand.push((self.order_map[&(a_nodes[a], b_nodes[b])], b_nodes[b]));
                    }
                }
                cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                let mut added_nodes = vec![];
                for (_, idx) in cand.iter() {
                    if added_nodes.len() == input.G[ga] - a_nodes.len() {
                        break;
                    }
                    if !added_nodes.contains(idx) {
                        added_nodes.push(*idx);
                    }
                }
                a_nodes.extend(added_nodes);
                b_nodes.retain(|node| !a_nodes.contains(node));
                assert!(a_nodes.len() == input.G[ga]);
                assert!(b_nodes.len() == input.G[gb]);

                // aのグループについて、最小全域木を構成
                let mut score_a = 0.0;
                let mut uf = UnionFind::new(a_nodes.len());
                let mut cand = vec![];
                for i in 0..a_nodes.len() {
                    for j in i + 1..a_nodes.len() {
                        cand.push((self.order_map[&(a_nodes[i], a_nodes[j])], i, j));
                    }
                }
                cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                for (_, i, j) in cand.iter() {
                    if uf.is_same(*i, *j) {
                        continue;
                    }
                    uf.unite(*i, *j);
                    score_a += dist[a_nodes[*i]][a_nodes[*j]];
                }
                // bのグループについて、最小全域木を構成
                let mut score_b = 0.0;
                let mut uf = UnionFind::new(b_nodes.len());
                let mut cand = vec![];
                for i in 0..b_nodes.len() {
                    for j in i + 1..b_nodes.len() {
                        cand.push((self.order_map[&(b_nodes[i], b_nodes[j])], i, j));
                    }
                }
                cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                for (_, i, j) in cand.iter() {
                    if uf.is_same(*i, *j) {
                        continue;
                    }
                    uf.unite(*i, *j);
                    score_b += dist[b_nodes[*i]][b_nodes[*j]];
                }

                let after_length = score_a + score_b + score_c;
                let diff_score = after_length - before_length;
                let T = T0 + (T1 - T0) * (get_time() / TLE);
                if diff_score < 0.0 || self.rng.gen_bool((-diff_score / T).exp()) {
                    lengths[ga] = score_a;
                    lengths[gb] = score_b;
                    lengths[gc] = score_c;
                    self.group[ga] = a_nodes.clone();
                    self.group[gb] = b_nodes.clone();
                    self.group[gc] = c_nodes.clone();
                    updated_cnt += 1;

                    for idx in self.group[ga].iter() {
                        group_map[*idx] = ga;
                    }

                    for &i in a_nodes.iter() {
                        for &(_, j) in dist_order_by[i].iter() {
                            if group_map[i] != group_map[j] {
                                neighbor_group[group_map[i]].push(group_map[j]);
                                break;
                            }
                        }
                    }
                    neighbor_group[ga].sort();
                    neighbor_group[ga].dedup();

                    for idx in self.group[gb].iter() {
                        group_map[*idx] = gb;
                    }

                    for &i in b_nodes.iter() {
                        for &(_, j) in dist_order_by[i].iter() {
                            if group_map[i] != group_map[j] {
                                neighbor_group[group_map[i]].push(group_map[j]);
                                break;
                            }
                        }
                    }
                    neighbor_group[gb].sort();
                    neighbor_group[gb].dedup();

                    for idx in self.group[gc].iter() {
                        group_map[*idx] = gc;
                    }

                    for &i in nodes.iter() {
                        for &(_, j) in dist_order_by[i].iter() {
                            if group_map[i] != group_map[j] {
                                neighbor_group[group_map[i]].push(group_map[j]);
                                break;
                            }
                        }
                    }
                    neighbor_group[gc].sort();
                    neighbor_group[gc].dedup();
                }
            }
        }
        eprintln!("updated_cnt = {}", updated_cnt);
        eprintln!("iter = {}", iter);
    }
    pub fn get_score(&self, dist: &Vec<Vec<f64>>) -> f64 {
        let mut score = 0.0;
        for group in self.group.iter() {
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
                score += dist[*a][*b];
            }
        }
        score
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

fn dfs(
    v: usize,
    edges: &Vec<Vec<usize>>,
    used: &mut Vec<bool>,
    size: &mut Vec<usize>,
    parents: &mut Vec<usize>,
    children: &mut Vec<Vec<usize>>,
) -> usize {
    for &u in edges[v].iter() {
        if used[u] {
            continue;
        }
        used[u] = true;
        parents[u] = v;
        children[v].push(u);
        size[v] += dfs(u, edges, used, size, parents, children);
    }
    size[v]
}
