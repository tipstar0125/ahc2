use proconio::input_interactive;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{coord::Coord, input::Input, normal::Normal};

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub coord: Coord,
    pub velocity: Coord,
    pub weight: f64,
}

pub struct Estimator {
    pub turn: usize,
    pub velocity_pdf: Normal,
    pub particles: Vec<Particle>,
}

impl Estimator {
    pub fn new(input: &Input, particle_num: usize) -> Self {
        Self {
            turn: 0,
            velocity_pdf: Normal::new(0.0, input.eps),
            particles: vec![
                Particle {
                    coord: input.s,
                    velocity: Coord { x: 0, y: 0 },
                    weight: 1.0,
                };
                particle_num
            ],
        }
    }
    pub fn move_(&mut self, rng: &mut Pcg64Mcg) {
        for i in 0..self.particles.len() {
            let mut fx = self.velocity_pdf.sample(rng);
            let mut fy = self.velocity_pdf.sample(rng);
            if rng.gen_bool(0.5) {
                std::mem::swap(&mut fx, &mut fy);
            }
            let fx = fx.round() as i64;
            let fy = fy.round() as i64;
            self.particles[i].velocity.x += fx;
            self.particles[i].velocity.y += fy;
            self.particles[i].coord.x += self.particles[i].velocity.x;
            self.particles[i].coord.y += self.particles[i].velocity.y;
            self.particles[i].coord.x = self.particles[i]
                .coord
                .x
                .max(-1e5 as i64 + 1)
                .min(1e5 as i64 - 1);
            self.particles[i].coord.y = self.particles[i]
                .coord
                .y
                .max(-1e5 as i64 + 1)
                .min(1e5 as i64 - 1);
        }
    }
    pub fn measure(&mut self, input: &Input, d: i64, is_x_direction: bool) {
        for i in 0..self.particles.len() {
            let particle_d = if is_x_direction {
                1e5 as i64 - self.particles[i].coord.x
            } else {
                1e5 as i64 - self.particles[i].coord.y
            };

            let std = particle_d as f64 * input.delta;
            let measure_pdf = Normal::new(particle_d as f64, std);
            self.particles[i].weight *= measure_pdf.pdf(d as f64);
        }
    }
    pub fn resampling(&mut self, rng: &mut Pcg64Mcg) -> Vec<Particle> {
        let mut ws = vec![];
        let mut s = 0.0;
        for i in 0..self.particles.len() {
            s += self.particles[i].weight;
            ws.push(s);
        }
        if s < 1e-100 {
            ws = ws.iter().map(|x| x + 1e-100).collect();
            s += 1e-100;
        }
        let step = s / self.particles.len() as f64;
        let mut r = rng.gen_range(0.0..step);
        let mut pos = 0;
        let mut particles = vec![];
        while particles.len() < self.particles.len() {
            if r < ws[pos] {
                self.particles[pos].weight = 1.0;
                particles.push(self.particles[pos]);
                r += step;
            } else {
                pos += 1;
            }
        }
        self.particles = particles;
        self.particles.clone()
    }
    pub fn action(&mut self, input: &Input, rng: &mut Pcg64Mcg) -> Vec<Particle> {
        if self.turn % 3 == 0 {
            println!("A 0 0");
        } else if self.turn % 3 == 1 {
            println!("S 1 0");
            input_interactive! {
                d: i64,
            }
            self.measure(input, d, true);
        } else {
            println!("S 0 1");
            input_interactive! {
                d: i64,
            }
            self.measure(input, d, false);
        }

        input_interactive! {
            c: usize,
            h: usize,
            _q: [usize; h]
        }

        if c == 1 {
            for i in 0..self.particles.len() {
                self.particles[i].velocity.x = 0;
                self.particles[i].velocity.y = 0;
            }
        } else {
            self.move_(rng);
        }

        self.turn += 1;
        self.resampling(rng)
    }
}
