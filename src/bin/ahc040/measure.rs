use itertools::Itertools;
use proconio::input_interactive;
use rand::{seq::SliceRandom, Rng};
use rand_pcg::Pcg64Mcg;
use rustc_hash::FxHashMap;

pub fn measure(N: usize, T: usize, sigma: i64, wh2: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut rng = Pcg64Mcg::new(10);
    let mut t = 0;
    let mut measures = vec![];
    let mut measure_cnt = vec![vec![0; 2]; N];

    let center_idx = {
        let mut idx;
        let mut d = 0;
        'outer: loop {
            for dir in [1, -1].iter() {
                idx = N / 2 + (d * dir) as usize;
                let (w, h) = wh2[idx];
                if w < 7e4 as i64 && h < 7e4 as i64 {
                    break 'outer;
                }
            }
            d += 1;
        }
        idx
    };

    while t < T {
        let order = (0..N).collect_vec();
        // 前半部分を横に置く
        let order_width = order
            .iter()
            .take(center_idx)
            .map(|x| {
                let rotate = if t == 0 {
                    false
                } else {
                    rng.gen_bool(measure_cnt[*x][0] as f64 / t as f64) // 横・縦の測定回数を均等にする
                };
                let w = if rotate { wh2[*x].1 } else { wh2[*x].0 };
                (*x, rotate, w)
            })
            .collect_vec();

        // 横に置いたものの中で最も幅が大きいものを探す
        // これを基準にして縦に置く
        let (max_width_idx, max_width_rotate, max_width) = order_width
            .iter()
            .take(order_width.len() - 1) // 最後の要素に縦置きすると、幅測定に影響が出るため除外
            .max_by_key(|x| x.2)
            .unwrap();

        // 後半部分を縦に置く
        let order_height = order
            .iter()
            .skip(center_idx)
            .map(|x| {
                let rotate = if t == 0 {
                    false
                } else {
                    rng.gen_bool(measure_cnt[*x][0] as f64 / t as f64) // 横・縦の測定回数を均等にする
                };
                let w = if rotate { wh2[*x].1 } else { wh2[*x].0 };
                (*x, rotate, w)
            })
            .collect_vec();

        let (_, _, first_width) = order_height[0]; // 縦置きの先頭

        // 横置きの最大幅が、縦置きの先頭の幅より十分大きい場合に測定
        if max_width - first_width > sigma {
            t += 1;
            println!("{}", N);
            let mut now = -1;
            let mut measure_width = FxHashMap::default();
            for &(idx, rotate, _) in order_width.iter() {
                println!("{} {} U {}", idx, if rotate { 1 } else { 0 }, now);
                now = idx as i32;
                measure_width.insert(idx, rotate);
                measure_cnt[idx][rotate as usize] += 1;
            }

            let mut measure_height = FxHashMap::default();
            measure_height.insert(*max_width_idx, !*max_width_rotate);
            for &(idx, rotate, _) in order_height.iter() {
                println!(
                    "{} {} U {}",
                    idx,
                    if rotate { 1 } else { 0 },
                    *max_width_idx as i32 - 1 // 右端基準なので、置きたい箱の1つ前の箱の右端に合わせる
                );
                measure_height.insert(idx, !rotate);
                measure_cnt[idx][rotate as usize] += 1;
            }
            input_interactive! {
                w: i64, h: i64,
            }
            // 測定方向と結果が分かれば、横置き、縦置きを区別する必要はない
            measures.push((measure_width, w));
            measures.push((measure_height, h));
        }
    }

    let mut state = State::new(wh2, sigma, measures);
    state.solve();
    state.wh
}

struct State {
    wh: Vec<(i64, i64)>,
    sigma: i64,
    // 箱の長さがwhだったとき、measuresに従って置いたときの長さ
    lengths: Vec<i64>,
    // 測定の際のindexと回転の情報(0: 横, 1: 縦)と測定値
    measures: Vec<(FxHashMap<usize, bool>, i64)>,
    // 各測定値の差の二乗和
    score: i64,
}

impl State {
    fn new(wh2: Vec<(i64, i64)>, sigma: i64, measures: Vec<(FxHashMap<usize, bool>, i64)>) -> Self {
        let mut score = 0;
        let mut lengths = vec![];
        for (measure, measure_len) in measures.iter() {
            let mut len = 0;
            for (idx, rotate) in measure.iter() {
                let (w, h) = wh2[*idx];
                if *rotate {
                    len += h;
                } else {
                    len += w;
                }
            }
            let diff = (measure_len - len).abs();
            score += diff * diff;
            lengths.push(len);
        }
        eprintln!("Initial measure score : {}", score);

        Self {
            wh: wh2,
            sigma,
            lengths,
            measures,
            score,
        }
    }
    fn solve(&mut self) {
        let n = self.wh.len();
        let max_delta = self.sigma / 20;

        for sep in 1..=200 {
            let delta = (max_delta / sep).max(1);

            for idx in 0..n {
                for r in 0..=1 {
                    let rotate = if r == 0 { false } else { true };

                    let dir = {
                        let mut next_score = self.score;
                        for (length, measure) in self.lengths.iter().zip(self.measures.iter()) {
                            if measure.0.contains_key(&idx) && measure.0[&idx] == rotate {
                                let before_diff = (length - measure.1).abs();
                                let after_diff = (length + delta - measure.1).abs();
                                next_score += after_diff * after_diff - before_diff * before_diff;
                            }
                        }
                        if next_score < self.score {
                            1
                        } else {
                            -1
                        }
                    };

                    let mut l = 1;
                    let mut r = 100;
                    while r - l > 1 {
                        let m = (l + r) / 2;
                        let d = delta * m * dir;
                        let mut next_score = self.score;

                        for (length, measure) in self.lengths.iter().zip(self.measures.iter()) {
                            if measure.0.contains_key(&idx) && measure.0[&idx] == rotate {
                                let before_diff = (length - measure.1).abs();
                                let after_diff = (length + d - measure.1).abs();
                                next_score += after_diff * after_diff - before_diff * before_diff;
                            }
                        }

                        let diff_score = next_score - self.score;

                        if diff_score < 0 {
                            l = m;
                        } else {
                            r = m;
                        }
                    }

                    let mut modified = l;
                    modified /= 20;
                    if modified == 0 {
                        continue;
                    }
                    let modified_delta = delta * modified * dir;
                    let mut next_score = self.score;

                    for (length, measure) in self.lengths.iter_mut().zip(self.measures.iter()) {
                        if measure.0.contains_key(&idx) && measure.0[&idx] == rotate {
                            let before_diff = (*length - measure.1).abs();
                            let after_diff = (*length + modified_delta - measure.1).abs();
                            *length += modified_delta;
                            next_score += after_diff * after_diff - before_diff * before_diff;
                        }
                    }
                    if rotate {
                        self.wh[idx].1 += modified_delta;
                    } else {
                        self.wh[idx].0 += modified_delta;
                    }
                    self.score = next_score;
                }
            }
        }
        eprintln!("Modified score: {}", self.score);
    }
}
