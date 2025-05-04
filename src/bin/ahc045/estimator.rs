use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use itertools::Itertools;
use proconio::input_interactive;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use rustc_hash::FxHashSet;

use crate::{common::get_time, coord::Coord, input::Input};

pub struct Estimator {
    rng: Pcg64Mcg,
    input: Input,
    pub xy: Vec<Coord>,
    pub dist: Vec<Vec<usize>>,
    queries: Vec<Vec<usize>>,
    mst_edges: Vec<Vec<(usize, usize)>>,
    inequalities: Vec<Inequality>,
}

impl Estimator {
    pub fn new(input: &Input) -> Self {
        // 矩形の中心を推定初期座標とする
        let xy = input.rects.iter().map(|rect| rect.center()).collect_vec();
        let mut dist = vec![vec![0; input.N]; input.N];
        for i in 0..input.N {
            for j in 0..input.N {
                dist[i][j] = xy[i].euclidean_dist(xy[j]);
                dist[j][i] = dist[i][j];
            }
        }
        Self {
            rng: Pcg64Mcg::new(100),
            input: input.clone(),
            xy,
            dist,
            queries: vec![],
            mst_edges: vec![],
            inequalities: vec![],
        }
    }
    pub fn triangle_query(&mut self) {
        let nodes_sorted_by_error = (0..self.input.N)
            .sorted_by_key(|&i| self.input.rects[i].long_side())
            .rev()
            .collect_vec();

        let mut used_cnt = self
            .input
            .rects
            .iter()
            .map(|rect| -(rect.long_side() as isize / 1000))
            .collect_vec();

        for &first_node_idx in nodes_sorted_by_error.iter() {
            let mut query_nodes = vec![first_node_idx];
            let mut used_edges = FxHashSet::default();
            get_query_nodes(
                &mut query_nodes,
                &mut used_edges,
                &used_cnt,
                &self.xy,
                &self.input,
                &mut self.rng,
            );
            if query_nodes.len() != self.input.L {
                continue;
            }
            for node_idx in query_nodes.iter() {
                used_cnt[*node_idx] += 1;
            }
            println!("? {} {}", query_nodes.len(), query_nodes.iter().join(" "));
            input_interactive! {
                uv: [(usize, usize); query_nodes.len() - 1],
            }
            self.mst_edges.push(uv);
            self.queries.push(query_nodes);
            if self.mst_edges.len() == self.input.Q {
                break;
            }
        }
    }
    pub fn three_node_query(&mut self) {
        let delta = 100; // 3点目の座標の誤差範囲
        let mut used = FxHashSet::default();
        let nodes_sorted_by_error = (0..self.input.N)
            .sorted_by_key(|&i| self.input.rects[i].long_side())
            .rev()
            .collect_vec();

        for &base_idx in nodes_sorted_by_error.iter() {
            let mut points = vec![base_idx];
            'outer: loop {
                // 1点しかない場合はランダムで2点目を選択
                if points.len() == 1 {
                    let second_idx = self.rng.gen_range(0..self.input.N);
                    if base_idx == second_idx {
                        continue;
                    }
                    points.push(second_idx);
                }

                // 3点目以降の点座標の候補
                let mut coord_center_cand = vec![];
                for i in 0..points.len() {
                    for j in 0..points.len() {
                        if i == j {
                            continue;
                        }
                        coord_center_cand
                            .push(self.xy[points[i]].rotate_120deg(self.xy[points[j]]));
                    }
                }
                let coord_center_cand = coord_center_cand
                    .into_iter()
                    .filter(|coord| coord.is_some())
                    .map(|coord| coord.unwrap())
                    .sorted()
                    .dedup()
                    .collect::<Vec<_>>();
                if coord_center_cand.is_empty() {
                    points.pop();
                    continue;
                }

                let mut cand = vec![];
                for next_coord_center in coord_center_cand.iter() {
                    let x_lower = next_coord_center.x.saturating_sub(delta / 2);
                    let x_upper = (next_coord_center.x + delta / 2).min(10000);
                    let y_lower = next_coord_center.y.saturating_sub(delta / 2);
                    let y_upper = (next_coord_center.y + delta / 2).min(10000);
                    assert!(x_lower <= x_upper);
                    assert!(y_lower <= y_upper);
                    let x_range = self.input.x_positions.range(x_lower..=x_upper);
                    let y_range = self.input.y_positions.range(y_lower..=y_upper);
                    let x_range_points: FxHashSet<usize> = x_range
                        .map(|(_, indices)| indices)
                        .flatten()
                        .cloned()
                        .collect();
                    let y_range_points: FxHashSet<usize> = y_range
                        .map(|(_, indices)| indices)
                        .flatten()
                        .cloned()
                        .collect();
                    cand.extend(x_range_points.intersection(&y_range_points));
                }

                'cand_loop: for &next_idx in cand.iter() {
                    if points.contains(&next_idx) {
                        continue 'cand_loop;
                    }
                    points.push(next_idx);

                    for k in 0..points.len() {
                        for l in k + 1..points.len() {
                            let mut idx0 = points[k];
                            let mut idx1 = points[l];
                            if idx0 > idx1 {
                                std::mem::swap(&mut idx0, &mut idx1);
                            }
                            if used.contains(&(idx0, idx1)) {
                                points.pop();
                                continue 'cand_loop;
                            }
                        }
                    }
                    for k in 0..points.len() {
                        for l in k + 1..points.len() {
                            used.insert((points[k], points[l]));
                        }
                    }
                    if points.len() == self.input.L {
                        println!("? {} {}", points.len(), points.iter().join(" "));
                        input_interactive! {
                            uv: [(usize, usize); points.len() - 1],
                        }
                        self.mst_edges.push(uv);
                        self.queries.push(points);
                        break 'outer;
                    }
                }
                points.pop();
            }
            if self.queries.len() == self.input.Q {
                break;
            }
        }
    }
    pub fn neighbor_query(&mut self) {
        let grid_num = 20;
        let delta = self.input.width / grid_num;
        let grid_centers = (0..=grid_num).map(|i| i * delta).collect::<Vec<_>>();
        let mut empty_cnt = 0;
        let nodes_sorted_by_error = (0..self.input.N)
            .sorted_by_key(|&i| self.input.rects[i].long_side())
            .rev()
            .collect_vec();

        for xi in 0..grid_num {
            for yi in 0..grid_num {
                let x_lower = grid_centers[xi];
                let x_upper = grid_centers[xi + 1];
                let y_lower = grid_centers[yi];
                let y_upper = grid_centers[yi + 1];
                let x_range = self.input.x_positions.range(x_lower..=x_upper);
                let y_range = self.input.y_positions.range(y_lower..=y_upper);
                let mut x_range_points = FxHashSet::default();
                for (_, indices) in x_range {
                    for &idx in indices {
                        x_range_points.insert(idx);
                    }
                }
                let mut y_range_points = FxHashSet::default();
                for (_, indices) in y_range {
                    for &idx in indices {
                        y_range_points.insert(idx);
                    }
                }
                let mut nodes = x_range_points
                    .intersection(&y_range_points)
                    .into_iter()
                    .sorted_by_key(|&idx| self.input.rects[*idx].long_side())
                    .rev()
                    .take(self.input.L)
                    .cloned()
                    .collect::<FxHashSet<_>>();
                if nodes.is_empty() {
                    nodes.insert(nodes_sorted_by_error[empty_cnt]);
                    empty_cnt += 1;
                }

                let mut Q = BinaryHeap::new();
                let mut dist = vec![1 << 60; self.input.N];
                for &node in nodes.iter() {
                    Q.push((Reverse(0), node));
                    dist[node] = 0;
                }
                while let Some((Reverse(d), u)) = Q.pop() {
                    if nodes.len() == self.input.L {
                        break;
                    }
                    nodes.insert(u);
                    for v in 0..self.input.N {
                        if nodes.contains(&v) {
                            continue;
                        }
                        let nd = self.xy[u].euclidean_dist(self.xy[v]);
                        if d + nd < dist[v] {
                            dist[v] = d + nd;
                            Q.push((Reverse(d + nd), v));
                        }
                    }
                }
                assert!(nodes.is_empty() || nodes.len() == self.input.L);
                if !nodes.is_empty() {
                    let query_nodes = nodes.into_iter().collect::<Vec<_>>();
                    println!("? {} {}", query_nodes.len(), query_nodes.iter().join(" "));
                    input_interactive! {
                        uv: [(usize, usize); query_nodes.len() - 1],
                    }
                    self.mst_edges.push(uv);
                    self.queries.push(query_nodes);
                }
            }
        }
    }
    pub fn get_inequality(&mut self) {
        eprintln!("query num = {}", self.queries.len());
        for (nodes, uv) in self.queries.iter().zip(self.mst_edges.iter()) {
            let nodes = nodes.iter().cloned().collect::<Vec<_>>();

            let mut idx_uv = vec![];
            for &(u, v) in uv.iter() {
                let u_idx = nodes.iter().position(|&p| p == u).unwrap();
                let v_idx = nodes.iter().position(|&p| p == v).unwrap();
                idx_uv.push((u_idx, v_idx));
            }

            let mut cycles = get_circle_edges(nodes.len(), &idx_uv);
            for cycle in cycles.iter_mut() {
                for edge in cycle.iter_mut() {
                    to_node_idx(edge, &nodes);
                }
            }
            for cycle in cycles.iter() {
                let long = cycle[0];
                for &short in cycle.iter().skip(1) {
                    self.inequalities.push(Inequality::new(short, long));
                }
            }
        }

        self.inequalities.sort();
        self.inequalities.dedup();
        eprintln!("inequalities num = {}", self.inequalities.len());
        self.inequalities
            .retain(|ineq| !ineq.has_no_error(&self.input));
        eprintln!("inequalities num = {}", self.inequalities.len());
    }
    pub fn climbing(&mut self, TLE: f64) {
        let mut true_dist = vec![vec![0; self.input.N]; self.input.N];
        for i in 0..self.input.N {
            for j in 0..self.input.N {
                true_dist[i][j] = self.input.xy[i].euclidean_dist(self.input.xy[j]);
            }
        }

        let mut before_dist_diff = 0;
        for i in 0..self.input.N {
            for j in i + 1..self.input.N {
                before_dist_diff += if true_dist[i][j] >= self.dist[i][j] {
                    true_dist[i][j] - self.dist[i][j]
                } else {
                    self.dist[i][j] - true_dist[i][j]
                };
            }
        }

        let before_ineq_error_num = self
            .inequalities
            .iter()
            .filter(|ineq| ineq.is_error_by_dist(&self.dist))
            .count();

        let deltas = self
            .input
            .rects
            .iter()
            .map(|rect| {
                (
                    (rect.x_max - rect.x_min) as f64,
                    (rect.y_max - rect.y_min) as f64,
                )
            })
            .collect::<Vec<_>>();

        let mut rng = Pcg64Mcg::new(100);
        let mut iter = 0;
        let mut updated_cnt = 0;
        let ineq_num = self.inequalities.len();
        let start_learning_rate = 0.1;
        let end_learning_rate = 0.02;

        loop {
            let elapsed_time = get_time();
            if elapsed_time > TLE {
                break;
            }

            let learning_rate = start_learning_rate
                + (end_learning_rate - start_learning_rate) * elapsed_time / TLE;

            iter += 1;
            let idx = rng.gen_range(0..ineq_num);
            if !self.inequalities[idx].is_error_by_dist(&self.dist) {
                continue;
            }

            if rng.gen_bool(0.5) {
                self.inequalities[idx].swap_short_nodes();
            }
            if rng.gen_bool(0.5) {
                self.inequalities[idx].swap_long_nodes();
            }

            let ineq = &self.inequalities[idx];

            // short
            let (dx, dy) = ineq.calc_gradient_short(&self.xy, &self.dist);
            let (dx, dy) = (
                dx * deltas[ineq.short.0].0 * learning_rate,
                dy * deltas[ineq.short.0].1 * learning_rate,
            );
            let x = (self.xy[ineq.short.0].x as f64 + dx).clamp(
                self.input.rects[ineq.short.0].x_min as f64,
                self.input.rects[ineq.short.0].x_max as f64,
            ) as usize;
            let y = (self.xy[ineq.short.0].y as f64 + dy).clamp(
                self.input.rects[ineq.short.0].y_min as f64,
                self.input.rects[ineq.short.0].y_max as f64,
            ) as usize;
            self.xy[ineq.short.0] = Coord::new(x, y);

            // long
            let (dx, dy) = ineq.calc_gradient_long(&self.xy, &self.dist);
            let (dx, dy) = (
                dx * deltas[ineq.long.0].0 * learning_rate,
                dy * deltas[ineq.long.0].1 * learning_rate,
            );
            let x = (self.xy[ineq.long.0].x as f64 + dx).clamp(
                self.input.rects[ineq.long.0].x_min as f64,
                self.input.rects[ineq.long.0].x_max as f64,
            ) as usize;
            let y = (self.xy[ineq.long.0].y as f64 + dy).clamp(
                self.input.rects[ineq.long.0].y_min as f64,
                self.input.rects[ineq.long.0].y_max as f64,
            ) as usize;

            self.xy[ineq.long.0] = Coord::new(x, y);

            for i in 0..self.input.N {
                self.dist[ineq.short.0][i] = self.xy[ineq.short.0].euclidean_dist(self.xy[i]);
                self.dist[i][ineq.short.0] = self.dist[ineq.short.0][i];
                self.dist[ineq.long.0][i] = self.xy[ineq.long.0].euclidean_dist(self.xy[i]);
                self.dist[i][ineq.long.0] = self.dist[ineq.long.0][i];
            }
            updated_cnt += 1;
        }

        let mut after_dist_diff = 0;
        for i in 0..self.input.N {
            for j in i + 1..self.input.N {
                after_dist_diff += if true_dist[i][j] >= self.dist[i][j] {
                    true_dist[i][j] - self.dist[i][j]
                } else {
                    self.dist[i][j] - true_dist[i][j]
                };
            }
        }

        eprintln!("===== Estimate by inequalities =====");
        eprintln!("before dist diff = {}", before_dist_diff);
        eprintln!("after  dist diff = {}", after_dist_diff);
        eprintln!(
            "improve rate: {}",
            after_dist_diff as f64 / before_dist_diff as f64
        );
        eprintln!(
            "before ineq: {}/{}",
            before_ineq_error_num,
            self.inequalities.len()
        );
        let after_ineq_error_num = self
            .inequalities
            .iter()
            .filter(|ineq| ineq.is_error_by_dist(&self.dist))
            .count();
        eprintln!(
            "after ineq: {}/{}",
            after_ineq_error_num,
            self.inequalities.len()
        );

        eprintln!("iter = {}", iter);
        eprintln!("updated_cnt = {}", updated_cnt);
        eprintln!("===== finished =====");
        eprintln!();
    }
    pub fn gibbs_sampling(&mut self, TLE: f64) -> Vec<Vec<f64>> {
        let mut rng = Pcg64Mcg::new(100);
        let mut dist_sum = self.dist.clone();
        let best_error_sum = self
            .inequalities
            .iter()
            .filter(|ineq| ineq.is_error_by_dist(&self.dist))
            .count();
        let mut ineqs_related_to_node = vec![vec![]; self.input.N];
        for ineq in self.inequalities.iter() {
            let mut nodes = vec![ineq.short.0, ineq.short.1, ineq.long.0, ineq.long.1];
            nodes.sort();
            nodes.dedup();
            for &node in nodes.iter() {
                ineqs_related_to_node[node].push(ineq);
            }
        }
        let mut cnt = 1;

        let start_time = get_time();

        'outer: for _ in 0..50 {
            let mut xy = self.xy.clone();
            let mut dist = self.dist.clone();
            for idx in 0..self.input.N {
                let before_coord = xy[idx];
                let before_error_num = ineqs_related_to_node[idx]
                    .iter()
                    .filter(|ineq| ineq.is_error_by_dist(&dist))
                    .count();
                for _ in 0..2 {
                    xy[idx] = self.input.rects[idx].random_coord(&mut rng);
                    for i in 0..self.input.N {
                        dist[idx][i] = xy[idx].euclidean_dist(xy[i]);
                        dist[i][idx] = dist[idx][i];
                    }
                    let after_error_num = ineqs_related_to_node[idx]
                        .iter()
                        .filter(|ineq| ineq.is_error_by_dist(&dist))
                        .count();
                    if before_error_num < after_error_num {
                        xy[idx] = before_coord;
                        for i in 0..self.input.N {
                            dist[idx][i] = xy[idx].euclidean_dist(xy[i]);
                            dist[i][idx] = dist[idx][i];
                        }
                    } else {
                        break;
                    }
                    if get_time() > TLE {
                        break 'outer;
                    }
                }
            }
            self.xy = xy;
            for i in 0..self.input.N {
                for j in 0..self.input.N {
                    dist_sum[i][j] += dist[i][j];
                    self.dist[i][j] = dist[i][j];
                }
            }
            cnt += 1;
        }
        let mut expected_dist = vec![vec![0.0; self.input.N]; self.input.N];
        for i in 0..self.input.N {
            for j in 0..self.input.N {
                expected_dist[i][j] = dist_sum[i][j] as f64 / cnt as f64;
            }
        }

        eprintln!("===== Gibbs sampling =====");
        eprintln!("best_error_sum = {}", best_error_sum);
        eprintln!("cnt = {}", cnt);
        let elapsed_time = get_time() - start_time;
        eprintln!("elapsed_time = {}", elapsed_time);
        eprintln!("===== finished =====");
        eprintln!();
        expected_dist
    }
}

fn get_query_nodes(
    query_nodes: &mut Vec<usize>,
    used_edges: &mut FxHashSet<(usize, usize)>,
    used_cnt: &Vec<isize>,
    xy: &Vec<Coord>,
    input: &Input,
    rng: &mut Pcg64Mcg,
) -> bool {
    const MIN_DIST: usize = 100;
    let n = query_nodes.len();

    // クエリの長さがLに達したら終了
    if n == input.L {
        return true;
    }

    if query_nodes.len() == 1 {
        // クエリの長さが1の場合は、正三角形を構成するノードを生成できないので、適当なノードを追加
        // 使用回数が少ないノードから順に試す
        for second_node_idx in (0..input.N).sorted_by_key(|&i| used_cnt[i]) {
            if query_nodes.contains(&second_node_idx) {
                continue;
            }
            let dist = xy[query_nodes[0]].euclidean_dist(xy[second_node_idx]);
            if dist < MIN_DIST {
                continue;
            }
            query_nodes.push(second_node_idx);
            if get_query_nodes(query_nodes, used_edges, used_cnt, xy, input, rng) {
                return true;
            }
            query_nodes.pop();
        }
    } else {
        // 正三角形を構成するノードを生成
        for i in 0..n {
            for j in 0..n {
                if i == j || used_edges.contains(&(i, j)) {
                    continue;
                }
                let coord = xy[query_nodes[i]].rotate_120deg(xy[query_nodes[j]]);

                if let Some(coord) = coord {
                    // 使用回数が少ないノードから順に試す
                    let nodes = get_neighbor_nodes(coord, input)
                        .into_iter()
                        .sorted_by_key(|&i| used_cnt[i])
                        .collect_vec();
                    let k = query_nodes.len();
                    used_edges.insert((i, j));
                    used_edges.insert((k, i));
                    used_edges.insert((j, k));

                    for node in nodes {
                        if query_nodes.contains(&node) {
                            continue;
                        }
                        // 同じような位置にあるノードはクエリに含めない(全てのノードから一定距離以上離れていること)
                        if (0..n)
                            .into_iter()
                            .all(|k| xy[query_nodes[k]].euclidean_dist(xy[node]) >= MIN_DIST)
                        {
                            query_nodes.push(node);
                            if get_query_nodes(query_nodes, used_edges, used_cnt, xy, input, rng) {
                                return true;
                            }
                            query_nodes.pop();
                        }
                    }
                    used_edges.remove(&(i, j));
                    used_edges.remove(&(k, i));
                    used_edges.remove(&(j, k));
                }
            }
        }
    }
    false
}

fn get_neighbor_nodes(coord: Coord, input: &Input) -> Vec<usize> {
    const RANGE: usize = 500;
    let x_lower = coord.x.saturating_sub(RANGE / 2);
    let x_upper = (coord.x + RANGE / 2).min(10000);
    let y_lower = coord.y.saturating_sub(RANGE / 2);
    let y_upper = (coord.y + RANGE / 2).min(10000);
    let x_range = input.x_positions.range(x_lower..=x_upper);
    let y_range = input.y_positions.range(y_lower..=y_upper);
    let x_range_points: FxHashSet<_> = x_range.map(|(_, indices)| indices).flatten().collect();
    let y_range_points: FxHashSet<_> = y_range.map(|(_, indices)| indices).flatten().collect();

    x_range_points
        .intersection(&y_range_points)
        .cloned()
        .into_iter()
        .cloned()
        .collect()
}

fn get_circle_edges(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut mst_edges = vec![vec![]; n];
    let mut not_mst_edges = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            not_mst_edges[i].push(j);
        }
    }

    for &(u, v) in uv.iter() {
        mst_edges[u].push(v);
        mst_edges[v].push(u);

        let remove_v_idx = not_mst_edges[u].iter().position(|&p| p == v).unwrap();
        not_mst_edges[u].remove(remove_v_idx);
        let remove_u_idx = not_mst_edges[v].iter().position(|&p| p == u).unwrap();
        not_mst_edges[v].remove(remove_u_idx);
    }

    fn dfs(
        u: usize,
        start: usize,
        next: usize,
        edges: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
        parent: &mut Vec<usize>,
        cycle: &mut Vec<(usize, usize)>,
    ) -> bool {
        if u == start {
            visited[next] = true;
            cycle.push((u, next));
            if dfs(next, start, next, edges, visited, parent, cycle) {
                return true;
            }
            cycle.pop();
        } else {
            for &v in edges[u].iter() {
                if !visited[v] {
                    visited[v] = true;
                    parent[v] = u;
                    cycle.push((u, v));
                    if dfs(v, start, next, edges, visited, parent, cycle) {
                        return true;
                    }
                    cycle.pop();
                } else if parent[u] != v {
                    cycle.push((u, v));
                    return true;
                }
            }
        }
        false
    }

    let mut cycles = vec![];

    for (start, nexts) in not_mst_edges.iter().enumerate() {
        for &next in nexts.iter() {
            if start > next {
                continue;
            }
            let mut Q = VecDeque::new();
            let mut visited = vec![false; n];
            let mut parent = vec![!0; n];
            let mut cycle = vec![];
            Q.push_back(start);
            visited[start] = true;
            dfs(
                start,
                start,
                next,
                &mst_edges,
                &mut visited,
                &mut parent,
                &mut cycle,
            );
            cycles.push(cycle);
        }
    }
    cycles
}

fn to_node_idx(edge: &mut (usize, usize), points: &Vec<usize>) {
    let mut u = points[edge.0];
    let mut v = points[edge.1];
    if u > v {
        std::mem::swap(&mut u, &mut v);
    }
    edge.0 = u;
    edge.1 = v;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Inequality {
    pub short: (usize, usize),
    pub long: (usize, usize),
}

impl Inequality {
    fn new(short: (usize, usize), long: (usize, usize)) -> Self {
        Self { short, long }
    }
    fn is_error_by_dist(&self, dist: &Vec<Vec<usize>>) -> bool {
        dist[self.short.0][self.short.1] > dist[self.long.0][self.long.1]
    }
    fn is_error_by_coord(&self, xy: &Vec<Coord>) -> bool {
        xy[self.short.0].euclidean_dist(xy[self.short.1])
            > xy[self.long.0].euclidean_dist(xy[self.long.1])
    }
    fn swap_short_nodes(&mut self) {
        std::mem::swap(&mut self.short.0, &mut self.short.1);
    }
    fn swap_long_nodes(&mut self) {
        std::mem::swap(&mut self.long.0, &mut self.long.1);
    }
    fn calc_gradient_short(&self, xy: &Vec<Coord>, dist: &Vec<Vec<usize>>) -> (f64, f64) {
        let length = dist[self.short.0][self.short.1] as f64;
        let dx = xy[self.short.1].x as f64 - xy[self.short.0].x as f64;
        let dy = xy[self.short.1].y as f64 - xy[self.short.0].y as f64;
        (dx / length, dy / length)
    }
    fn calc_gradient_long(&self, xy: &Vec<Coord>, dist: &Vec<Vec<usize>>) -> (f64, f64) {
        let length = dist[self.long.0][self.long.1] as f64;
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_circle_edges() {
        let points = vec![3, 5, 6, 8, 2];
        let mut uv = vec![];
        //      3
        //     / \
        //    5   6
        //       / \
        //      8   2
        uv.push((3, 5));
        uv.push((3, 6));
        uv.push((6, 8));
        uv.push((6, 2));
        let mut idx_uv = vec![];
        for &(u, v) in uv.iter() {
            let u_idx = points.iter().position(|&p| p == u).unwrap();
            let v_idx = points.iter().position(|&p| p == v).unwrap();
            idx_uv.push((u_idx, v_idx));
        }
        let mut cycles = get_circle_edges(points.len(), &idx_uv);
        for cycle in cycles.iter_mut() {
            for edge in cycle.iter_mut() {
                to_node_idx(edge, &points);
            }
        }

        let mut inequalities = vec![];
        for cycle in cycles.iter() {
            let long = cycle[0];
            for &short in cycle.iter().skip(1) {
                inequalities.push(Inequality::new(short, long));
            }
        }
        inequalities.sort();
        inequalities.dedup();

        for row in inequalities.iter() {
            eprintln!("short = {:?}, long = {:?}", row.short, row.long);
        }
    }
}
