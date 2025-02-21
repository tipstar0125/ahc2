use proconio::input;

use crate::{coord::Coord, hash::CalcHash};

pub fn read_input() -> Input {
    input! {
        N: usize,
        M: usize,
        K: usize,
        T: usize,
        pos: [(usize, usize, usize, usize,); M],
    }

    let mut home = vec![];
    let mut workspace = vec![];
    let mut home_workspace_field = vec![vec![vec![]; N]; N];

    for (idx, &(ih, jh, iw, jw)) in pos.iter().enumerate() {
        home.push(Coord::new(ih, jh));
        workspace.push(Coord::new(iw, jw));
        home_workspace_field[ih][jh].push(idx);
        home_workspace_field[iw][jw].push(idx + M);
    }

    let calc_hash = CalcHash::new(N);

    Input {
        N,
        M,
        K,
        T,
        home,
        workspace,
        home_workspace_field,
        calc_hash,
    }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub K: usize,
    pub T: usize,
    pub home: Vec<Coord>,
    pub workspace: Vec<Coord>,
    pub home_workspace_field: Vec<Vec<Vec<usize>>>,
    pub calc_hash: CalcHash,
}
