use proconio::input;

use crate::coord::Coord;

pub fn read_input() -> Input {
    input! {
        N: usize,
        M: usize,
        C_: [String; N]
    }

    let C: Vec<Vec<char>> = C_.iter().map(|s| s.chars().collect()).collect();

    let mut start = Coord::new(0, 0);
    for i in 0..N {
        for j in 0..N {
            if C[i][j] == 'A' {
                start = Coord::new(i, j);
            }
        }
    }

    Input { N, M, C, start }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub C: Vec<Vec<char>>,
    pub start: Coord,
}
