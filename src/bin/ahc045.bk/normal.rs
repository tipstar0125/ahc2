use std::{collections::VecDeque, f64::consts::PI};

use rand::Rng;
use rand_pcg::Pcg64Mcg;

#[derive(Debug)]
pub struct Normal {
    mu: f64,
    std: f64,
    queue: VecDeque<f64>,
}

impl Normal {
    pub fn new(mu: f64, std: f64) -> Self {
        Self {
            mu,
            std,
            queue: VecDeque::new(),
        }
    }
    pub fn sample(&mut self, rng: &mut Pcg64Mcg) -> f64 {
        if self.queue.is_empty() {
            let (x, y) = box_muller(rng, self.mu, self.std);
            self.queue.push_back(x);
            self.queue.push_back(y);
        }
        self.queue.pop_front().unwrap()
    }
    pub fn pdf(&self, x: f64) -> f64 {
        normal_pdf(x, self.mu, self.std)
    }
}

fn standard_normal_pair(rng: &mut Pcg64Mcg) -> (f64, f64) {
    let u1 = rng.gen::<f64>();
    let u2 = rng.gen::<f64>();
    let r = (-2.0 * u1.ln()).sqrt();
    let theta = 2.0 * PI * u2;
    (r * theta.cos(), r * theta.sin())
}

fn box_muller(rng: &mut Pcg64Mcg, mu: f64, std: f64) -> (f64, f64) {
    let u1 = rng.gen::<f64>();
    let u2 = rng.gen::<f64>();

    (
        mu + (-2.0 * u1.ln() * std.powf(2.0)).sqrt() * (2.0 * PI * u2).cos(),
        mu + (-2.0 * u1.ln() * std.powf(2.0)).sqrt() * (2.0 * PI * u2).sin(),
    )
}

pub fn sample_2d_normal(mu: [f64; 2], sigma: [[f64; 2]; 2], rng: &mut Pcg64Mcg) -> (f64, f64) {
    // Cholesky分解（2x2用）
    let a = sigma[0][0];
    let b = sigma[0][1];
    let c = sigma[1][1];

    let l11 = a.sqrt();
    let l21 = b / l11;
    let l22 = (c - l21 * l21).sqrt();

    // 標準正規分布からサンプル
    let (z1, z2) = standard_normal_pair(rng);

    // 線形変換
    let x = mu[0] + l11 * z1;
    let y = mu[1] + l21 * z1 + l22 * z2;
    (x, y)
}

fn normal_pdf(x: f64, mu: f64, std: f64) -> f64 {
    let v = (x - mu) / std;
    // 正確には以下だが、尤度計算において定数は不要
    // (-0.5 * v * v).exp() / ((2.0 * PI).sqrt() * std)
    (-0.5 * v * v).exp() / std
}
