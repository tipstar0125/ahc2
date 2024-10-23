use proconio::{input, marker::Chars};

pub fn read_input() -> Input {
    input! {
        N: usize,
        M: usize,
        V: usize,
        S: [Chars; N],
        T: [Chars; N],
    }

    Input { N, M, V, S, T }
}

#[derive(Debug)]
pub struct Input {
    N: usize,
    M: usize,
    V: usize,
    S: Vec<Vec<char>>,
    T: Vec<Vec<char>>,
}
