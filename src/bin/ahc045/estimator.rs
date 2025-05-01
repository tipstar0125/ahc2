use itertools::Itertools;
use proconio::input_interactive;
use rand_pcg::Pcg64Mcg;
use rustc_hash::FxHashSet;

use crate::{coord::Coord, input::Input};

pub struct Estimator {
    pub rng: Pcg64Mcg,
    pub input: Input,
    pub xy: Vec<Coord>,
    pub dist: Vec<Vec<usize>>,
    pub mst_edges: Vec<Vec<(usize, usize)>>,
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
            mst_edges: vec![],
        }
    }
    pub fn query(mut self) -> Self {
        let nodes_sorted_by_error = (0..self.input.N)
            .sorted_by_key(|&i| self.input.rects[i].long_side())
            .rev()
            .collect_vec();

        let mut used_mst_edges = FxHashSet::default();
        let mut used_cnt = vec![0; self.input.N];

        for &first_node_idx in nodes_sorted_by_error.iter() {
            let mut query_nodes = vec![first_node_idx];
            let mut used_edges = FxHashSet::default();
            get_query_nodes(
                &mut query_nodes,
                &mut used_edges,
                &used_mst_edges,
                &used_cnt,
                &self.xy,
                &self.input,
                &mut self.rng,
            );
            if query_nodes.len() != self.input.L {
                eprintln!("skip");
                continue;
            }
            for node_idx in query_nodes.iter() {
                used_cnt[*node_idx] += 1;
            }
            println!("? {} {}", query_nodes.len(), query_nodes.iter().join(" "));
            input_interactive! {
                uv: [(usize, usize); query_nodes.len() - 1],
            }
            for &(u, v) in uv.iter() {
                used_mst_edges.insert((u, v));
                used_mst_edges.insert((v, u));
            }
            self.mst_edges.push(uv);
            if self.mst_edges.len() == self.input.Q {
                break;
            }
        }
        eprintln!("{}", used_cnt.iter().join(" "));
        self
    }
}

fn get_query_nodes(
    query_nodes: &mut Vec<usize>,
    used_edges: &mut FxHashSet<(usize, usize)>,
    used_mst_edges: &FxHashSet<(usize, usize)>,
    used_cnt: &Vec<usize>,
    xy: &Vec<Coord>,
    input: &Input,
    rng: &mut Pcg64Mcg,
) -> bool {
    const MIN_DIST: usize = 1000;
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
            // 既にクエリでMSTの辺と判定された辺はクエリには含まない
            if used_mst_edges.contains(&(query_nodes[0], second_node_idx)) {
                continue;
            }
            let dist = xy[query_nodes[0]].euclidean_dist(xy[second_node_idx]);
            if dist < MIN_DIST {
                continue;
            }
            query_nodes.push(second_node_idx);
            if get_query_nodes(
                query_nodes,
                used_edges,
                used_mst_edges,
                used_cnt,
                xy,
                input,
                rng,
            ) {
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
                        // 既にクエリでMSTの辺と判定された辺はクエリには含まない
                        if used_mst_edges.contains(&(query_nodes[i], node))
                            || used_mst_edges.contains(&(query_nodes[j], node))
                        {
                            continue;
                        }
                        // 同じような位置にあるノードはクエリに含めない(全てのノードから一定距離以上離れていること)
                        if (0..n)
                            .into_iter()
                            .all(|k| xy[query_nodes[k]].euclidean_dist(xy[node]) >= MIN_DIST)
                        {
                            query_nodes.push(node);
                            if get_query_nodes(
                                query_nodes,
                                used_edges,
                                used_mst_edges,
                                used_cnt,
                                xy,
                                input,
                                rng,
                            ) {
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
    const RANGE: usize = 1000;
    let x_lower = coord.x.saturating_sub(RANGE / 2);
    let x_upper = (coord.x + RANGE / 2).min(10000);
    let y_lower = coord.y.saturating_sub(RANGE / 2);
    let y_upper = (coord.y + RANGE / 2).min(10000);
    let x_range = input.x_positions.range(x_lower..=x_upper);
    let y_range = input.y_positions.range(y_lower..=y_upper);
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

    x_range_points
        .intersection(&y_range_points)
        .cloned()
        .collect()
}
