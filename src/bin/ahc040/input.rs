use proconio::input_interactive;

use crate::hash::CalcHash;

const MIN: i32 = 1e4 as i32;
const MAX: i32 = 1e5 as i32;

pub fn read_input() -> Input {
    input_interactive! {
        N: usize, T: usize, sigma: i32,
        _wh2: [(i32, i32); N],
        // wh: [(i32, i32); N],
    }

    let mut wh2 = vec![];
    for (w, h) in _wh2 {
        wh2.push((w.max(MIN).min(MAX), h.max(MIN).min(MAX)));
    }

    eprintln!("N = {}", N);
    eprintln!("T = {}", T);
    eprintln!("sigma = {}", sigma);

    let mut area = 0.0;
    for (w, h) in wh2.iter() {
        area += *w as f64 * *h as f64;
    }
    let width_limit = area.sqrt() as i32 + 2e4 as i32;

    Input {
        N,
        T,
        sigma,
        wh2,
        calc_hash: CalcHash::new(N),
        width_limit,
    }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub T: usize,
    pub sigma: i32,
    pub wh2: Vec<(i32, i32)>,
    pub calc_hash: CalcHash,
    pub width_limit: i32,
}
