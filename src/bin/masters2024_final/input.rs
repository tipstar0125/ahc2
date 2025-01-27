use proconio::input_interactive;

pub fn read_input() -> Input {
    input_interactive! {
        N: usize,
        M: usize,
        eps: f64,
        delta: f64,
        s: (i64, i64),
        ps: [(i64, i64); N],
        walls: [(i64, i64, i64, i64); M],
    }

    Input {
        N,
        M,
        eps,
        delta,
        s,
        ps,
        walls,
    }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub eps: f64,
    pub delta: f64,
    pub s: (i64, i64),
    pub ps: Vec<(i64, i64)>,
    pub walls: Vec<(i64, i64, i64, i64)>,
}
