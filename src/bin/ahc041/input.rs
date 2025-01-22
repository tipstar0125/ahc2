use std::vec;

use proconio::input;

pub fn read_input() -> Input {
    input! {
        N: usize,
        M: usize,
        H: usize,
        A: [i64; N],
        UV: [(usize, usize); M],
        _XY: [(usize, usize); N],
    }

    let mut G = vec![vec![]; N];
    for (u, v) in UV {
        G[u].push(v);
        G[v].push(u);
    }

    Input { N, M, H, A, G }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub H: usize,
    pub A: Vec<i64>,
    pub G: Vec<Vec<usize>>,
}
