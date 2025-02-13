use proconio::{input, marker::Chars};

use crate::hash::CalcHash;

pub fn read_input() -> Input {
    input! {
        N: usize,
        C: [Chars; N],

    }

    Input {
        N,
        C,
        calc_hash: CalcHash::new(N),
    }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub C: Vec<Vec<char>>,
    pub calc_hash: CalcHash,
}
