use std::collections::{BTreeSet, BinaryHeap, VecDeque};

use itertools::Itertools;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{common::get_time, input::Input};

#[derive(Debug, Clone)]
pub struct Node {
    pub num: usize,
    pub h: i64,
    pub parent: i64,
    pub children: Vec<usize>,
    pub sum_A: i64,
    pub hmax: i64,
}

impl Node {
    pub fn is_root(&self) -> bool {
        self.parent == -1
    }
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub score: i64,
    pub nodes: Vec<Node>,
}

impl State {
    pub fn new(input: &Input) -> Self {
        Self {
            score: input.A.iter().sum::<i64>() + 1,
            nodes: (0..input.N)
                .map(|i| Node {
                    num: i,
                    h: 0,
                    parent: -1,
                    children: vec![],
                    sum_A: input.A[i] as i64,
                    hmax: 0,
                })
                .collect(),
        }
    }
    pub fn dfs(&mut self, input: &Input) {
        let mut used = vec![false; input.N];
        let mut ans = vec![-1; input.N];
        let mut score = 1;
        let mut order = input
            .A
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, a)| (a, i))
            .collect_vec();
        order.sort();

        for (_, root) in order.iter() {
            if used[*root] {
                continue;
            }
            let mut Q = vec![];
            Q.push((*root, 0));
            score += input.A[*root];
            used[*root] = true;
            while let Some((pos, h)) = Q.pop() {
                if h == input.H {
                    continue;
                }
                for nxt in input.G[pos].iter() {
                    if used[*nxt] {
                        continue;
                    }
                    Q.push((*nxt, h + 1));
                    score += input.A[*nxt] * (h as i64 + 2);
                    used[*nxt] = true;
                    ans[*nxt] = pos as i32;
                }
            }
        }
        eprintln!("Score = {}", score);
        println!("{}", ans.iter().join(" "));
    }
    pub fn greedy_dfs(&mut self, input: &Input) {
        let mut used = vec![false; input.N];
        let mut used_cnt = 0;
        let mut ans = vec![-1; input.N];
        let mut score = 1;

        while used_cnt < input.N {
            let mut cands = vec![];
            for root in 0..input.N {
                if used[root] {
                    continue;
                }
                let mut used_part = used.clone();
                let mut used_part_cnt = 0;
                let mut score_part = 0;
                let mut Q = vec![];
                Q.push((root, 0));
                used_part[root] = true;
                used_part_cnt += 1;
                score_part += input.A[root];
                while let Some((pos, h)) = Q.pop() {
                    if h == input.H {
                        continue;
                    }
                    for nxt in input.G[pos].iter() {
                        if used_part[*nxt] {
                            continue;
                        }
                        Q.push((*nxt, h + 1));
                        used_part[*nxt] = true;
                        used_part_cnt += 1;
                        score_part += input.A[*nxt] * (h as i64 + 2);
                    }
                }
                cands.push((score_part as f64 / used_part_cnt as f64, root));
            }
            cands.sort_by(|a, b| b.partial_cmp(a).unwrap());
            let root = cands[0].1;
            let mut Q = vec![];
            Q.push((root, 0));
            used[root] = true;
            used_cnt += 1;
            score += input.A[root];
            while let Some((pos, h)) = Q.pop() {
                if h == input.H {
                    continue;
                }
                for nxt in input.G[pos].iter() {
                    if used[*nxt] {
                        continue;
                    }
                    Q.push((*nxt, h + 1));
                    used[*nxt] = true;
                    used_cnt += 1;
                    score += input.A[*nxt] * (h as i64 + 2);
                    ans[*nxt] = pos as i32;
                }
            }
        }

        self.score = score;
        for (c, p) in ans.iter().enumerate() {
            if *p != -1 {
                self.nodes[*p as usize].children.push(c);
            }
            self.nodes[c].parent = *p as i64;
        }
        for i in 0..input.N {
            if self.nodes[i].is_root() {
                let mut Q = vec![];
                let mut leafs = vec![];
                Q.push((i, 0));
                while let Some((id, h)) = Q.pop() {
                    self.nodes[id].h = h;
                    if self.nodes[id].is_leaf() {
                        leafs.push(id);
                    }
                    for child_id in self.nodes[id].children.iter() {
                        Q.push((*child_id, h + 1));
                    }
                }

                let mut Q = BinaryHeap::new();
                let mut used = BTreeSet::new();
                for id in leafs.iter() {
                    self.nodes[*id].hmax = self.nodes[*id].h;
                    self.nodes[*id].sum_A = input.A[*id] as i64;
                    Q.push((self.nodes[*id].h, *id));
                    used.insert(*id);
                }
                while let Some((_, id)) = Q.pop() {
                    if !self.nodes[id].is_root() {
                        let parent_id = self.nodes[id].parent as usize;
                        self.nodes[parent_id].hmax =
                            self.nodes[parent_id].hmax.max(self.nodes[id].hmax);
                        self.nodes[parent_id].sum_A += self.nodes[id].sum_A;
                        if used.contains(&parent_id) {
                            continue;
                        }
                        used.insert(parent_id);
                        Q.push((self.nodes[parent_id].h, parent_id));
                    }
                }
            }
        }
    }
    pub fn greedy(&mut self, input: &Input) {
        let mut used = vec![false; input.N];
        let mut used_cnt = 0;
        let mut ans = vec![-1; input.N];
        let mut score = 1;
        let mut order = input
            .A
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, a)| (a, i))
            .collect_vec();
        order.sort();

        while used_cnt < input.N {
            let root = {
                let mut ret = !0;
                for (_, i) in order.iter() {
                    if !used[*i] {
                        ret = *i;
                        break;
                    }
                }
                ret
            };

            let mut Q = VecDeque::new();
            let mut routes = vec![];
            let route = vec![root];
            let remain = 5;
            Q.push_back(route);
            while let Some(route) = Q.pop_front() {
                if route.len() == input.H + 1 - remain {
                    routes.push(route);
                    continue;
                }
                let pos = *route.last().unwrap();
                let mut exists = false;
                for nxt in input.G[pos].iter() {
                    if used[*nxt] || route.contains(nxt) {
                        continue;
                    }
                    exists = true;
                    let mut next_route = route.clone();
                    next_route.push(*nxt);
                    Q.push_back(next_route);
                }
                if !exists {
                    routes.push(route);
                }
            }

            let mut best_part_score = 0;
            let mut best_part_ans = vec![];
            let mut best_used_cnt = 0;

            for route in routes.iter() {
                let mut part_used = used.clone();
                let mut part_score = input.A[root];
                let mut part_ans = vec![(root, -1)];
                let mut part_used_cnt = 1;
                part_used[root] = true;
                for i in 1..route.len() {
                    part_score += input.A[route[i]] * (i as i64 + 1);
                    part_used[route[i]] = true;
                    part_ans.push((route[i], route[i - 1] as i32));
                    part_used_cnt += 1;
                }

                let mut now = vec![route[route.len() - 1]];
                for r in 0..remain {
                    let mut next = vec![];
                    for i in now.iter() {
                        for nxt in input.G[*i].iter() {
                            if part_used[*nxt] {
                                continue;
                            }
                            part_used[*nxt] = true;
                            part_used_cnt += 1;
                            part_score += input.A[*nxt] * (route.len() as i64 + r as i64 + 1);
                            part_ans.push((*nxt, *i as i32));
                            next.push(*nxt);
                        }
                    }
                    now = next;
                }
                if part_score > best_part_score {
                    best_part_score = part_score;
                    best_part_ans = part_ans;
                    best_used_cnt = part_used_cnt;
                }
            }
            score += best_part_score;
            used_cnt += best_used_cnt;
            for (c, p) in best_part_ans.iter() {
                used[*c] = true;
                ans[*c] = *p;
            }
        }

        self.score = score;
        for (c, p) in ans.iter().enumerate() {
            if *p != -1 {
                self.nodes[*p as usize].children.push(c);
            }
            self.nodes[c].parent = *p as i64;
        }
        for i in 0..input.N {
            if self.nodes[i].is_root() {
                let mut Q = vec![];
                let mut leafs = vec![];
                Q.push((i, 0));
                while let Some((id, h)) = Q.pop() {
                    self.nodes[id].h = h;
                    if self.nodes[id].is_leaf() {
                        leafs.push(id);
                    }
                    for child_id in self.nodes[id].children.iter() {
                        Q.push((*child_id, h + 1));
                    }
                }

                let mut Q = BinaryHeap::new();
                let mut used = BTreeSet::new();
                for id in leafs.iter() {
                    self.nodes[*id].hmax = self.nodes[*id].h;
                    self.nodes[*id].sum_A = input.A[*id] as i64;
                    Q.push((self.nodes[*id].h, *id));
                    used.insert(*id);
                }
                while let Some((_, id)) = Q.pop() {
                    if !self.nodes[id].is_root() {
                        let parent_id = self.nodes[id].parent as usize;
                        self.nodes[parent_id].hmax =
                            self.nodes[parent_id].hmax.max(self.nodes[id].hmax);
                        self.nodes[parent_id].sum_A += self.nodes[id].sum_A;
                        if used.contains(&parent_id) {
                            continue;
                        }
                        used.insert(parent_id);
                        Q.push((self.nodes[parent_id].h, parent_id));
                    }
                }
            }
        }
    }
    pub fn annealing(&mut self, input: &Input) {
        let tle = 1.95;
        let T0 = 100.0;
        let T1 = 0.1;
        let mut rng = Pcg64Mcg::new(100);
        let mut iter = 0;
        let mut valid_iter = 0;
        let mut update_iter = 0;

        while get_time() < tle {
            let node_id = rng.gen_range(0..self.nodes.len());
            let neighbor_id = input.G[node_id][rng.gen_range(0..input.G[node_id].len())];
            iter += 1;
            if !self.is_valid(&self.nodes[node_id], &self.nodes[neighbor_id], input) {
                continue;
            }
            valid_iter += 1;
            let diff_score = self.calc_diff_score(&self.nodes[node_id], &self.nodes[neighbor_id]);
            let temp = T0 + (T1 - T0) * get_time() / tle;
            if diff_score > 0 || rng.gen_bool((diff_score as f64 / temp).exp()) {
                if self.is_loop(&self.nodes[node_id], &self.nodes[neighbor_id]) {
                    continue;
                }
                update_iter += 1;
                self.score += diff_score;

                // 付け替え元の親とその祖先の更新
                if !self.nodes[node_id].is_root() {
                    let mut parent_id = self.nodes[node_id].parent as usize;
                    // 付け替え元の親の子からnode_idを削除
                    self.nodes[parent_id].children.retain(|&x| x != node_id);
                    while parent_id != !0 {
                        self.nodes[parent_id].sum_A -= self.nodes[node_id].sum_A;
                        self.nodes[parent_id].hmax = self.nodes[parent_id]
                            .children
                            .iter()
                            .map(|&id| self.nodes[id].hmax)
                            .max()
                            .unwrap_or(self.nodes[parent_id].h);
                        parent_id = self.nodes[parent_id].parent as usize;
                    }
                }

                // 付け替えるNodeとその子孫の更新
                self.nodes[node_id].parent = neighbor_id as i64;
                let diff_h = (self.nodes[neighbor_id].h + 1) - self.nodes[node_id].h;
                let mut change_node_ids = vec![node_id];
                while let Some(id) = change_node_ids.pop() {
                    self.nodes[id].h += diff_h;
                    self.nodes[id].hmax += diff_h;
                    for child_id in self.nodes[id].children.iter() {
                        change_node_ids.push(*child_id);
                    }
                }

                // 付け替え先の親とその祖先の更新
                self.nodes[neighbor_id].children.push(node_id);
                let mut parent_id = neighbor_id;
                while parent_id != !0 {
                    self.nodes[parent_id].sum_A += self.nodes[node_id].sum_A;
                    self.nodes[parent_id].hmax = self.nodes[parent_id]
                        .children
                        .iter()
                        .map(|&id| self.nodes[id].hmax)
                        .max()
                        .unwrap_or(self.nodes[parent_id].h);
                    parent_id = self.nodes[parent_id].parent as usize;
                }
            }
        }
        eprintln!("iter = {}", iter);
        eprintln!("valid_iter = {}", valid_iter);
        eprintln!("update_iter = {}", update_iter);
    }
    pub fn is_valid(&self, child: &Node, parent: &Node, input: &Input) -> bool {
        // 親の変更なし
        if child.parent == parent.num as i64 {
            return false;
        }
        // 高さ制約違反
        if child.hmax + (parent.h + 1) - child.h > input.H as i64 {
            return false;
        }
        true
    }
    pub fn is_loop(&self, child: &Node, parent: &Node) -> bool {
        let mut Q = VecDeque::new();
        Q.push_back(child.num);
        while let Some(id) = Q.pop_front() {
            if parent.num == id {
                return true;
            }
            for child_id in self.nodes[id].children.iter() {
                Q.push_back(*child_id);
            }
        }
        false
    }
    pub fn calc_diff_score(&self, child: &Node, parent: &Node) -> i64 {
        let diff_h = (parent.h + 1) - child.h;
        diff_h * child.sum_A as i64
    }
    pub fn output(&self) {
        // eprintln!("Score = {}", self.score);
        let ans = self
            .nodes
            .iter()
            .map(|node| node.parent)
            .collect::<Vec<_>>();
        println!("{}", ans.iter().join(" "));
    }
}
