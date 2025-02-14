use itertools::Itertools;
use proconio::input_interactive;

use crate::{
    coord::Coord,
    estimator::{Estimator, Particle},
    input::Input,
};

pub struct State {
    pub turn: usize,
    pub coord: Coord,
    pub destination_idx: usize,
    pub reached_destination: Vec<bool>,
    pub order: Vec<usize>,
    pub estimator: Estimator,
}

impl State {
    pub fn new(input: &Input, estimator: Estimator) -> Self {
        let order = tsp(&input.s, &input.ps);
        Self {
            turn: 0,
            coord: input.s.clone(),
            destination_idx: order[0],
            reached_destination: vec![false; input.ps.len()],
            order,
            estimator,
        }
    }
    pub fn accelerate(&self) {
        println!("A 0 0");
    }
    pub fn measure(&mut self, input: &Input) {
        if self.turn % 3 == 1 {
            let is_direction_plus = self.coord.x >= 0;
            println!("S {} 0", if is_direction_plus { 1 } else { -1 });
            input_interactive! {
                d: i64,
            }
            self.estimator
                .update_measure(input, d, true, is_direction_plus);
        } else if self.turn % 3 == 2 {
            let is_direction_plus = self.coord.y >= 0;
            println!("S 0 {}", if is_direction_plus { 1 } else { -1 });
            input_interactive! {
                d: i64,
            }
            self.estimator
                .update_measure(input, d, false, is_direction_plus);
        } else {
            unreachable!();
        }
    }
    pub fn action(&mut self, input: &Input) {
        if self.turn % 3 == 0 {
            self.accelerate();
        } else {
            self.measure(input);
        }

        input_interactive! {
            c: usize,
            h: usize,
            q: [usize; h]
        }

        if c == 1 {
            self.estimator.stop();
        } else {
            self.estimator.update_motion();
        }

        for &i in &q {
            self.reached_destination[i] = true;
        }
        self.next_destination();

        self.estimator.resampling();
        self.coord = self.estimator.get_estimated_position();
        self.turn += 1;
    }
    pub fn next_destination(&mut self) {
        for &idx in &self.order {
            if !self.reached_destination[idx] {
                self.destination_idx = idx;
                return;
            }
        }
    }
    pub fn get_coord(&self) -> Coord {
        self.coord
    }
    pub fn get_particles(&self) -> Vec<Particle> {
        self.estimator.particles.clone()
    }
    pub fn get_reached_destination(&self) -> Vec<bool> {
        self.reached_destination.clone()
    }
}

fn tsp(start: &Coord, ps: &Vec<Coord>) -> Vec<usize> {
    let mut best_dist = 1 << 60;
    let mut best_order = vec![];
    for order in (0..ps.len()).permutations(ps.len()) {
        let mut dist = 0;

        let mut pos = start.clone();
        for i in 0..ps.len() {
            let next = ps[order[i]].clone();
            let dx = next.x - pos.x;
            let dy = next.y - pos.y;
            dist += dx * dx + dy * dy;
            pos = next;
        }
        if dist < best_dist {
            best_dist = dist;
            best_order = order;
        }
    }
    best_order
}

// a^2+b^2<=c^2になるようにa, bをスケーリング
fn scale_to_fit(a: i64, b: i64, c: i64) -> (i64, i64) {
    if a * a + b * b <= c * c {
        return (a, b);
    }

    let sq = ((a * a + b * b) as f64).sqrt();
    let sa = (a as f64 * c as f64 / sq) as i64;
    let sb = (b as f64 * c as f64 / sq) as i64;

    (sa, sb)
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use rand_pcg::Pcg64Mcg;

    use super::*;

    #[test]
    fn test_scale_to_fit() {
        let mut rng = Pcg64Mcg::new(100);
        for _ in 0..1e7 as usize {
            let a = rng.gen_range(-10000..=10000);
            let b = rng.gen_range(-10000..=10000);
            let c = 500;
            let (sa, sb) = scale_to_fit(a, b, c);
            assert!(sa * sa + sb * sb <= c * c);
        }
    }
}
