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
        G[u].push((A[v], v));
        G[v].push((A[u], u));
    }

    let G = G
        .into_iter()
        .map(|mut v| {
            v.sort();
            v.into_iter().map(|(_, u)| u).collect()
        })
        .collect();

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
