#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{common::get_time, input::read_input};
use construct::Forest;
use estimator::Estimator;
use input::Input;

mod common;
mod construct;
mod coord;
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
    let estimator = Estimator::new(input);
    Forest::solve(input, &estimator.xy, TLE);
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
