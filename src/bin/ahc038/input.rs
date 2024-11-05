use proconio::{input, marker::Chars};

use crate::{arm::Arm, hash::CalcHash};

const GRAB_SCORE: usize = 1;
const RELEASE_SCORE: usize = 2;

pub fn read_input() -> Input {
    input! {
        N: usize,
        _M: usize,
        V: usize,
        S: [Chars; N],
        T: [Chars; N],
    }

    let mut M = 0;
    for i in 0..N {
        for j in 0..N {
            if S[i][j] == '1' && T[i][j] == '0' {
                M += 1;
            }
        }
    }
    eprintln!("input: N = {}, M = {}, V = {}", N, M, V);

    Input {
        N,
        M,
        V,
        S,
        T,
        arm: Arm::new(N, V),
        calc_hash: CalcHash::new(N, V),
        grab_score: GRAB_SCORE,
        release_score: RELEASE_SCORE,
        necessary_score: M * (GRAB_SCORE + RELEASE_SCORE),
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        N: usize, _M: usize, V: usize,
        S: [Chars; N],
        T: [Chars; N],
    }
    let mut M = 0;
    for i in 0..N {
        for j in 0..N {
            if S[i][j] == '1' && T[i][j] == '0' {
                M += 1;
            }
        }
    }
    Input {
        N,
        M,
        V,
        S,
        T,
        arm: Arm::new(N, V),
        calc_hash: CalcHash::new(N, V),
        grab_score: GRAB_SCORE,
        release_score: RELEASE_SCORE,
        necessary_score: M * (GRAB_SCORE + RELEASE_SCORE),
    }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub V: usize,
    pub S: Vec<Vec<char>>,
    pub T: Vec<Vec<char>>,
    pub arm: Arm,
    pub calc_hash: CalcHash,
    pub grab_score: usize,
    pub release_score: usize,
    pub necessary_score: usize,
}
