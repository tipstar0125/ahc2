#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{common::get_time, input::read_input};
use coord::{calc_dist2, Coord};
use cut::CutTree;
use input::Input;
use itertools::Itertools;

mod common;
mod construct;
mod coord;
mod cut;
mod dsu;
mod estimator;
mod input;
mod test;

/*
<問題概要>
- 座標平面上にN個の点がある
- 各点の座標の真値は与えられず、存在する範囲が与えられる
- 座標の真値は0-10000の範囲である
- これらの点をM個のグループに分ける
- 各グループはちょうどG[i]個の点から構成される必要がある
- 以下のクエリを最大でQ回実行できる
    - 点群の中から最大でLの点を選択しクエリを投げる
    - クエリの結果、選択した点群の座標の真値をもとに最小全域木を構成した場合の隣接リストが返ってくる
- 各グループを木構造として結合した場合の辺の長さの総和を最小化する
*/

const TLE: f64 = 1.9; // 時間制限

fn solve(input: &Input) {
    let xy_center = input
        .range
        .iter()
        .map(|(lx, rx, ly, ry)| Coord::new((lx + rx) / 2, (ly + ry) / 2))
        .collect_vec();
    let mut dist = vec![vec![0.0; input.N]; input.N];
    for i in 0..input.N {
        for j in 0..input.N {
            dist[i][j] = calc_dist2(xy_center[i], xy_center[j]) as f64;
            dist[j][i] = dist[i][j];
        }
    }
    let mut cut_tree = CutTree::new(input, &dist);
    dist = cut_tree.query(input);
    cut_tree.cut(input);
    cut_tree.make_rest(input, &dist);
    cut_tree.climbing2(input, &dist, TLE);
    cut_tree.output(&dist);
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
