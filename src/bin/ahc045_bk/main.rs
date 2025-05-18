#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{common::get_time, input::read_input};
use common::eprint_yellow;
use coord::Coord;
use cut::CutTree;
use estimator::Estimator;
use input::Input;

mod common;
mod coord;
mod cut;
mod dsu;
mod estimator;
mod input;
mod test;
mod vis;

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

fn solve(input: &Input) -> Vec<Coord> {
    let mut estimator = Estimator::new(&input);
    eprint_yellow(format!("estimator init elapsed = {:.3}", get_time()).as_str());
    estimator.climbing(&input, 0.5);
    eprint_yellow(format!("climbing elapsed = {:.3}", get_time()).as_str());
    let dist = estimator.gibbs_sampling(&input, TLE);
    eprint_yellow(format!("gibbs sampling elapsed = {:.3}", get_time()).as_str());
    let mut cut_tree = CutTree::new(input, &dist);
    cut_tree.cut(input);
    eprint_yellow(format!("cut elapsed = {:.3}", get_time()).as_str());
    cut_tree.make_rest(input, &dist);
    eprint_yellow(format!("make rest elapsed = {:.3}", get_time()).as_str());
    cut_tree.annealing(input, &dist, TLE);
    eprint_yellow(format!("annealing elapsed = {:.3}", get_time()).as_str());
    cut_tree.output(&dist);
    estimator.xy
}

fn main() {
    get_time();
    let input = read_input();

    let estimate_points = solve(&input);
    let output = Output { estimate_points };
    // vis::visualizer(input, output, 100);

    // let delta = 500;
    // let pos0_center = Coord::new(3000, 7000);
    // let range0 = (
    //     pos0_center.x - delta,
    //     pos0_center.x + delta,
    //     pos0_center.y - delta,
    //     pos0_center.y + delta,
    // );
    // let pos1_center = Coord::new(7000, 7000);
    // let range1 = (
    //     pos1_center.x - delta,
    //     pos1_center.x + delta,
    //     pos1_center.y - delta,
    //     pos1_center.y + delta,
    // );
    // let pos2_center = rotate_120deg(pos0_center, pos1_center);
    // let range2 = (
    //     pos2_center.x - delta,
    //     pos2_center.x + delta,
    //     pos2_center.y - delta,
    //     pos2_center.y + delta,
    // );

    // let pos3_center = rotate_120deg(pos2_center, pos1_center);
    // let range3 = (
    //     pos3_center.x - delta,
    //     pos3_center.x + delta,
    //     pos3_center.y - delta,
    //     pos3_center.y + delta,
    // );

    // let pos4_center = rotate_120deg(pos0_center, pos2_center);
    // let range4 = (
    //     pos4_center.x - delta,
    //     pos4_center.x + delta,
    //     pos4_center.y - delta,
    //     pos4_center.y + delta,
    // );

    // let mut rng = Pcg64Mcg::new(100);
    // let pos0 = Coord::new(
    //     rng.gen_range(range0.0..=range0.1),
    //     rng.gen_range(range0.2..=range0.3),
    // );
    // let pos1 = Coord::new(
    //     rng.gen_range(range1.0..=range1.1),
    //     rng.gen_range(range1.2..=range1.3),
    // );
    // let pos2 = Coord::new(
    //     rng.gen_range(range2.0..=range2.1),
    //     rng.gen_range(range2.2..=range2.3),
    // );

    // let pos3 = Coord::new(
    //     rng.gen_range(range3.0..=range3.1),
    //     rng.gen_range(range3.2..=range3.3),
    // );
    // let pos4 = Coord::new(
    //     rng.gen_range(range4.0..=range4.1),
    //     rng.gen_range(range4.2..=range4.3),
    // );
    // input.xy = vec![pos0, pos1, pos2, pos3, pos4];
    // input.range = vec![range0, range1, range2, range3, range4];

    // let mut edges = vec![];
    // for i in 0..input.xy.len() {
    //     for j in i + 1..input.xy.len() {
    //         edges.push((i, j, calc_dist2(input.xy[i], input.xy[j]) as f64));
    //     }
    // }
    // edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    // let true_edges = edges
    //     .into_iter()
    //     .take(input.xy.len() - 1)
    //     .map(|(i, j, _)| (i, j))
    //     .collect_vec();

    // let mu0 = [pos0_center.x as f64, pos0_center.y as f64];
    // let mu1 = [pos1_center.x as f64, pos1_center.y as f64];
    // let mu2 = [pos2_center.x as f64, pos2_center.y as f64];
    // let mu3 = [pos3_center.x as f64, pos3_center.y as f64];
    // let mu4 = [pos4_center.x as f64, pos4_center.y as f64];
    // let sigma = delta as f64 * delta as f64 / 4.0;
    // let sigma = [[sigma, 0.0], [0.0, sigma]];

    // let mut ok_points = vec![];
    // let mut ok_points0 = vec![];
    // let mut ok_points1 = vec![];
    // let mut ok_points2 = vec![];
    // let mut ok_points3 = vec![];
    // let mut ok_points4 = vec![];
    // for _ in 0..500000 {
    //     let (x0, y0) = sample_2d_normal(mu0, sigma, &mut rng);
    //     let pos0_ = Coord::new(x0 as usize, y0 as usize);
    //     let (x1, y1) = sample_2d_normal(mu1, sigma, &mut rng);
    //     let pos1_ = Coord::new(x1 as usize, y1 as usize);
    //     let (x2, y2) = sample_2d_normal(mu2, sigma, &mut rng);
    //     let pos2_ = Coord::new(x2 as usize, y2 as usize);
    //     let (x3, y3) = sample_2d_normal(mu3, sigma, &mut rng);
    //     let pos3_ = Coord::new(x3 as usize, y3 as usize);
    //     let (x4, y4) = sample_2d_normal(mu4, sigma, &mut rng);
    //     let pos4_ = Coord::new(x4 as usize, y4 as usize);
    //     let pos_ = vec![pos0_, pos1_, pos2_, pos3_, pos4_];
    //     let mut edges = vec![];
    //     for i in 0..pos_.len() {
    //         for j in i + 1..pos_.len() {
    //             edges.push((i, j, calc_dist2(pos_[i], pos_[j]) as f64));
    //         }
    //     }
    //     edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    //     let edges = edges
    //         .into_iter()
    //         .take(input.xy.len() - 1)
    //         .map(|(i, j, _)| (i, j))
    //         .collect_vec();
    //     if edges == true_edges {
    //         eprintln!("ok");
    //         ok_points.push(pos0_);
    //         ok_points.push(pos1_);
    //         ok_points.push(pos2_);
    //         ok_points.push(pos3_);
    //         ok_points.push(pos4_);
    //         ok_points0.push(pos0_);
    //         ok_points1.push(pos1_);
    //         ok_points2.push(pos2_);
    //         ok_points3.push(pos3_);
    //         ok_points4.push(pos4_);
    //     }
    // }

    // let mut ellipses = vec![];
    // let mu0 = compute_mean(&ok_points0);
    // let mu1 = compute_mean(&ok_points1);
    // let mu2 = compute_mean(&ok_points2);
    // let mu3 = compute_mean(&ok_points3);
    // let mu4 = compute_mean(&ok_points4);
    // let sigma0 = compute_covariance(&ok_points0);
    // let sigma1 = compute_covariance(&ok_points1);
    // let sigma2 = compute_covariance(&ok_points2);
    // let sigma3 = compute_covariance(&ok_points3);
    // let sigma4 = compute_covariance(&ok_points4);
    // ellipses.push((mu0, sigma0));
    // ellipses.push((mu1, sigma1));
    // ellipses.push((mu2, sigma2));
    // ellipses.push((mu3, sigma3));
    // ellipses.push((mu4, sigma4));

    // eprintln!("elapsed = {:.3}", get_time());

    // let output = Output {
    //     true_edges,
    //     ok_points,
    //     ellipses,
    // };
    // vis::visualizer(input, output, 100);

    eprintln!("Elapsed time = {:.3}", get_time());
}

const SIN120: f64 = 0.8660254037844386;
const COS120: f64 = -0.5;

fn rotate_120deg(pos0: Coord, pos1: Coord) -> Coord {
    let dx = pos0.x as f64 - pos1.x as f64;
    let dy = pos0.y as f64 - pos1.y as f64;
    let x = pos0.x as f64 + dx * COS120 - dy * SIN120;
    let y = pos0.y as f64 + dx * SIN120 + dy * COS120;
    Coord::new(x as usize, y as usize)
}

fn compute_mean(points: &Vec<Coord>) -> [f64; 2] {
    let n = points.len() as f64;
    let mut sum = [0.0, 0.0];
    for p in points {
        sum[0] += p.x as f64;
        sum[1] += p.y as f64;
    }
    [sum[0] / n, sum[1] / n]
}

fn compute_covariance(points: &Vec<Coord>) -> [[f64; 2]; 2] {
    let n = points.len() as f64;
    let mean = compute_mean(points);

    let mut cov = [[0.0, 0.0], [0.0, 0.0]];
    for p in points {
        let dx = p.x as f64 - mean[0];
        let dy = p.y as f64 - mean[1];
        cov[0][0] += dx * dx;
        cov[0][1] += dx * dy;
        cov[1][0] += dy * dx;
        cov[1][1] += dy * dy;
    }

    for i in 0..2 {
        for j in 0..2 {
            cov[i][j] /= n;
        }
    }

    cov
}

pub struct Output {
    // pub true_edges: Vec<(usize, usize)>,
    pub estimate_points: Vec<Coord>,
    // pub ok_points: Vec<Coord>,
    // pub ellipses: Vec<([f64; 2], [[f64; 2]; 2])>,
}
