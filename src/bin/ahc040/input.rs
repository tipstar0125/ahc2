use proconio::input_interactive;

use crate::hash::CalcHash;

const MIN: i64 = 1e4 as i64;
const MAX: i64 = 1e5 as i64;

pub fn read_input() -> Input {
    input_interactive! {
        N: usize, mut T: usize, sigma: i64,
        _wh2: [(i64, i64); N],
        // wh: [(i64, i64); N],
    }

    let mut wh2 = vec![];
    for (w, h) in _wh2 {
        wh2.push((w.max(MIN).min(MAX), h.max(MIN).min(MAX)));
    }

    eprintln!("N = {}", N);
    eprintln!("T = {}", T);
    eprintln!("sigma = {}", sigma);

    let mut wh2_list = vec![vec![]; N];
    for (i, (w, h)) in wh2.iter().enumerate() {
        wh2_list[i].push((*w, *h));
    }

    let mut n = 0;
    while T > 30 {
        println!("1");
        println!("{} 0 U -1", n % N);
        input_interactive! {
            w: i64, h: i64,
        }
        wh2_list[n % N].push((w.max(MIN).min(MAX), h.max(MIN).min(MAX)));
        n += 1;
        T -= 1;
    }
    let mut wh = vec![];
    for whs in wh2_list.iter() {
        let mut w_ave = 0;
        let mut h_ave = 0;
        for (w, h) in whs.iter() {
            w_ave += w;
            h_ave += h;
        }
        w_ave /= whs.len() as i64;
        h_ave /= whs.len() as i64;
        wh.push((w_ave, h_ave));
    }

    let mut area = 0.0;
    for (w, h) in wh.iter() {
        area += *w as f64 * *h as f64;
    }
    let width_limit = area.sqrt() as i64 + 1e5 as i64;

    Input {
        N,
        T,
        sigma,
        wh2: wh,
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
