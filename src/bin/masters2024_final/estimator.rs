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
    pub rng: Pcg64Mcg,
    pub turn: usize,
    pub velocity_x_pdf: Normal,
    pub velocity_y_pdf: Normal,
    pub particles: Vec<Particle>,
}

impl Estimator {
    pub fn new(input: &Input, particle_num: usize) -> Self {
        Self {
            rng: Pcg64Mcg::new(100),
            turn: 0,
            velocity_x_pdf: Normal::new(0.0, input.eps * 2.0),
            velocity_y_pdf: Normal::new(0.0, input.eps * 2.0),
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
    pub fn update_motion(&mut self) {
        for i in 0..self.particles.len() {
            let fx = self.velocity_x_pdf.sample(&mut self.rng).round() as i64;
            let fy = self.velocity_y_pdf.sample(&mut self.rng).round() as i64;
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
    pub fn update_measure(
        &mut self,
        input: &Input,
        d: i64,
        is_x_direction: bool,
        is_direction_plus: bool,
    ) {
        for i in 0..self.particles.len() {
            let mut particle_d = if is_x_direction {
                1e5 as i64 - self.particles[i].coord.x
            } else {
                1e5 as i64 - self.particles[i].coord.y
            };

            if !is_direction_plus {
                particle_d = 2e5 as i64 - particle_d;
            }

            let std = particle_d as f64 * input.delta;
            let measure_pdf = Normal::new(particle_d as f64, std);
            self.particles[i].weight *= measure_pdf.pdf(d as f64);
        }
    }
    pub fn resampling(&mut self) -> Vec<Particle> {
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
        let mut r = self.rng.gen_range(0.0..step);
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
    pub fn stop(&mut self) {
        for i in 0..self.particles.len() {
            self.particles[i].velocity.x = 0;
            self.particles[i].velocity.y = 0;
        }
    }
    pub fn get_estimated_position(&self) -> Coord {
        let len = self.particles.len() as i64;
        Coord {
            x: self.particles.iter().map(|p| p.coord.x).sum::<i64>() / len,
            y: self.particles.iter().map(|p| p.coord.y).sum::<i64>() / len,
        }
    }
}
