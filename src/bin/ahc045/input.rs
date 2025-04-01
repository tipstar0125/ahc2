use proconio::input_interactive;

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

    Input {
        N,
        M,
        Q,
        L,
        W,
        G,
        range,
    }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub Q: usize,
    pub L: usize,
    pub W: usize,
    pub G: Vec<usize>,
    pub range: Vec<(usize, usize, usize, usize)>,
}
