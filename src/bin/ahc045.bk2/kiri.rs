use rand::prelude::*;
use std::collections::HashMap;
use std::collections::HashMap as Map;
use std::collections::HashSet;
use std::io::{self, BufRead, Write};
use std::time::{Duration, Instant};

// グローバル定数：コードテスト = 1、本番実行 = 0
const CODETEST: usize = 0;
const TIME_LIMIT: f64 = 1.90;

fn sample_2d_normal(
    e_x: f64,
    e_y: f64,
    v_xx: f64,
    v_xy: f64,
    v_yy: f64,
    rng: &mut ThreadRng,
) -> (f64, f64) {
    // Box-Muller法で標準正規分布から z1, z2 を生成
    let u1: f64 = rng.gen::<f64>();
    let u2: f64 = rng.gen::<f64>();
    let r = (-2.0 * u1.ln()).sqrt();
    let theta = 2.0 * std::f64::consts::PI * u2;
    let z1 = r * theta.cos();
    let z2 = r * theta.sin();

    // 分散行列のCholesky分解により変換
    let l11 = v_xx.sqrt();
    let l21 = if l11.abs() > 1e-14 { v_xy / l11 } else { 0.0 };
    let l22 = (v_yy - l21 * l21).max(0.0).sqrt();

    let x = e_x + l11 * z1;
    let y = e_y + l21 * z1 + l22 * z2;
    (x, y)
}
fn compute_info_gain(sim_results: &[Vec<u32>], v_list: &[f64]) -> f64 {
    let mut sum_entropy = 0.0;
    let mut sum_weighted_entropy = 0.0;

    for (i, res_list) in sim_results.iter().enumerate() {
        let mut freq: HashMap<u32, usize> = HashMap::new();
        for &bitmask in res_list {
            *freq.entry(bitmask).or_insert(0) += 1;
        }

        let n_sims = res_list.len() as f64;
        if n_sims == 0.0 {
            continue;
        }

        let mut entropy = 0.0;
        for &count in freq.values() {
            let p = count as f64 / n_sims;
            entropy -= p * p.log2();
        }

        sum_entropy += entropy;
        sum_weighted_entropy += entropy * v_list[i].sqrt();
    }

    sum_weighted_entropy
}
struct UnionFind {
    parent: Vec<isize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: vec![-1; n],
        }
    }

    fn root(&mut self, mut a: usize) -> usize {
        let mut path = vec![];
        while self.parent[a] >= 0 {
            path.push(a);
            a = self.parent[a] as usize;
        }
        for x in path {
            self.parent[x] = a as isize;
        }
        a
    }

    fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.root(a);
        let mut rb = self.root(b);
        if ra == rb {
            return false;
        }
        if self.parent[rb] < self.parent[ra] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[ra] += self.parent[rb];
        self.parent[rb] = ra as isize;
        true
    }

    #[allow(dead_code)]
    fn same(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }
}

#[derive(Debug)]
pub struct Data {
    pub n: usize,
    pub m: usize,
    pub q: usize,
    pub l: usize,
    pub w: usize,
    pub g: Vec<usize>,
    pub a: Vec<(usize, usize, usize, usize)>, // ← lx, rx, ly, ry
    pub actual: Option<Vec<(f64, f64)>>,
    pub code_test: bool, // ← Pythonの CODETEST = 1 に対応
}

impl Data {
    fn read_input() -> io::Result<Data> {
        if CODETEST == 1 {
            Self::read_codetest_input()
        } else {
            Self::read_real_input()
        }
    }

    fn read_real_input() -> io::Result<Data> {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        let first_line = lines.next().unwrap()?;
        let mut iter = first_line.split_whitespace();
        let n = iter.next().unwrap().parse().unwrap();
        let m = iter.next().unwrap().parse().unwrap();
        let q = iter.next().unwrap().parse().unwrap();
        let l = iter.next().unwrap().parse().unwrap();
        let w = iter.next().unwrap().parse().unwrap();

        let g_line = lines.next().unwrap()?;
        let g = g_line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let mut a = vec![];
        for _ in 0..n {
            let line = lines.next().unwrap()?;
            let vals: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            a.push((vals[0], vals[1], vals[2], vals[3]));
        }

        Ok(Data {
            n,
            m,
            q,
            l,
            w,
            g,
            a,
            actual: None,
            code_test: CODETEST == 1,
        })
    }

    fn read_codetest_input() -> io::Result<Data> {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        let first_line = lines.next().unwrap()?;
        let mut iter = first_line.split_whitespace();
        let n = iter.next().unwrap().parse().unwrap();
        let m = iter.next().unwrap().parse().unwrap();
        let q = iter.next().unwrap().parse().unwrap();
        let l = iter.next().unwrap().parse().unwrap();
        let w = iter.next().unwrap().parse().unwrap();

        let g_line = lines.next().unwrap()?;
        let g = g_line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let mut a = vec![];
        for _ in 0..n {
            let line = lines.next().unwrap()?;
            let vals: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            a.push((vals[0], vals[1], vals[2], vals[3]));
        }

        let mut actual = vec![];
        for _ in 0..n {
            let line = lines.next().unwrap()?;
            let vals: Vec<usize> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            actual.push((vals[0] as f64, vals[1] as f64));
        }

        Ok(Data {
            n,
            m,
            q,
            l,
            w,
            g,
            a,
            actual: Some(actual),
            code_test: CODETEST == 1,
        })
    }

    fn print_summary(&self) {
        println!("N = {}", self.n);
        println!("M = {}", self.m);
        println!("Q = {}", self.q);
        println!("L = {}", self.l);
        println!("W = {}", self.w);
        println!("G = {:?}", self.g);
        println!("Rects[0..3] = {:?}", &self.a[..self.a.len().min(3)]);
        if let Some(actual) = &self.actual {
            println!("Actual[0..3] = {:?}", &actual[..actual.len().min(3)]);
        }
    }
}
#[derive(Debug)]
struct PointState {
    i: usize,
    lx: f64,
    rx: f64,
    ly: f64,
    ry: f64,
    e_x: f64,
    e_y: f64,
    var_xx: f64,
    var_yy: f64,
    var_xy: f64,
    max_var: f64,
    point_state_type: usize, // 0 = uniform, 1 = normal
}

impl PointState {
    fn new(i: usize, lx: usize, rx: usize, ly: usize, ry: usize) -> Self {
        let lx = lx as f64;
        let rx = rx as f64;
        let ly = ly as f64;
        let ry = ry as f64;

        let e_x = (lx + rx) / 2.0;
        let e_y = (ly + ry) / 2.0;
        let var_xx = ((rx - lx).powi(2)) / 12.0;
        let var_yy = ((ry - ly).powi(2)) / 12.0;
        let var_xy = 0.0;
        let mut ps = PointState {
            i,
            lx,
            rx,
            ly,
            ry,
            e_x,
            e_y,
            var_xx,
            var_yy,
            var_xy,
            max_var: 0.0,
            point_state_type: 0,
        };
        ps.max_var = ps.calc_max_variance_direction();
        ps
    }

    fn calc_max_variance_direction(&self) -> f64 {
        let trace = self.var_xx + self.var_yy;
        let diff = (self.var_xx - self.var_yy) / 2.0;
        let root_term = (diff.powi(2) + self.var_xy.powi(2)).sqrt();
        (trace / 2.0 + root_term).abs()
    }

    fn sample(&self, k: usize) -> Vec<(f64, f64)> {
        let mut rng = thread_rng();
        let mut result = Vec::with_capacity(k);

        if self.point_state_type == 0 {
            for _ in 0..k {
                let x = rng.gen_range(self.lx..=self.rx);
                let y = rng.gen_range(self.ly..=self.ry);
                result.push((x, y));
            }
        } else {
            for _ in 0..k {
                let (x, y) = sample_2d_normal(
                    self.e_x,
                    self.e_y,
                    self.var_xx,
                    self.var_xy,
                    self.var_yy,
                    &mut rng,
                );
                let x = x.clamp(self.lx, self.rx);
                let y = y.clamp(self.ly, self.ry);
                result.push((x, y));
            }
        }

        result
    }

    fn debug_print(&self, sample_count: usize) {
        println!("--- PointState #{} ---", self.i);
        println!(
            "Rect = ({:.1}, {:.1}) - ({:.1}, {:.1})",
            self.lx, self.ly, self.rx, self.ry
        );
        println!("E = ({:.1}, {:.1})", self.e_x, self.e_y);
        println!(
            "Var = xx: {:.1}, yy: {:.1}, xy: {:.1}",
            self.var_xx, self.var_yy, self.var_xy
        );
        println!("Max variance dir = {:.3}", self.max_var);
        println!("Sample {} points:", sample_count);
        for (x, y) in self.sample(sample_count) {
            println!("  ({:.1}, {:.1})", x, y);
        }
        println!("----------------------");
    }
    fn update_by_query_param(&mut self, param: (usize, f64, f64, f64, f64, f64)) {
        let (k, sx, sy, sxx, sxy, syy) = param;
        if k < 2 {
            return;
        }

        let kf = k as f64;
        let e_x = sx / kf;
        let e_y = sy / kf;

        let var_xx = (sxx - 2.0 * e_x * sx + e_x * e_x * kf) / (kf - 1.0);
        let var_xy = (sxy - e_x * sy - e_y * sx + e_x * e_y * kf) / (kf - 1.0);
        let var_yy = (syy - 2.0 * e_y * sy + e_y * e_y * kf) / (kf - 1.0);

        // 平均・分散の重み付き更新（既存3、今回kとして）
        self.e_x = (self.e_x * 3.0 + e_x * kf) / (kf + 3.0);
        self.e_y = (self.e_y * 3.0 + e_y * kf) / (kf + 3.0);
        self.var_xx = (self.var_xx * 3.0 + var_xx * (kf - 1.0)) / (kf + 2.0);
        self.var_xy = (self.var_xy * 3.0 + var_xy * (kf - 1.0)) / (kf + 2.0);
        self.var_yy = (self.var_yy * 3.0 + var_yy * (kf - 1.0)) / (kf + 2.0);

        self.max_var = self.calc_max_variance_direction();
        self.point_state_type = 1;
    }
}

struct State {
    data: Data,
    points: Vec<PointState>,
    block2points: Map<(usize, usize), Vec<usize>>,
}

impl State {
    fn new(data: Data) -> Self {
        let mut points = Vec::with_capacity(data.n);
        for (i, &(lx, rx, ly, ry)) in data.a.iter().enumerate() {
            points.push(PointState::new(i, lx, rx, ly, ry));
        }

        let mut state = State {
            data,
            points,
            block2points: Map::new(),
        };
        state.update_block2points();
        state
    }

    fn update_block2points(&mut self) {
        self.block2points.clear();
        for (i, ps) in self.points.iter().enumerate() {
            let bx = ((ps.e_x / 500.0).floor() as usize).min(19);
            let by = ((ps.e_y / 500.0).floor() as usize).min(19);
            self.block2points.entry((bx, by)).or_default().push(i);
        }
    }
    fn calc_total_var(&self) -> f64 {
        self.points.iter().map(|ps| ps.max_var).sum()
    }

    fn calc_act_diff_average_square(&self) -> f64 {
        if let Some(actual) = &self.data.actual {
            let sum: f64 = self
                .points
                .iter()
                .zip(actual.iter())
                .map(|(ps, &(ax, ay))| {
                    let dx = ps.e_x - ax;
                    let dy = ps.e_y - ay;
                    dx * dx + dy * dy
                })
                .sum();
            (sum / self.data.n as f64).sqrt()
        } else {
            0.0
        }
    }

    fn calc_act_diff_average_abs(&self) -> f64 {
        if let Some(actual) = &self.data.actual {
            let sum: f64 = self
                .points
                .iter()
                .zip(actual.iter())
                .map(|(ps, &(ax, ay))| {
                    let dx = ps.e_x - ax;
                    let dy = ps.e_y - ay;
                    (dx * dx + dy * dy).sqrt()
                })
                .sum();
            sum / self.data.n as f64
        } else {
            0.0
        }
    }
    fn do_query(&self, cities: &[usize]) -> Vec<(usize, usize)> {
        if cities.len() < 2 {
            return vec![];
        }

        if CODETEST == 1 {
            // シミュレーションモード（actual 座標を使って MST を構築）
            let nodes = cities;
            let mut edges = vec![];
            let mut stdout = io::stdout().lock();
            if true {
                write!(stdout, "? {}", cities.len()).unwrap();
                for &c in cities {
                    write!(stdout, " {}", c).unwrap();
                }
                writeln!(stdout).unwrap();
                stdout.flush().unwrap();
            }

            let actual = self.data.actual.as_ref().unwrap();

            for i in 0..nodes.len() {
                for j in (i + 1)..nodes.len() {
                    let u = nodes[i];
                    let v = nodes[j];
                    let (ux, uy) = actual[u];
                    let (vx, vy) = actual[v];
                    let dx = ux - vx;
                    let dy = uy - vy;
                    let dist = ((dx * dx + dy * dy).sqrt().floor()) as i64;
                    edges.push((dist, u.min(v), u.max(v)));
                }
            }

            // Kruskal法で MST を作る
            edges.sort();
            let mut uf = UnionFind::new(self.data.n);
            let mut mst = vec![];

            for &(_dist, u, v) in &edges {
                if uf.unite(u, v) {
                    mst.push((u, v));
                    if mst.len() == cities.len() - 1 {
                        break;
                    }
                }
            }

            mst
        } else {
            // 本番モード：クエリを投げる
            let mut stdout = io::stdout().lock();
            write!(stdout, "? {}", cities.len()).unwrap();
            for &c in cities {
                write!(stdout, " {}", c).unwrap();
            }
            writeln!(stdout).unwrap();
            stdout.flush().unwrap();

            let stdin = io::stdin();
            let mut lines = stdin.lock().lines();
            let mut result = vec![];
            for _ in 0..(cities.len() - 1) {
                let line = lines.next().unwrap().unwrap();
                let mut iter = line.split_whitespace();
                let u: usize = iter.next().unwrap().parse().unwrap();
                let v: usize = iter.next().unwrap().parse().unwrap();
                result.push((u.min(v), u.max(v)));
            }
            result
        }
    }
    fn calc_effective_edges(&self, city_list: &[usize]) -> Vec<(usize, usize)> {
        let mut effective_edges = HashSet::new();

        for (i, &c1) in city_list.iter().enumerate() {
            let mut d_list = vec![];
            let (x1, y1) = (self.points[c1].e_x, self.points[c1].e_y);
            let mut min_dist = f64::MAX;

            for (j, &c2) in city_list.iter().enumerate() {
                if i == j {
                    continue;
                }
                let (x2, y2) = (self.points[c2].e_x, self.points[c2].e_y);
                let dd = (x1 - x2).powi(2) + (y1 - y2).powi(2);
                min_dist = min_dist.min(dd);
                d_list.push((j, dd));
            }

            for (j, dd) in d_list {
                if dd < min_dist * 2.0 {
                    let edge = (i.min(j), i.max(j));
                    effective_edges.insert(edge);
                }
            }
        }

        effective_edges.into_iter().collect()
    }

    /// with_points = true の場合の処理
    fn simulate_query_with_points(
        &self,
        city_list: &[usize],
        _k_sim: usize, // 無視していい
        query_mask_list: &[u32],
        deadline: Instant,
    ) -> (Vec<(usize, f64, f64, f64, f64, f64)>, usize) {
        let l = city_list.len();
        let effective_edges = self.calc_effective_edges(city_list);
        let mut rng = thread_rng();
        let mut params = vec![(0, 0.0, 0.0, 0.0, 0.0, 0.0); l];
        let mut sim_count = 0;
        let mut lp = 0;
        while lp < 50 || Instant::now() < deadline {
            lp += 1;
            let samples: Vec<(f64, f64)> = city_list
                .iter()
                .map(|&c| self.points[c].sample(1)[0])
                .collect();

            let mask = Self::calc_limited_mst(&samples, &effective_edges);

            for i in 0..l {
                if mask[i] == query_mask_list[i] {
                    let (x, y) = samples[i];
                    let (k, sx, sy, sxx, sxy, syy) = params[i];
                    params[i] = (k + 1, sx + x, sy + y, sxx + x * x, sxy + x * y, syy + y * y);
                }
            }
            sim_count += 1;
        }

        (params, sim_count)
    }
    fn maskify(&self, city_list: &[usize], edges: &[(usize, usize)]) -> Vec<u32> {
        let mut index_map = HashMap::new();
        for (i, &c) in city_list.iter().enumerate() {
            index_map.insert(c, i);
        }

        let mut mask = vec![0u32; city_list.len()];
        for &(u, v) in edges {
            let iu = index_map[&u];
            let iv = index_map[&v];
            mask[iu] |= 1 << iv;
            mask[iv] |= 1 << iu;
        }
        mask
    }

    /// with_points = false の場合の処理（bitmaskの履歴）
    fn simulate_query_bitmasks(&self, city_list: &[usize], k_sim: usize) -> Vec<Vec<u32>> {
        let l = city_list.len();
        let effective_edges = self.calc_effective_edges(city_list);
        let mut rng = thread_rng();
        let mut results = vec![vec![]; l];

        for _ in 0..k_sim {
            let samples: Vec<(f64, f64)> = city_list
                .iter()
                .map(|&c| self.points[c].sample(1)[0])
                .collect();

            let mask = Self::calc_limited_mst(&samples, &effective_edges);
            for i in 0..l {
                results[i].push(mask[i]);
            }
        }

        results
    }
    fn simulate_query_var_reduction(&self, city_list: &[usize], num_sim: usize) -> f64 {
        let l = city_list.len();
        let mut rng = rand::thread_rng();

        // クエリ前の var の平方根
        let before_vars: Vec<f64> = city_list
            .iter()
            .map(|&i| (self.points[i].var_xx + self.points[i].var_yy).powf(0.3))
            .collect();

        use std::collections::HashMap;
        let mut param_map: HashMap<(usize, u32), (usize, f64, f64, f64, f64, f64)> = HashMap::new();

        let effective_edges = self.calc_effective_edges(city_list);

        for _ in 0..num_sim {
            let samples: Vec<(f64, f64)> = city_list
                .iter()
                .map(|&c| self.points[c].sample(1)[0])
                .collect();

            let mask = Self::calc_limited_mst(&samples, &effective_edges);

            for i in 0..l {
                let key = (i, mask[i]);
                let (x, y) = samples[i];
                let entry = param_map.entry(key).or_insert((0, 0.0, 0.0, 0.0, 0.0, 0.0));
                entry.0 += 1;
                entry.1 += x;
                entry.2 += y;
                entry.3 += x * x;
                entry.4 += x * y;
                entry.5 += y * y;
            }
        }

        let mut total_reduction = 0.0;

        for i in 0..l {
            let mut weighted_sum = 0.0;
            let mut total_count = 0;

            for (&(idx, _), &(k, sx, sy, sxx, sxy, syy)) in &param_map {
                if idx != i {
                    continue;
                }
                if k < 2 {
                    continue;
                }

                let kf = k as f64;
                let ex = sx / kf;
                let ey = sy / kf;

                let var_xx = (sxx - 2.0 * ex * sx + ex * ex * kf) / (kf - 1.0);
                let var_yy = (syy - 2.0 * ey * sy + ey * ey * kf) / (kf - 1.0);
                let var_sum = (var_xx + var_yy).powf(0.3);

                weighted_sum += var_sum * kf;
                total_count += k;
            }

            if total_count > 0 {
                let after = weighted_sum / total_count as f64;
                total_reduction += before_vars[i] - after;
            }
        }

        total_reduction
    }
    fn calc_limited_mst(points: &[(f64, f64)], edges: &[(usize, usize)]) -> Vec<u32> {
        let l = points.len();
        let mut uf = UnionFind::new(l);
        let mut remain = l - 1;
        let mut mask = vec![0u32; l];

        let mut edge_list: Vec<_> = edges
            .iter()
            .map(|&(i, j)| {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];
                let d = (x1 - x2).powi(2) + (y1 - y2).powi(2);
                (d, i, j)
            })
            .collect();

        edge_list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for &(_, i, j) in &edge_list {
            if uf.unite(i, j) {
                mask[i] |= 1 << j;
                mask[j] |= 1 << i;
                remain -= 1;
                if remain == 0 {
                    break;
                }
            }
        }

        mask
    }
    pub fn pick_query_triangle_approach(
        &mut self,
        deadline: Instant,
        l: usize,
        used_first_two_points: &mut HashSet<u64>,
    ) -> (Vec<usize>, usize, f64) {
        let mut sorted_idx: Vec<usize> = (0..self.data.n).collect();
        sorted_idx.sort_by(|&i, &j| {
            self.points[j]
                .max_var
                .partial_cmp(&self.points[i].max_var)
                .unwrap()
        });

        let mut best_score = -1.0;
        let mut best_query = vec![];
        let mut rng = thread_rng();

        let i0 = sorted_idx[0];
        let p0 = (self.points[i0].e_x, self.points[i0].e_y);

        let mut try_count = 0;
        let mut success_count = 0;
        let is_small_l = self.data.l <= 6;

        for &i1 in &sorted_idx[1..500.min(sorted_idx.len())] {
            try_count += 1;
            if used_first_two_points.contains(&((i0.min(i1) * 1000 + i0.max(i1)) as u64)) {
                continue;
            }

            let p1 = (self.points[i1].e_x, self.points[i1].e_y);
            let base_dist = ((p0.0 - p1.0).powi(2) + (p0.1 - p1.1).powi(2)).sqrt();
            let lower_bound = self.points[i0].max_var * 2.0;
            let lower_bound = lower_bound.min(1000.0);
            let upper_bound = 10000.0 / (self.data.l as f64).sqrt();

            if !(lower_bound < base_dist && base_dist < upper_bound) {
                continue;
            }

            let mut used: HashSet<usize> = [i0, i1].into_iter().collect();
            let mut used_pos = vec![p0, p1];

            let mut stack = vec![];
            for p2 in triangle_candidates(p0, p1) {
                if !is_valid_point(p2, &used_pos, base_dist) {
                    continue;
                }
                if let Some((_score, ni)) = self.pick_block_vertex(p2, &used) {
                    stack.push((_score, ni, p0, p1));
                }
            }

            while stack.len() > 0 && used.len() < l {
                stack.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
                let (_score, i, p1, p2) = stack.pop().unwrap();
                let pos = (self.points[i].e_x, self.points[i].e_y);
                if !is_valid_point(pos, &used_pos, base_dist) {
                    continue;
                }
                used.insert(i);
                used_pos.push(pos);

                for &prev_p in &[p1, p2] {
                    for np in triangle_candidates(pos, prev_p) {
                        if !is_valid_point(np, &used_pos, base_dist) {
                            continue;
                        }
                        if let Some((_score, ni)) = self.pick_block_vertex(np, &used) {
                            stack.push((_score, ni, pos, prev_p));
                        }
                    }
                }
            }

            let query: Vec<usize> = used.iter().cloned().collect();
            let v_list: Vec<f64> = query.iter().map(|&i| self.points[i].max_var).collect();
            let score = if is_small_l {
                self.simulate_query_var_reduction(&query, 200)
            } else {
                let sim_result = self.simulate_query_bitmasks(&query, 50);
                compute_info_gain(&sim_result, &v_list)
            };

            if score > best_score {
                best_score = score;
                best_query = query;
            }
            success_count += 1;

            if success_count >= 999 || (success_count >= 3 && Instant::now() > deadline) {
                used_first_two_points.insert((i0.min(i1) * 1000 + i0.max(i1)) as u64);
                break;
            }
        }

        best_query.sort();
        (best_query, success_count, best_score)
    }
    fn pick_block_vertex(
        &self,
        (px, py): (f64, f64),
        used_set: &std::collections::HashSet<usize>,
    ) -> Option<(f64, usize)> {
        if !(0.0..=10000.0).contains(&px) || !(0.0..=10000.0).contains(&py) {
            return None;
        }
        let bx = (px / 500.0).floor() as usize;
        let by = (py / 500.0).floor() as usize;
        let b_id = (bx.min(19), by.min(19));
        let cands = self.block2points.get(&b_id)?;

        let mut best = None;
        let mut best_val = std::f64::MAX;

        for &i in cands {
            if used_set.contains(&i) {
                continue;
            }
            let dx = self.points[i].e_x - px;
            let dy = self.points[i].e_y - py;
            let dist = (dx * dx + dy * dy).sqrt();
            let var = self.points[i].max_var;
            let val = if var < 1.0 {
                dist
            } else {
                dist / var.powf(0.20)
            };
            if val < best_val {
                best_val = val;
                best = Some(i);
            }
        }

        best.map(|i| (best_val, i))
    }
}
fn is_valid_point(p: (f64, f64), used_p: &Vec<(f64, f64)>, base_dist: f64) -> bool {
    let (x, y) = p;
    if x < 0.0 || x > 10000.0 || y < 0.0 || y > 10000.0 {
        return false;
    }
    for &(xx, yy) in used_p {
        if (x - xx).powi(2) + (y - yy).powi(2) < base_dist.powi(2) / 2.0 {
            return false;
        }
    }
    true
}
fn triangle_candidates(p0: (f64, f64), p1: (f64, f64)) -> Vec<(f64, f64)> {
    let (x0, y0) = p0;
    let (x1, y1) = p1;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let d2 = dx * dx + dy * dy;
    if d2 < 1e-9 {
        return vec![];
    }
    let d = d2.sqrt();
    let mx = (x0 + x1) / 2.0;
    let my = (y0 + y1) / 2.0;
    let h = (3.0_f64).sqrt() / 2.0 * d;
    let ndx = -dy / d;
    let ndy = dx / d;
    let p2a = (mx + h * ndx, my + h * ndy);
    let p2b = (mx - h * ndx, my - h * ndy);
    vec![p2a, p2b]
}
fn hash_query(v: &[usize]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &x in v {
        h ^= x as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn run(state: &mut State, global_start_time: Instant) {
    let q = state.data.q;
    let local_start_time = Instant::now();
    let time_limit = Duration::from_secs_f64(TIME_LIMIT) - (local_start_time - global_start_time);

    let is_small_l = state.data.l <= 15;
    let mut saved_queries: Vec<(Vec<usize>, Vec<u32>)> = vec![];
    let mut used_first_two_points = HashSet::new();
    let base = if is_small_l {
        0.6 * time_limit.as_secs_f64() as f64
    } else {
        0.75 * time_limit.as_secs_f64() as f64
    };
    for query_id in 0..q {
        let sim_deadline =
            local_start_time + Duration::from_secs_f64(base * (query_id as f64 + 0.7) / q as f64);

        let (candidate, tri_loop_count, best_info_gain) = state.pick_query_triangle_approach(
            sim_deadline,
            state.data.l,
            &mut used_first_two_points,
        );

        let query_hash = hash_query(&candidate);

        let mask = state.do_query(&candidate);
        let mask_bit = state.maskify(&candidate, &mask);
        if is_small_l {
            saved_queries.push((candidate.clone(), mask_bit.clone()));
        }

        let update_deadline =
            local_start_time + Duration::from_secs_f64(base * (query_id as f64 + 1.0) / q as f64);
        let (params, sim_loop_count) =
            state.simulate_query_with_points(&candidate, 1000, &mask_bit, update_deadline);
        if CODETEST == 1 && query_id == 0 {
            eprintln!("params (query_id = 0):");
            for (i, param) in candidate.iter().zip(&params) {
                let (k, sx, sy, sxx, sxy, syy) = param;
                eprintln!(
                    "  idx = {}, k = {}, sx = {:.1}, sy = {:.1}, sxx = {:.1}, sxy = {:.1}, syy = {:.1}",
                    i, k, sx, sy, sxx, sxy, syy
                );
            }
        }
        for (i, param) in candidate.iter().zip(params.iter()) {
            if CODETEST == 1 && (query_id == 0 || query_id == 1 || query_id == 399) {
                let ps_before = &state.points[*i];
                let (k, sx, sy, sxx, sxy, syy) = param;

                // eprintln!("--- Point {} ---", i);
                // eprintln!("param: k = {}, sx = {:.1}, sy = {:.1}, sxx = {:.1}, sxy = {:.1}, syy = {:.1}", k, sx, sy, sxx, sxy, syy);
                // eprintln!("before: e=({:.2},{:.2}), var_xx={:.2}, var_yy={:.2}, var_xy={:.2}", ps_before.e_x, ps_before.e_y, ps_before.var_xx, ps_before.var_yy, ps_before.var_xy);

                // lx, rx, ly, ry の表示
                // eprintln!("F {} {} {} {}", ps_before.lx, ps_before.rx, ps_before.ly, ps_before.ry);

                // Actualの (x, y) 座標を表示
                // eprintln!("R {} {}", state.data.actual.as_ref().unwrap()[*i].0, state.data.actual.as_ref().unwrap()[*i].1);
                // eprintln!("B {} {} {} {} {}", ps_before.e_x, ps_before.e_y, ps_before.var_xx, ps_before.var_xy, ps_before.var_yy);
            }
            state.points[*i].update_by_query_param(*param);
            if CODETEST == 1 && (query_id == 0 || query_id == 1 || query_id == 399) {
                let ps_after = &state.points[*i];
                // eprintln!("A {} {} {} {} {}", ps_after.e_x, ps_after.e_y, ps_after.var_xx, ps_after.var_xy, ps_after.var_yy);
                // eprintln!("after : e=({:.2},{:.2}), var_xx={:.2}, var_yy={:.2}, var_xy={:.2}", ps_after.e_x, ps_after.e_y, ps_after.var_xx, ps_after.var_yy, ps_after.var_xy);
            }
        }

        state.update_block2points();
        if CODETEST == 1 {
            let mut err = std::io::stderr().lock();
            writeln!(
                err,
                "query_id: {}, tri_loop: {}, sim_loop: {}, best_info_gain: {}",
                query_id, tri_loop_count, sim_loop_count, best_info_gain
            )
            .unwrap();
        }
    }
    if is_small_l {
        let second_pass_start = Instant::now();
        let per_query_time = TIME_LIMIT * 0.15 / q as f64;
        for (i, (candidate, mask_bit)) in saved_queries.iter().enumerate() {
            let deadline =
                second_pass_start + Duration::from_secs_f64(per_query_time * (i + 1) as f64);
            let (params, _sim_loop) =
                state.simulate_query_with_points(candidate, 1000, mask_bit, deadline);
            for (j, param) in candidate.iter().zip(params.iter()) {
                if CODETEST == 1 && (i == 0 || i == 1 || i == 399) {
                    let ps_before = &state.points[*j];
                    let (k, sx, sy, sxx, sxy, syy) = param;

                    // eprintln!("--- Point {} ---", i);
                    // eprintln!("param: k = {}, sx = {:.1}, sy = {:.1}, sxx = {:.1}, sxy = {:.1}, syy = {:.1}", k, sx, sy, sxx, sxy, syy);
                    eprintln!(
                        "before: e=({:.2},{:.2}), var_xx={:.2}, var_yy={:.2}, var_xy={:.2}",
                        ps_before.e_x,
                        ps_before.e_y,
                        ps_before.var_xx,
                        ps_before.var_yy,
                        ps_before.var_xy
                    );

                    // lx, rx, ly, ry の表示
                    // eprintln!("F {} {} {} {}", ps_before.lx, ps_before.rx, ps_before.ly, ps_before.ry);

                    // Actualの (x, y) 座標を表示
                    // eprintln!("R {} {}", state.data.actual.as_ref().unwrap()[*i].0, state.data.actual.as_ref().unwrap()[*i].1);
                    // eprintln!("B {} {} {} {} {}", ps_before.e_x, ps_before.e_y, ps_before.var_xx, ps_before.var_xy, ps_before.var_yy);
                }

                state.points[*j].update_by_query_param(*param);
                if CODETEST == 1 && (i == 0 || i == 1 || i == 399) {
                    let ps_after = &state.points[*j];
                    // eprintln!("A {} {} {} {} {}", ps_after.e_x, ps_after.e_y, ps_after.var_xx, ps_after.var_xy, ps_after.var_yy);
                    eprintln!(
                        "after : e=({:.2},{:.2}), var_xx={:.2}, var_yy={:.2}, var_xy={:.2}",
                        ps_after.e_x,
                        ps_after.e_y,
                        ps_after.var_xx,
                        ps_after.var_yy,
                        ps_after.var_xy
                    );
                }
            }
        }
    }
    if false {
        // 座標の出力
        println!("---");
        for ps in &state.points {
            println!(
                "{:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
                ps.e_x, ps.e_y, ps.var_xx, ps.var_xy, ps.var_yy, ps.max_var
            );
        }
        println!("---");
    }
}
fn calc_groups(group_id: &[usize], m: usize) -> Vec<Vec<usize>> {
    let mut groups = vec![vec![]; m];
    for (i, &g) in group_id.iter().enumerate() {
        groups[g].push(i);
    }
    groups
}

fn calc_score(group_id: &[usize], xx: &[f64], yy: &[f64], group_size_factor: &[f64]) -> f64 {
    let m = group_size_factor.len();
    let groups = calc_groups(group_id, m);
    let mut score = 0.0;

    for (i, group) in groups.iter().enumerate() {
        let mut s = 0.0;
        for j in 0..group.len() {
            for k in 0..j {
                let u = group[j];
                let v = group[k];
                let dx = (xx[u] - xx[v]) as f64;
                let dy = (yy[u] - yy[v]) as f64;
                s += dx * dx + dy * dy;
            }
        }
        score += s / group_size_factor[i];
    }

    score
}

fn connection_sa(
    state: &State,
    total_time_limit: f64,
    global_start_time: Instant,
) -> Vec<Vec<usize>> {
    let data = &state.data;
    let n = data.n;
    let m = data.m;
    let local_start_time = Instant::now();
    let mut group_id = vec![0; n];
    let mut idx = 0;
    for (i, &g) in data.g.iter().enumerate() {
        for _ in 0..g {
            group_id[idx] = i;
            idx += 1;
        }
    }

    let mut rng = rand::thread_rng();
    group_id.shuffle(&mut rng);

    // let group_size = data.g.clone();
    // group_size はこの時点で f64 にしておく
    let group_size = data.g.iter().map(|&a| a as f64).collect::<Vec<f64>>();
    let group_size_factor: Vec<f64> = group_size.iter().map(|&a| (a as f64).powf(-1.2)).collect();
    let mut group_x_sum = vec![0.0; m];
    let mut group_y_sum = vec![0.0; m];
    let mut xx = vec![0.0; n]; // ← f64
    let mut yy = vec![0.0; n];

    for i in 0..n {
        xx[i] = state.points[i].e_x.round();
        yy[i] = state.points[i].e_y.round();
        let g = group_id[i];
        group_x_sum[g] += xx[i];
        group_y_sum[g] += yy[i];
    }

    let mut score = calc_score(&group_id, &xx, &yy, &group_size_factor);
    // local_start_time から、プログラム開始＋TIME_LIMIT までの時間を計算
    let sa_time = total_time_limit - (local_start_time - global_start_time).as_secs_f64();
    let time_limit = sa_time;
    let mut loop_cnt = 0;
    let mut success_count = 0;

    let temp_start = 1e6_f64;
    let temp_end = 1e3_f64;

    let mut temp = temp_start;
    while true {
        if loop_cnt % 100 == 0 {
            let current_time = Instant::now();
            let time_ratio = (current_time - local_start_time).as_secs_f64() / time_limit;
            if time_ratio > 1.0 {
                break;
            }
            temp = temp_start * (temp_end / temp_start).powf(time_ratio);
        }

        let mut i = 0;
        let mut j = 0;
        while i == j {
            i = rng.gen_range(0..n);
            j = rng.gen_range(0..n);
        }
        let g1 = group_id[i];
        let g2 = group_id[j];
        if g1 == g2 {
            loop_cnt += 1;
            continue;
        }

        let x1 = xx[i];
        let y1 = yy[i];
        let x2 = xx[j];
        let y2 = yy[j];

        let mut d = ((x1 * x1 + y1 * y1) - (x2 * x2 + y2 * y2))
            * ((group_size[g2] - 1.0) * group_size_factor[g2]
                - (group_size[g1] - 1.0) * group_size_factor[g1]);

        d -= 2.0
            * (((x1 - x2) * (group_x_sum[g2] - x2) + (y1 - y2) * (group_y_sum[g2] - y2))
                * group_size_factor[g2]
                - ((x1 - x2) * (group_x_sum[g1] - x1) + (y1 - y2) * (group_y_sum[g1] - y1))
                    * group_size_factor[g1]);

        if d < 0.0 || (d < temp * 7.0 && rng.gen::<f64>() < (-d / temp).exp()) {
            group_id[i] = g2;
            group_id[j] = g1;
            group_x_sum[g1] -= x1 - x2;
            group_y_sum[g1] -= y1 - y2;
            group_x_sum[g2] += x1 - x2;
            group_y_sum[g2] += y1 - y2;
            score += d;
            success_count += 1;
        }

        loop_cnt += 1;
    }
    if CODETEST == 1 {
        eprintln!("SA loop_cnt = {}", loop_cnt);
        eprintln!("SA success_count = {}", success_count);
    }

    calc_groups(&group_id, m)
}

fn calc_mst(groups: &[Vec<usize>], state: &State) -> Vec<Vec<(usize, usize)>> {
    let n = state.data.n;
    let mut dist_matrix = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..i {
            let dx = state.points[i].e_x - state.points[j].e_x;
            let dy = state.points[i].e_y - state.points[j].e_y;
            dist_matrix[i][j] = dx * dx + dy * dy;
            dist_matrix[j][i] = dist_matrix[i][j];
        }
    }

    fn sub_calc(group: &[usize], dist_matrix: &[Vec<f64>]) -> Vec<(usize, usize)> {
        let mut edges = vec![];
        for i in 0..group.len() {
            for j in 0..i {
                let u = group[i];
                let v = group[j];
                edges.push((dist_matrix[u][v], u.min(v), u.max(v)));
            }
        }
        edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut uf = UnionFind::new(group.len());
        let mut idx = HashMap::new();
        for (i, &v) in group.iter().enumerate() {
            idx.insert(v, i);
        }

        let mut ret = vec![];
        for &(_, u, v) in &edges {
            let ui = idx[&u];
            let vi = idx[&v];
            if uf.unite(ui, vi) {
                ret.push((u, v));
            }
        }
        ret
    }

    groups.iter().map(|g| sub_calc(g, &dist_matrix)).collect()
}
fn calc_actual_score(ans_edges: &[Vec<(usize, usize)>], data: &Data) -> i64 {
    // score は整数で計算
    let mut score = 0 as i64;
    let actual = data.actual.as_ref().unwrap();
    for edges in ans_edges {
        for &(u, v) in edges {
            let (ux, uy) = actual[u];
            let (vx, vy) = actual[v];
            let dx = ux - vx;
            let dy = uy - vy;
            score += (dx * dx + dy * dy).sqrt().floor() as i64;
        }
    }
    score
}

fn answer(groups: &[Vec<usize>], ans_edges: &[Vec<(usize, usize)>]) {
    println!("!");
    for (group, edges) in groups.iter().zip(ans_edges.iter()) {
        // グループの頂点を出力
        for (i, &node) in group.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", node);
        }
        println!();

        // MSTの辺を出力
        for &(u, v) in edges {
            println!("{} {}", u, v);
        }
    }
}
fn fine_tune_connection_sa(groups: &mut Vec<Vec<usize>>, state: &State, total_time_limit: f64) {
    use rand::Rng;
    use std::time::Instant;

    let start_time = Instant::now();
    let time_limit = total_time_limit * 0.1;
    let n = state.data.n;
    let m = state.data.m;
    let mut rng = rand::thread_rng();
    let mut dist = vec![vec![0.0; n]; n];
    if m == 1 {
        return;
    }
    for i in 0..n {
        for j in 0..n {
            let dx = state.points[i].e_x - state.points[j].e_x;
            let dy = state.points[i].e_y - state.points[j].e_y;
            dist[i][j] = (dx * dx + dy * dy).sqrt();
        }
    }

    let temp_start: f64 = 10.0;
    let temp_end: f64 = 1.0;

    fn mst_cost(group: &[usize], dist: &Vec<Vec<f64>>) -> f64 {
        let mut edges = vec![];
        for i in 0..group.len() {
            for j in 0..i {
                let u = group[i];
                let v = group[j];
                edges.push((dist[u][v], i, j));
            }
        }
        edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut uf = UnionFind::new(group.len());
        let mut cost = 0.0;
        for &(d, i, j) in &edges {
            if uf.unite(i, j) {
                cost += d;
            }
        }
        cost
    }

    let mut loop_cnt = 0;
    let mut success_count = 0;
    while start_time.elapsed().as_secs_f64() < time_limit {
        let time_ratio = start_time.elapsed().as_secs_f64() / time_limit;
        let temp = temp_start * (temp_end / temp_start).powf(time_ratio);

        let mut gi = rng.gen_range(0..m);
        let mut gj = rng.gen_range(0..m);
        while gi == gj || groups[gi].is_empty() || groups[gj].is_empty() {
            gi = rng.gen_range(0..m);
            gj = rng.gen_range(0..m);
        }

        let i_idx = rng.gen_range(0..groups[gi].len());
        let j_idx = rng.gen_range(0..groups[gj].len());
        let u = groups[gi][i_idx];
        let v = groups[gj][j_idx];

        let mut new_gi = groups[gi].clone();
        let mut new_gj = groups[gj].clone();
        new_gi[i_idx] = v;
        new_gj[j_idx] = u;

        let old_cost = mst_cost(&groups[gi], &dist) + mst_cost(&groups[gj], &dist);
        let new_cost = mst_cost(&new_gi, &dist) + mst_cost(&new_gj, &dist);

        let delta = new_cost - old_cost;
        if delta < 0.0 || rng.gen::<f64>() < (-delta / temp).exp() {
            groups[gi] = new_gi;
            groups[gj] = new_gj;
            success_count += 1;
        }

        loop_cnt += 1;
    }

    if CODETEST == 1 {
        eprintln!("[FineTune SA] loop_cnt = {}", loop_cnt);
        eprintln!("[FineTune SA] success_count = {}", success_count);
    }
}
fn main() -> io::Result<()> {
    let global_start_time = Instant::now();
    let data = Data::read_input()?;
    let mut state = State::new(data);
    run(&mut state, global_start_time);
    if CODETEST == 1 {
        eprintln!(
            "Total Var: {:.3}",
            (state.calc_total_var() / state.data.n as f64)
                .sqrt()
                .round() as usize
        );
        eprintln!(
            "AvgDiff (RMSE): {:.3}",
            state.calc_act_diff_average_square()
        );
        eprintln!("AvgDiff (Abs): {:.3}", state.calc_act_diff_average_abs());
    }

    let mut groups = connection_sa(&state, TIME_LIMIT, global_start_time);
    let ans = calc_mst(&groups, &state);
    if CODETEST == 1 {
        eprintln!("Initial MST Cost: {}", calc_actual_score(&ans, &state.data));
    }
    if false {
        fine_tune_connection_sa(&mut groups, &state, TIME_LIMIT);
        let ans = calc_mst(&groups, &state);
    }
    answer(&groups, &ans);
    if CODETEST == 1 {
        let final_score = calc_actual_score(&ans, &state.data);
        eprintln!("Final MST Cost: {}", final_score);
    }
    if CODETEST == 1 {
        eprintln!("--- Point Estimates (rounded) ---");
        for (i, p) in state.points.iter().enumerate() {
            let ex = p.e_x.round() as i64;
            let ey = p.e_y.round() as i64;
            let vxx = p.var_xx.round() as i64;
            let vxy = p.var_xy.round() as i64;
            let vyy = p.var_yy.round() as i64;
            eprintln!("{} {} {} {} {} {}", i, ex, ey, vxx, vxy, vyy);
        }
    }
    Ok(())
}
