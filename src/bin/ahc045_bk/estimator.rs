use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, VecDeque},
};

use itertools::Itertools;
use proconio::input_interactive;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use rustc_hash::FxHashSet;

use crate::{
    common::get_time,
    coord::{calc_dist, Coord},
    input::Input,
};

pub struct Estimator {
    ineqs: Vec<Ineq>,
    pub xy: Vec<Coord>,
    pub dist: Vec<Vec<usize>>,
}

impl Estimator {
    pub fn new(input: &Input) -> Self {
        // 中心座標を仮定
        let xy_center = input
            .range
            .iter()
            .map(|(lx, rx, ly, ry)| Coord::new((lx + rx) / 2, (ly + ry) / 2))
            .collect::<Vec<_>>();

        let mut dist_center = vec![vec![0; input.N]; input.N];
        for i in 0..input.N {
            for j in 0..input.N {
                if i == j {
                    continue;
                }
                dist_center[i][j] = calc_dist(xy_center[i], xy_center[j]);
                dist_center[j][i] = dist_center[i][j];
            }
        }

        let mut x_positions = BTreeMap::default();
        let mut y_positions = BTreeMap::default();
        for i in 0..input.N {
            let x = xy_center[i].x;
            let y = xy_center[i].y;
            x_positions.entry(x).or_insert(vec![]).push(i);
            y_positions.entry(y).or_insert(vec![]).push(i);
        }

        // 誤差が大きい順に点をソート
        let sorted_points = input
            .range
            .iter()
            .enumerate()
            .map(|(i, (lx, rx, ly, ry))| ((rx - ly).max(ry - lx), i))
            .sorted_by(|a, b| b.0.cmp(&a.0))
            .collect::<Vec<_>>();

        let mut queries = vec![];

        if input.L == 3 {
            // 誤差が大きい順に基準座標を選択し、クエリの重複がないように3点を選択
            let mut rng = Pcg64Mcg::new(100);
            let delta = 100; // 3点目の座標の誤差範囲
            let mut used = FxHashSet::default();

            for &(_, base_idx) in sorted_points.iter() {
                let mut points = vec![base_idx];
                'outer: loop {
                    // 1点しかない場合はランダムで2点目を選択
                    if points.len() == 1 {
                        let second_idx = rng.gen_range(0..input.N);
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
                                .push(rotate_120deg(xy_center[points[i]], xy_center[points[j]]));
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
                        let x_range = x_positions.range(x_lower..=x_upper);
                        let y_range = y_positions.range(y_lower..=y_upper);
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
                        if points.len() == input.L {
                            queries.push(points);
                            break 'outer;
                        }
                    }
                    points.pop();
                }
                if queries.len() == input.Q {
                    break;
                }
            }
        } else {
            let size = 10000;
            let grid_num = 20;
            let delta = size / grid_num;
            let grid_centers = (0..=grid_num).map(|i| i * delta).collect::<Vec<_>>();

            for xi in 0..grid_num {
                for yi in 0..grid_num {
                    let x_lower = grid_centers[xi];
                    let x_upper = grid_centers[xi + 1];
                    let y_lower = grid_centers[yi];
                    let y_upper = grid_centers[yi + 1];
                    let x_range = x_positions.range(x_lower..=x_upper);
                    let y_range = y_positions.range(y_lower..=y_upper);
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
                        .map(|&idx| {
                            let error = (input.range[idx].1 - input.range[idx].0)
                                .max(input.range[idx].3 - input.range[idx].2);
                            (error, idx)
                        })
                        .sorted_by(|a, b| b.0.cmp(&a.0))
                        .map(|(_, idx)| idx)
                        .take(input.L)
                        .collect::<FxHashSet<_>>();

                    let mut Q = BinaryHeap::new();
                    let mut dist = vec![1 << 60; input.N];
                    for &node in nodes.iter() {
                        Q.push((Reverse(0), node));
                        dist[node] = 0;
                    }
                    while let Some((Reverse(d), u)) = Q.pop() {
                        if nodes.len() == input.L {
                            break;
                        }
                        nodes.insert(u);
                        for v in 0..input.N {
                            if nodes.contains(&v) {
                                continue;
                            }
                            let nd = calc_dist(xy_center[u], xy_center[v]);
                            if d + nd < dist[v] {
                                dist[v] = d + nd;
                                Q.push((Reverse(d + nd), v));
                            }
                        }
                    }
                    assert!(nodes.is_empty() || nodes.len() == input.L);
                    if !nodes.is_empty() {
                        queries.push(nodes.into_iter().collect::<Vec<_>>());
                    }
                }
            }
        }

        let mut ineqs = vec![];
        for nodes in queries.iter() {
            let nodes = nodes.iter().cloned().collect::<Vec<_>>();
            println!("? {} {}", nodes.len(), nodes.iter().join(" "));
            input_interactive! {
                uv: [(usize, usize); nodes.len() - 1],
            }

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
                    ineqs.push(Ineq::new(short, long));
                }
            }
        }

        // TODO 重複を削除を可能な限り内容にクエリを工夫する必要がある
        ineqs.sort();
        ineqs.dedup();

        Self {
            ineqs,
            xy: xy_center,
            dist: dist_center,
        }
    }
    pub fn climbing(&mut self, input: &Input, TLE: f64) {
        let mut true_dist = vec![vec![0; input.N]; input.N];
        for i in 0..input.N {
            for j in 0..input.N {
                true_dist[i][j] = calc_dist(input.xy[i], input.xy[j]);
            }
        }

        let mut before_dist_diff = 0;
        for i in 0..input.N {
            for j in i + 1..input.N {
                before_dist_diff += if true_dist[i][j] >= self.dist[i][j] {
                    true_dist[i][j] - self.dist[i][j]
                } else {
                    self.dist[i][j] - true_dist[i][j]
                };
            }
        }

        let before_ineq_error_num = self
            .ineqs
            .iter()
            .filter(|ineq| ineq.is_error_by_dist(&self.dist))
            .count();

        let deltas = input
            .range
            .iter()
            .map(|(lx, rx, ly, ry)| ((rx - lx) as f64, (ry - ly) as f64))
            .collect::<Vec<_>>();

        let mut rng = Pcg64Mcg::new(100);
        let mut iter = 0;
        let mut updated_cnt = 0;
        let ineq_num = self.ineqs.len();
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
            if !self.ineqs[idx].is_error_by_dist(&self.dist) {
                continue;
            }

            if rng.gen_bool(0.5) {
                self.ineqs[idx].swap_short_nodes();
            }
            if rng.gen_bool(0.5) {
                self.ineqs[idx].swap_long_nodes();
            }

            let ineq = &self.ineqs[idx];

            // short
            let (dx, dy) = ineq.calc_gradient_short(&self.xy, &self.dist);
            let (dx, dy) = (
                dx * deltas[ineq.short.0].0 * learning_rate,
                dy * deltas[ineq.short.0].1 * learning_rate,
            );
            let x = (self.xy[ineq.short.0].x as f64 + dx).clamp(
                input.range[ineq.short.0].0 as f64,
                input.range[ineq.short.0].1 as f64,
            ) as usize;
            let y = (self.xy[ineq.short.0].y as f64 + dy).clamp(
                input.range[ineq.short.0].2 as f64,
                input.range[ineq.short.0].3 as f64,
            ) as usize;
            self.xy[ineq.short.0] = Coord::new(x, y);

            // long
            let (dx, dy) = ineq.calc_gradient_long(&self.xy, &self.dist);
            let (dx, dy) = (
                dx * deltas[ineq.long.0].0 * learning_rate,
                dy * deltas[ineq.long.0].1 * learning_rate,
            );
            let x = (self.xy[ineq.long.0].x as f64 + dx).clamp(
                input.range[ineq.long.0].0 as f64,
                input.range[ineq.long.0].1 as f64,
            ) as usize;
            let y = (self.xy[ineq.long.0].y as f64 + dy).clamp(
                input.range[ineq.long.0].2 as f64,
                input.range[ineq.long.0].3 as f64,
            ) as usize;

            self.xy[ineq.long.0] = Coord::new(x, y);

            for i in 0..input.N {
                self.dist[ineq.short.0][i] = calc_dist(self.xy[ineq.short.0], self.xy[i]);
                self.dist[i][ineq.short.0] = self.dist[ineq.short.0][i];
                self.dist[ineq.long.0][i] = calc_dist(self.xy[ineq.long.0], self.xy[i]);
                self.dist[i][ineq.long.0] = self.dist[ineq.long.0][i];
            }
            updated_cnt += 1;
        }

        let mut after_dist_diff = 0;
        for i in 0..input.N {
            for j in i + 1..input.N {
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
            self.ineqs.len()
        );
        let after_ineq_error_num = self
            .ineqs
            .iter()
            .filter(|ineq| ineq.is_error_by_dist(&self.dist))
            .count();
        eprintln!("after ineq: {}/{}", after_ineq_error_num, self.ineqs.len());

        eprintln!("iter = {}", iter);
        eprintln!("updated_cnt = {}", updated_cnt);
        eprintln!("===== finished =====");
        eprintln!();
    }
    pub fn gibbs_sampling(&mut self, input: &Input, TLE: f64) -> Vec<Vec<f64>> {
        let mut rng = Pcg64Mcg::new(100);
        let mut dist_sum = self.dist.clone();
        let best_error_sum = self
            .ineqs
            .iter()
            .filter(|ineq| ineq.is_error_by_dist(&self.dist))
            .count();
        let mut ineqs_related_to_node = vec![vec![]; input.N];
        for ineq in self.ineqs.iter() {
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
            for idx in 0..input.N {
                let before_coord = xy[idx];
                let before_error_num = ineqs_related_to_node[idx]
                    .iter()
                    .filter(|ineq| ineq.is_error_by_dist(&dist))
                    .count();
                for _ in 0..2 {
                    xy[idx] = Coord::new(
                        rng.gen_range(input.range[idx].0..=input.range[idx].1),
                        rng.gen_range(input.range[idx].2..=input.range[idx].3),
                    );
                    for i in 0..input.N {
                        dist[idx][i] = calc_dist(xy[idx], xy[i]);
                        dist[i][idx] = dist[idx][i];
                    }
                    let after_error_num = ineqs_related_to_node[idx]
                        .iter()
                        .filter(|ineq| ineq.is_error_by_dist(&dist))
                        .count();
                    if before_error_num < after_error_num {
                        xy[idx] = before_coord;
                        for i in 0..input.N {
                            dist[idx][i] = calc_dist(xy[idx], xy[i]);
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
            for i in 0..input.N {
                for j in 0..input.N {
                    dist_sum[i][j] += dist[i][j];
                    self.dist[i][j] = dist[i][j];
                }
            }
            cnt += 1;
        }
        let mut expected_dist = vec![vec![0.0; input.N]; input.N];
        for i in 0..input.N {
            for j in 0..input.N {
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

const SIN120: f64 = 0.8660254037844386;
const COS120: f64 = -0.5;

fn rotate_120deg(pos0: Coord, pos1: Coord) -> Option<Coord> {
    let dx = pos0.x as f64 - pos1.x as f64;
    let dy = pos0.y as f64 - pos1.y as f64;
    let x = pos0.x as f64 + dx * COS120 - dy * SIN120;
    let y = pos0.y as f64 + dx * SIN120 + dy * COS120;
    if x < 0.0 || x > 10000.0 || y < 0.0 || y > 10000.0 {
        None
    } else {
        Some(Coord::new(x as usize, y as usize))
    }
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
struct Ineq {
    pub short: (usize, usize),
    pub long: (usize, usize),
}

impl Ineq {
    fn new(short: (usize, usize), long: (usize, usize)) -> Self {
        Self { short, long }
    }
    fn is_error_by_dist(&self, dist: &Vec<Vec<usize>>) -> bool {
        dist[self.short.0][self.short.1] > dist[self.long.0][self.long.1]
    }
    fn is_error_by_coord(&self, xy: &Vec<Coord>) -> bool {
        calc_dist(xy[self.short.0], xy[self.short.1]) > calc_dist(xy[self.long.0], xy[self.long.1])
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
                inequalities.push(Ineq::new(short, long));
            }
        }
        inequalities.sort();
        inequalities.dedup();

        for row in inequalities.iter() {
            eprintln!("short = {:?}, long = {:?}", row.short, row.long);
        }
    }
}
