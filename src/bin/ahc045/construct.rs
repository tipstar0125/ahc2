use itertools::Itertools;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{
    common::get_time,
    coord::{calc_dist2, Coord},
    input::Input,
};

pub struct Forest {}

impl Forest {
    pub fn solve(input: &Input, xy: &Vec<Coord>, TLE: f64) {
        // グループに含まれるべき点の個数で降順ソートし、大きいものから順にグループを構成する
        // グループを構成する最初の頂点は、使用していない頂点の中からランダムに選択する
        // 次の頂点は、使用していない頂点の中から、最も近い頂点を選択し、グループに含め木構造を構築する
        // これらの構築を時間制限まえ繰り返し実行し、木の辺の長さの総和が最小のもを出力する

        let mut dist = vec![vec![0; input.N]; input.N];
        for i in 0..input.N {
            for j in 0..input.N {
                dist[i][j] = calc_dist2(xy[i], xy[j]);
            }
        }

        let mut G_with_idx = input
            .G
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, g)| (g, i))
            .collect::<Vec<(usize, usize)>>();
        G_with_idx.sort();
        G_with_idx.reverse();

        let mut dist_idx = vec![vec![]; input.N];

        for i in 0..input.N {
            for j in 0..input.N {
                if i == j {
                    continue;
                }
                dist_idx[i].push((dist[i][j], j));
            }
            dist_idx[i].sort();
        }

        let mut rng = Pcg64Mcg::new(10);
        let mut best_score = usize::MAX;
        let mut best_ans = vec![];
        let mut iter = 0;

        while get_time() < TLE {
            iter += 1;
            let mut used = vec![false; input.N];
            let mut proceed_idx = vec![0; input.N];
            let mut ans = vec![vec![]; input.M];
            let mut score = 0;

            for &(num, g_idx) in G_with_idx.iter() {
                let mut node_idx = rng.gen_range(0..input.N);
                while used[node_idx] {
                    node_idx = rng.gen_range(0..input.N);
                }
                used[node_idx] = true;
                let mut nodes = vec![node_idx];
                ans[g_idx].push((node_idx, !0));

                while nodes.len() < num {
                    let mut target_node = !0;
                    let mut next_node = !0;
                    let mut min_dist = usize::MAX;
                    for &node_idx in nodes.iter() {
                        loop {
                            let (dist, next_idx) = dist_idx[node_idx][proceed_idx[node_idx]];
                            if used[next_idx] {
                                proceed_idx[node_idx] += 1;
                                continue;
                            }
                            if dist < min_dist {
                                min_dist = dist;
                                next_node = next_idx;
                                target_node = node_idx;
                            }
                            break;
                        }
                    }
                    assert!(target_node != !0);
                    assert!(next_node != !0);
                    assert!(min_dist != usize::MAX);
                    used[next_node] = true;
                    nodes.push(next_node);
                    ans[g_idx].push((target_node, next_node));
                    score += min_dist;
                }
            }
            if score < best_score {
                best_score = score;
                best_ans = ans;
            }
        }
        println!("!");
        for row in best_ans {
            assert!(row.len() > 0);
            if row.len() == 1 {
                println!("{}", row[0].0);
            } else {
                let mut nodes = vec![];
                for (a, b) in row.iter().skip(1) {
                    nodes.push(a);
                    nodes.push(b);
                }
                nodes.sort();
                nodes.dedup();
                println!("{}", nodes.iter().join(" "));
                for (a, b) in row.iter().skip(1) {
                    println!("{} {}", a, b);
                }
            }
        }
        eprintln!("iter = {}", iter);
        eprintln!("Score = {}", best_score);
    }
}
