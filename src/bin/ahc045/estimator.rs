use itertools::Itertools;
use rand_pcg::Pcg64Mcg;

use crate::{coord::Coord, input::Input};

pub struct Estimator {
    pub xy: Vec<Coord>,
    pub dist: Vec<Vec<usize>>,
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
        Self { xy, dist }
    }
    pub fn query(&self, input: &Input) {
        let mut rng = Pcg64Mcg::new(100);
        let nodes_sorted_by_error = (0..input.N)
            .sorted_by_key(|&i| input.rects[i].long_side())
            .rev()
            .take(input.Q)
            .collect_vec();

        for &first_node_idx in nodes_sorted_by_error.iter() {
            let mut query_nodes = vec![first_node_idx];
            get_query_nodes(&mut query_nodes, input, &mut rng);
            //     println!("? {} {}", query_nodes.len(), query_nodes.iter().join(" "));
        }
    }
}

fn get_query_nodes(query_nodes: &mut Vec<usize>, input: &Input, rng: &mut Pcg64Mcg) -> bool {
    if query_nodes.len() == input.L {
        return true;
    }
    if query_nodes.len() == 1 {
        for second_node_idx in 0..input.N {
            if query_nodes.contains(&second_node_idx) {
                continue;
            }
            query_nodes.push(second_node_idx);
            let found = get_query_nodes(query_nodes, input, rng);
            if found {
                return true;
            }
            query_nodes.pop();
        }
    } else {
    }
    false
}
