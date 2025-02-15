use proconio::input;

use crate::coord::Coord;

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

    for (ih, jh, iw, jw) in pos {
        home.push(Coord::new(ih, jh));
        workspace.push(Coord::new(iw, jw));
    }

    Input {
        N,
        M,
        K,
        T,
        home,
        workspace,
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
}
