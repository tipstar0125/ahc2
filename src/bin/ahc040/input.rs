use proconio::input_interactive;

const MIN: i32 = 1e4 as i32;
const MAX: i32 = 1e5 as i32;

pub fn read_input() -> Input {
    input_interactive! {
        N: usize, T: usize, sigma: i32,
        _wh2: [(i32, i32); N],
    }

    let mut wh2 = vec![];
    for (w, h) in _wh2 {
        wh2.push((w.max(MIN).min(MAX), h.max(MIN).min(MAX)));
    }

    Input { N, T, sigma, wh2 }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub T: usize,
    pub sigma: i32,
    pub wh2: Vec<(i32, i32)>,
}
