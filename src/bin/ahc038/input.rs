use proconio::{input, marker::Chars};

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

    Input { N, M, V, S, T }
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
    Input { N, M, V, S, T }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub V: usize,
    pub S: Vec<Vec<char>>,
    pub T: Vec<Vec<char>>,
}
