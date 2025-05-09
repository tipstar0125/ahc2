use std::cmp::max;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::coord::Coord;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x_min: usize,
    pub x_max: usize,
    pub y_min: usize,
    pub y_max: usize,
}

impl Rect {
    pub fn center(&self) -> Coord {
        Coord {
            x: (self.x_min + self.x_max) / 2,
            y: (self.y_min + self.y_max) / 2,
        }
    }

    pub fn long_side(&self) -> usize {
        max(self.x_max - self.x_min, self.y_max - self.y_min)
    }
    pub fn random_coord(&self, rng: &mut Pcg64Mcg) -> Coord {
        let x = rng.gen_range(self.x_min..=self.x_max);
        let y = rng.gen_range(self.y_min..=self.y_max);
        Coord { x, y }
    }
    pub fn min_dist(&self, other: &Rect) -> usize {
        let dx = if self.x_max < other.x_min {
            other.x_min - self.x_max
        } else if other.x_max < self.x_min {
            self.x_min - other.x_max
        } else {
            0
        };

        let dy = if self.y_max < other.y_min {
            other.y_min - self.y_max
        } else if other.y_max < self.y_min {
            self.y_min - other.y_max
        } else {
            0
        };

        let square_dist = (dx * dx + dy * dy) as f64;
        let dist = square_dist.sqrt();
        dist as usize
    }

    pub fn max_dist(&self, other: &Rect) -> usize {
        let dx = self
            .x_min
            .abs_diff(other.x_min)
            .max(self.x_min.abs_diff(other.x_max))
            .max(self.x_max.abs_diff(other.x_min))
            .max(self.x_max.abs_diff(other.x_max));

        let dy = self
            .y_min
            .abs_diff(other.y_min)
            .max(self.y_min.abs_diff(other.y_max))
            .max(self.y_max.abs_diff(other.y_min))
            .max(self.y_max.abs_diff(other.y_max));

        let square_dist = (dx * dx + dy * dy) as f64;
        let dist = square_dist.sqrt();
        dist as usize
    }
}
