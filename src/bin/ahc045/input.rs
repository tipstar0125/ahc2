use proconio::input_interactive;

use crate::coord::Coord;

pub fn read_input() -> Input {
    input_interactive! {
        N: usize, // 点の個数(N=800固定)
        M: usize, // グループ個数(M=1-400可変)
        Q: usize, // クエリ個数(Q=400固定)
        L: usize, // クエリの際、指定できる点の最大個数(L=3-15可変)
        W: usize, // 点の座標が含まれる長方形の幅や高さとして有り得る最大値(W=500-2500可変)
        G: [usize; M], // 各グループに含まれるべき点の個数、総和はN
        range: [(usize, usize, usize, usize); N], // 各点の座標が含まれる長方形の座標情報(lx, rx, ly, ry)、点の座標はxyともに0-10000の範囲
    }

    eprintln!("M = {}", M);
    eprintln!("L = {}", L);
    eprintln!("W = {}", W);

    let is_local = std::env::var("ATCODER").and(Ok(false)).unwrap_or(true);
    let xy = if is_local {
        input_interactive! {
            xy: [(usize, usize); N],
        }
        xy.into_iter().map(|(x, y)| Coord::new(x, y)).collect()
    } else {
        vec![Coord::new(0, 0); N]
    };

    Input {
        width: 10000,
        height: 10000,
        N,
        M,
        Q,
        L,
        W,
        G,
        range,
        xy,
    }
}

#[derive(Debug)]
pub struct Input {
    pub width: usize,
    pub height: usize,
    pub N: usize,
    pub M: usize,
    pub Q: usize,
    pub L: usize,
    pub W: usize,
    pub G: Vec<usize>,
    pub range: Vec<(usize, usize, usize, usize)>,
    pub xy: Vec<Coord>,
}
