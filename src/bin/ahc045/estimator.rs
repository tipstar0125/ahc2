use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
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
    inequalities: Vec<Inequality>,
    pub xy: Vec<Coord>,
    pub dist: Vec<Vec<usize>>,
    pub error_num: Vec<usize>,
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
            let ratio_lower_threshold = 0.99;
            let ratio_upper_threshold = 1.01;
            let mut used = FxHashSet::default();
            let mut used_cnt = vec![0; input.N];

            for &(_, base_idx) in sorted_points.iter() {
                'outer: loop {
                    let second_idx = rng.gen_range(0..input.N);
                    if base_idx == second_idx {
                        continue;
                    }
                    let base_coord = xy_center[base_idx];
                    let second_coord = xy_center[second_idx];
                    let dist0 = calc_dist(base_coord, second_coord);
                    let third_coord_center = {
                        let cand1_coord = rotate_120deg(base_coord, second_coord);
                        let cand2_coord = rotate_120deg(base_coord, second_coord);
                        if cand1_coord.is_some() {
                            cand1_coord
                        } else if cand2_coord.is_some() {
                            cand2_coord
                        } else {
                            None
                        }
                    };
                    if third_coord_center.is_none() {
                        continue;
                    }
                    let x_lower = third_coord_center.unwrap().x.saturating_sub(delta / 2);
                    let x_upper = (third_coord_center.unwrap().x + delta / 2).min(10000);
                    let y_lower = third_coord_center.unwrap().y.saturating_sub(delta / 2);
                    let y_upper = (third_coord_center.unwrap().y + delta / 2).min(10000);
                    assert!(x_lower <= x_upper);
                    assert!(y_lower <= y_upper);
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
                    let cand = x_range_points.intersection(&y_range_points);
                    for &third_idx in cand {
                        if base_idx == third_idx || second_idx == third_idx {
                            continue;
                        }
                        let mut points = vec![base_idx, second_idx, third_idx];
                        points.sort();
                        if queries.contains(&points) {
                            continue;
                        }
                        if used.contains(&(points[0], points[1]))
                            || used.contains(&(points[0], points[2]))
                            || used.contains(&(points[1], points[2]))
                        {
                            continue;
                        }
                        let third_coord = xy_center[third_idx];
                        assert!(third_coord.x >= x_lower && third_coord.x <= x_upper);
                        assert!(third_coord.y >= y_lower && third_coord.y <= y_upper);
                        let dist1 = calc_dist(base_coord, third_coord);
                        let dist2 = calc_dist(second_coord, third_coord);
                        let ratio1 = dist1 as f64 / dist0 as f64;
                        let ratio2 = dist2 as f64 / dist0 as f64;
                        if ratio1 < ratio_lower_threshold
                            || ratio1 > ratio_upper_threshold
                            || ratio2 < ratio_lower_threshold
                            || ratio2 > ratio_upper_threshold
                        {
                            continue;
                        }
                        used.insert((points[0], points[1]));
                        used.insert((points[0], points[2]));
                        used.insert((points[1], points[2]));
                        queries.push(vec![points[0], points[1], points[2]]);
                        used_cnt[points[0]] += 1;
                        used_cnt[points[1]] += 1;
                        used_cnt[points[2]] += 1;
                        break 'outer;
                    }
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

        let mut inequalities = vec![Inequality::new(); input.N];
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
                    let related_nodes = vec![long.0, long.1, short.0, short.1];
                    for &idx in related_nodes.iter() {
                        inequalities[idx].add(short, long);
                    }
                }
            }
        }

        let mut error_num = vec![0; input.N];
        for i in 0..input.N {
            error_num[i] = inequalities[i].get_error_num(&dist_center);
        }

        Self {
            inequalities,
            xy: xy_center,
            dist: dist_center,
            error_num,
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

        let mut rng = Pcg64Mcg::new(100);
        let mut iter = 0;
        let mut updated_cnt = 0;
        let check_interval = 10000;
        while get_time() < TLE {
            iter += 1;
            let idx = rng.gen_range(0..input.N);
            let before_error_num = self.inequalities[idx].get_error_num(&self.dist);

            let before_coord = self.xy[idx];
            self.xy[idx] = Coord::new(
                rng.gen_range(input.range[idx].0..=input.range[idx].1),
                rng.gen_range(input.range[idx].2..=input.range[idx].3),
            );
            for i in 0..input.N {
                self.dist[idx][i] = calc_dist(self.xy[idx], self.xy[i]);
                self.dist[i][idx] = self.dist[idx][i];
            }
            let after_error_num = self.inequalities[idx].get_error_num(&self.dist);
            if before_error_num < after_error_num {
                self.xy[idx] = before_coord;
                for i in 0..input.N {
                    self.dist[idx][i] = calc_dist(self.xy[idx], self.xy[i]);
                    self.dist[i][idx] = self.dist[idx][i];
                }
            } else {
                updated_cnt += 1;
            }
            if updated_cnt % check_interval == check_interval - 1 {
                let error_sum = self
                    .inequalities
                    .iter()
                    .map(|ineq| ineq.get_error_num(&self.dist))
                    .sum::<usize>();
                if error_sum == 0 {
                    break;
                }
            }
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
            "before_error_num = {}",
            self.error_num.iter().sum::<usize>()
        );
        for i in 0..input.N {
            self.error_num[i] = self.inequalities[i].get_error_num(&self.dist);
        }
        eprintln!("after_error_num = {}", self.error_num.iter().sum::<usize>());
        eprintln!("iter = {}", iter);
        eprintln!("updated_cnt = {}", updated_cnt);
        eprintln!("===== finished =====");
        eprintln!();
    }
    pub fn gibbs_sampling(&mut self, input: &Input) {
        let mut rng = Pcg64Mcg::new(100);
        let mut dist_sum = self.dist.clone();
        let mut best_error_sum = self.error_num.iter().sum::<usize>();
        let mut cnt = 1;

        let start_time = get_time();

        for _ in 0..50 {
            let mut xy = self.xy.clone();
            let mut dist = self.dist.clone();
            for idx in 0..input.N {
                let before_coord = xy[idx];
                let before_error_num = self.inequalities[idx].get_error_num(&dist);
                for _ in 0..2 {
                    xy[idx] = Coord::new(
                        rng.gen_range(input.range[idx].0..=input.range[idx].1),
                        rng.gen_range(input.range[idx].2..=input.range[idx].3),
                    );
                    for i in 0..input.N {
                        dist[idx][i] = calc_dist(xy[idx], xy[i]);
                        dist[i][idx] = dist[idx][i];
                    }
                    let after_error_num = self.inequalities[idx].get_error_num(&dist);
                    if before_error_num < after_error_num {
                        xy[idx] = before_coord;
                        for i in 0..input.N {
                            dist[idx][i] = calc_dist(xy[idx], xy[i]);
                            dist[i][idx] = dist[idx][i];
                        }
                    } else {
                        break;
                    }
                }
            }
            let after_error_sum = self
                .inequalities
                .iter()
                .map(|ineq| ineq.get_error_num(&dist))
                .sum::<usize>();
            if after_error_sum <= best_error_sum {
                best_error_sum = after_error_sum;
                self.xy = xy;
                for i in 0..input.N {
                    for j in 0..input.N {
                        dist_sum[i][j] += dist[i][j];
                        self.dist[i][j] = dist[i][j];
                    }
                }
                cnt += 1;
            }
        }

        for i in 0..input.N {
            for j in 0..input.N {
                self.dist[i][j] = dist_sum[i][j] / cnt as usize;
            }
        }

        eprintln!("===== Gibbs sampling =====");
        eprintln!("best_error_sum = {}", best_error_sum);
        eprintln!("cnt = {}", cnt);
        let elapsed_time = get_time() - start_time;
        eprintln!("elapsed_time = {}", elapsed_time);
        eprintln!("===== finished =====");
        eprintln!();
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

#[derive(Clone, Debug)]
struct Inequality(BTreeMap<(usize, usize), BTreeSet<(usize, usize)>>);

impl Inequality {
    fn new() -> Self {
        Self(BTreeMap::default())
    }
    fn add(&mut self, mut short: (usize, usize), mut long: (usize, usize)) {
        if short.0 > short.1 {
            std::mem::swap(&mut short.0, &mut short.1);
        }
        if long.0 > long.1 {
            std::mem::swap(&mut long.0, &mut long.1);
        }
        self.0
            .entry(short)
            .or_insert(BTreeSet::default())
            .insert(long);
    }
    fn get_error_num(&self, dist: &Vec<Vec<usize>>) -> usize {
        let mut error_num = 0;
        for (short, longs) in self.0.iter() {
            let dist_short = dist[short.0][short.1];
            for &long in longs.iter() {
                let dist_long = dist[long.0][long.1];
                if dist_short > dist_long {
                    error_num += 1;
                }
            }
        }
        error_num
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
        let mut inequalities = vec![Inequality::new(); points.iter().max().unwrap() + 1];
        for cycle in cycles.iter() {
            let long = cycle[0];
            for &short in cycle.iter().skip(1) {
                let related_nodes = vec![long.0, long.1, short.0, short.1];
                for &idx in related_nodes.iter() {
                    inequalities[idx].add(short, long);
                }
            }
        }

        for (idx, row) in inequalities.iter().enumerate() {
            eprintln!("{}", idx);
            for (short, longs) in row.0.iter() {
                eprintln!("{:?} {:?}", short, longs);
            }
        }
    }
}
