use proconio::input;

pub fn read_input() -> Input {
    input! {
        N: usize,
        M: usize,
        H: usize,
        UV: [(usize, usize); M],
        XY: [(usize, usize); N],
    }

    Input { N, M, H, UV, XY }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub H: usize,
    pub UV: Vec<(usize, usize)>,
    pub XY: Vec<(usize, usize)>,
}
