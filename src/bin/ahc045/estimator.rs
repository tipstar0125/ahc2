use itertools::Itertools;
use proconio::input_interactive;

use crate::{
    coord::{calc_dist2, Coord},
    input::Input,
};

pub struct Estimator {
    pub subset: Vec<Vec<usize>>,
    pub edges: Vec<Vec<(usize, usize)>>,
    pub dist: Vec<Vec<f64>>,
}

impl Estimator {
    pub fn new(input: &Input) -> Self {
        // 座標範囲の中心を点の座標と仮定
        let xy_center = input
            .range
            .iter()
            .map(|(lx, rx, ly, ry)| Coord::new((lx + rx) / 2, (ly + ry) / 2))
            .collect::<Vec<Coord>>();

        // クエリに選ばれる点の基準は誤差範囲が大きい点を優先する
        let mut delta = input
            .range
            .iter()
            .enumerate()
            .map(|(i, (lx, rx, ly, ry))| (rx - lx + ry - ly, i))
            .collect::<Vec<_>>();
        delta.sort();
        delta.reverse();
        delta.truncate(input.Q);

        let mut subset = vec![];
        let mut edges = vec![];
        let mut count_included = vec![vec![0; input.N]; input.N];
        let mut count_appear = vec![vec![0; input.N]; input.N];

        // クエリ対象の点をまんべんなく選択し、最小全域木の辺となる頂点間の長さは
        // 比較的距離が短いと予想される
        // そのため、その辺が選ばれる回数をカウントし、カウントされる割合を長さに換算する

        for base_idx in delta.iter().map(|(_, i)| *i) {
            let base = xy_center[base_idx];

            let mut candidates = vec![];
            for (i, &coord) in xy_center.iter().enumerate() {
                if i == base_idx {
                    continue;
                }
                let dist = calc_dist2(base, coord);
                candidates.push((dist, i));
            }
            candidates.sort();
            candidates.truncate(input.L - 1);
            candidates.push((0, base_idx));
            let mut selected = candidates.iter().map(|(_, i)| *i).collect::<Vec<_>>();
            selected.sort();

            for i in selected.iter() {
                for j in selected.iter() {
                    if i == j {
                        continue;
                    }
                    count_appear[*i][*j] += 1;
                }
            }

            println!(
                "? {} {}",
                selected.len(),
                selected.iter().map(|i| i).join(" ")
            );
            input_interactive! {
                uv: [(usize, usize); selected.len() - 1],
            }

            let mut edge = vec![];
            for (mut u, mut v) in uv {
                if u > v {
                    std::mem::swap(&mut u, &mut v);
                }
                edge.push((u, v));
                count_included[u][v] += 1;
                count_included[v][u] += 1;
            }

            subset.push(selected);
            edges.push(edge);
        }

        let mut dist = vec![vec![0.0; input.N]; input.N];
        for i in 0..input.N {
            let coord_i = xy_center[i];
            for j in 0..input.N {
                if i == j {
                    continue;
                }
                let coord_j = xy_center[j];
                let score = if count_appear[i][j] == 0 {
                    0.0
                } else {
                    count_included[i][j] as f64 / count_appear[i][j] as f64
                };
                let d = (calc_dist2(coord_i, coord_j) as f64).sqrt();
                dist[i][j] = d - score * 3000.0;
            }
        }

        Self {
            subset,
            edges,
            dist,
        }
    }
}
