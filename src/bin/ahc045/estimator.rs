use itertools::Itertools;
use proconio::input_interactive;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use rustc_hash::FxHashSet;

use crate::{
    common::{eprint_blue, get_time},
    coord::Coord,
    input::Input,
};

pub struct Estimator {
    rng: Pcg64Mcg,
    pub positions: Vec<Coord>,
    query_nodes: Vec<Vec<usize>>,
    mst_edges: Vec<Vec<(usize, usize)>>,
    ineqs: Vec<Inequality>,
}

fn query(nodes: &Vec<usize>) -> Vec<(usize, usize)> {
    println!("? {} {}", nodes.len(), nodes.iter().join(" "));
    input_interactive! {
        uv: [(usize, usize); nodes.len() - 1],
    }
    uv
}

impl Estimator {
    pub fn new(input: &Input) -> Self {
        Self {
            rng: Pcg64Mcg::new(100),
            positions: input.rects.iter().map(|r| r.center()).collect(),
            query_nodes: vec![],
            mst_edges: vec![],
            ineqs: vec![],
        }
    }
    pub fn measure(&mut self, input: &Input) {
        // 誤差降順にソート
        let order_by_error = (0..input.N)
            .map(|i| (input.rects[i].long_side(), i))
            .sorted_by_key(|(err, _)| *err)
            .rev()
            .take(input.Q)
            .map(|(_, i)| i)
            .collect_vec();

        for idx in order_by_error {
            // 距離が短い順に点の候補をソート
            let cand = (0..input.N)
                .map(|i| (self.positions[idx].euclidean_dist(self.positions[i]), i))
                .sorted_by_key(|(d, _)| *d)
                .map(|(_, i)| i)
                .collect_vec();
            let query_nodes = cand.iter().take(input.L).copied().collect_vec();
            let mut_edges = query(&query_nodes);
            self.query_nodes.push(query_nodes);
            self.mst_edges.push(mut_edges);
        }
    }
    pub fn get_ineqs(&mut self, input: &Input) {
        let mut G = vec![vec![]; input.N];
        for (query_nodes, mst_edges) in self.query_nodes.iter().zip(self.mst_edges.iter()) {
            // MSTに含まれない辺を列挙
            let mut not_mst_edges = vec![];
            for i in 0..query_nodes.len() {
                for j in i + 1..query_nodes.len() {
                    let u = query_nodes[i].min(query_nodes[j]);
                    let v = query_nodes[i].max(query_nodes[j]);
                    if !mst_edges.contains(&(u, v)) {
                        not_mst_edges.push((u, v));
                    }
                }
            }

            // 使い回すために初期化
            for &idx in query_nodes.iter() {
                G[idx].clear();
            }
            for &(u, v) in mst_edges {
                G[u].push(v);
                G[v].push(u);
            }

            // MSTに含まれない辺の頂点間のパスを取得
            // このパスに含まれる辺はMSTに含まれない辺よりも短い
            for (start, goal) in not_mst_edges {
                let long = (start, goal);
                let mut path = vec![start];
                let mut visited = FxHashSet::default();
                visited.insert(start);
                dfs(start, goal, &G, &mut visited, &mut path);
                for i in 0..path.len() - 1 {
                    let short = (path[i], path[i + 1]);
                    self.ineqs.push(Inequality::new(short, long));
                }
            }
        }
    }
    pub fn filter_ineqs(&mut self, input: &Input) {
        // 矩形のどこにあっても常に不等式を満たすものを除外
        self.ineqs.retain(|ineq| !ineq.has_no_error(input));

        // 不等式を満たさない可能性が低いものを除外
        let random_num = 20;
        let random_positions = (0..random_num)
            .map(|_| {
                (0..input.N)
                    .map(|i| input.rects[i].random_coord(&mut self.rng))
                    .collect_vec()
            })
            .collect_vec();
        self.ineqs.retain(|ineq| {
            let short_u = ineq.short.0;
            let short_v = ineq.short.1;
            let long_u = ineq.long.0;
            let long_v = ineq.long.1;
            for i in 0..random_num {
                let short_dist =
                    random_positions[i][short_u].euclidean_dist(random_positions[i][short_v]);
                let long_dist =
                    random_positions[i][long_u].euclidean_dist(random_positions[i][long_v]);
                if short_dist > long_dist {
                    return true;
                }
            }
            false
        });
    }
    pub fn get_ids(&mut self, input: &Input) -> Vec<Vec<usize>> {
        // 各点に関連する不等式のidを管理
        let mut ids = vec![vec![]; input.N];
        for (i, ineq) in self.ineqs.iter().enumerate() {
            ids[ineq.short.0].push(i);
            ids[ineq.short.1].push(i);
            ids[ineq.long.0].push(i);
            ids[ineq.long.1].push(i);
        }
        for i in 0..input.N {
            ids[i].sort();
            ids[i].dedup();
        }
        ids
    }
    pub fn climbing_random(&mut self, input: &Input, tle: f64) {
        let ids = self.get_ids(input);
        let mut crt = self
            .ineqs
            .iter()
            .filter(|ineq| ineq.is_error(&self.positions))
            .count() as i64;
        eprint_blue(&format!("estimator climbing random crt = {}", crt));

        let mut iter = 0;
        loop {
            let elapsed = get_time();
            if elapsed > tle || crt == 0 {
                break;
            }

            let idx = self.rng.gen_range(0..input.N);
            if ids[idx].is_empty() {
                continue;
            }
            let before_pos = self.positions[idx];
            let next_pos = input.rects[idx].random_coord(&mut self.rng);
            let before_error = ids[idx]
                .iter()
                .filter(|&&id| self.ineqs[id].is_error(&self.positions))
                .count();

            self.positions[idx] = next_pos;
            let after_error = ids[idx]
                .iter()
                .filter(|&&id| self.ineqs[id].is_error(&self.positions))
                .count();

            let diff = after_error as i64 - before_error as i64;
            if diff <= 0 {
                crt += diff;
            } else {
                self.positions[idx] = before_pos;
            }
            iter += 1;
        }

        eprint_blue(&format!("estimator climbing random iter = {}", iter));
        eprint_blue(&format!("estimator climbing random crt = {}", crt));
    }
}

fn dfs(
    u: usize,
    goal: usize,
    G: &Vec<Vec<usize>>,
    visited: &mut FxHashSet<usize>,
    path: &mut Vec<usize>,
) -> bool {
    if u == goal {
        return true;
    }
    for &v in G[u].iter() {
        if !visited.contains(&v) {
            visited.insert(v);
            path.push(v);
            if dfs(v, goal, G, visited, path) {
                return true;
            }
            visited.remove(&v);
            path.pop();
        }
    }
    false
}

pub struct Inequality {
    pub short: (usize, usize),
    pub long: (usize, usize),
}

impl Inequality {
    fn new(short: (usize, usize), long: (usize, usize)) -> Self {
        Self { short, long }
    }
    fn is_error(&self, xy: &Vec<Coord>) -> bool {
        xy[self.short.0].euclidean_dist(xy[self.short.1])
            > xy[self.long.0].euclidean_dist(xy[self.long.1])
    }
    fn swap_short_nodes(&mut self) {
        std::mem::swap(&mut self.short.0, &mut self.short.1);
    }
    fn swap_long_nodes(&mut self) {
        std::mem::swap(&mut self.long.0, &mut self.long.1);
    }
    fn calc_gradient_short(&self, xy: &Vec<Coord>) -> (f64, f64) {
        let length = xy[self.short.0].euclidean_dist(xy[self.short.1]) as f64;
        let dx = xy[self.short.1].x as f64 - xy[self.short.0].x as f64;
        let dy = xy[self.short.1].y as f64 - xy[self.short.0].y as f64;
        (dx / length, dy / length)
    }
    fn calc_gradient_long(&self, xy: &Vec<Coord>) -> (f64, f64) {
        let length = xy[self.long.0].euclidean_dist(xy[self.long.1]) as f64;
        let dx = xy[self.long.1].x as f64 - xy[self.long.0].x as f64;
        let dy = xy[self.long.1].y as f64 - xy[self.long.0].y as f64;
        (-dx / length, -dy / length)
    }
    fn has_no_error(&self, input: &Input) -> bool {
        let long_min_dist = input.rects[self.long.0].min_dist(&input.rects[self.long.1]);
        let short_max_dist = input.rects[self.short.0].max_dist(&input.rects[self.short.1]);
        short_max_dist < long_min_dist
    }
}
