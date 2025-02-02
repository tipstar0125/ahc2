use proconio::{input, marker::Chars};

pub fn read_input() -> Input {
    input! {
        N: usize,
        C: [Chars; N],

    }
    Input { N, C }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub C: Vec<Vec<char>>,
}
