use proconio::input;

use crate::coord::Coord;

pub fn read_input() -> Input {
    input! {
        N : usize,
        M : usize,
        ij : [(usize, usize); M],
    }

    let destinations = ij
        .iter()
        .map(|(i, j)| Coord::new(*i, *j))
        .collect::<Vec<_>>();
    let start = destinations[0];
    let destinations = destinations[1..].to_vec();

    Input {
        N,
        M,
        start,
        destinations,
    }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub start: Coord,
    pub destinations: Vec<Coord>,
}
