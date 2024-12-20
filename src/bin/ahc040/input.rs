use proconio::input_interactive;

use crate::hash::CalcHash;
use crate::measure::measure;

const MIN: i64 = 1e4 as i64;
const MAX: i64 = 1e5 as i64;

pub fn read_input() -> Input {
    input_interactive! {
        N: usize, mut T: usize, sigma: i64,
        _wh2: [(i64, i64); N],
    }

    #[cfg(feature = "local")]
    input_interactive! {
        _wh: [(i64, i64); N],
    }

    let mut wh2 = vec![];
    for (w, h) in _wh2 {
        wh2.push((w.max(MIN).min(MAX), h.max(MIN).min(MAX)));
    }

    eprintln!("N = {}", N);
    eprintln!("T = {}", T);
    eprintln!("sigma = {}", sigma);

    let measure_num = (T as f64 * 0.8) as usize;
    T -= measure_num;
    let modified_wh = measure(N, measure_num, sigma, wh2.clone());

    let mut clamped_wh = vec![];
    for (w, h) in modified_wh {
        clamped_wh.push((w.max(MIN).min(MAX), h.max(MIN).min(MAX)));
    }

    #[cfg(feature = "local")]
    {
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::Write;
        let mut file = File::create("box.csv").unwrap();
        writeln!(file, "{}", "before,after").unwrap();

        let mut before_square_sum = 0;
        let mut after_square_sum = 0;
        for i in 0..N {
            let (w0, h0) = _wh[i];
            let (w1, h1) = wh2[i];
            before_square_sum += (w1 - w0) * (w1 - w0) + (h1 - h0) * (h1 - h0);
            let (w2, h2) = clamped_wh[i];
            after_square_sum += (w2 - w0) * (w2 - w0) + (h2 - h0) * (h2 - h0);
            writeln!(file, "{},{}", w1 - w0, w2 - w0).unwrap();
            writeln!(file, "{},{}", h1 - h0, h2 - h0).unwrap();
        }
        eprintln!("before_square_sum = {}", before_square_sum);
        eprintln!("after_square_sum = {}", after_square_sum);
    }

    let mut area = 0.0;
    for (w, h) in clamped_wh.iter() {
        area += *w as f64 * *h as f64;
    }
    let width_limit = area.sqrt() as i64 + 1e5 as i64;

    Input {
        N,
        T,
        sigma,
        wh2: clamped_wh,
        calc_hash: CalcHash::new(width_limit),
        width_limit,
    }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub T: usize,
    pub sigma: i64,
    pub wh2: Vec<(i64, i64)>,
    pub calc_hash: CalcHash,
    pub width_limit: i64,
}
