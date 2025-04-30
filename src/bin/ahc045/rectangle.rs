use std::{cmp::max, collections::HashSet};

use itertools::Itertools;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{coord::Coord, dsu::UnionFind};

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
}
