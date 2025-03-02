pub const NEG: usize = usize::MAX;

pub const DIJ4: [Coord; 4] = [
    Coord { i: 0, j: 1 },   // Right
    Coord { i: 1, j: 0 },   // Down
    Coord { i: 0, j: NEG }, // Left
    Coord { i: NEG, j: 0 }, // Up
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    pub i: usize,
    pub j: usize,
}

impl Coord {
    pub fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }
    pub fn in_map(self, size: usize) -> bool {
        self.i < size && self.j < size
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "i: {}, j: {}", self.i, self.j)?;
        Ok(())
    }
}

impl std::ops::Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            i: self.i.wrapping_add(rhs.i),
            j: self.j.wrapping_add(rhs.j),
        }
    }
}

impl std::ops::Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, rhs: Coord) -> Self::Output {
        Coord {
            i: self.i.wrapping_sub(rhs.i),
            j: self.j.wrapping_sub(rhs.j),
        }
    }
}

impl std::ops::Mul<Coord> for Coord {
    type Output = Coord;
    fn mul(self, rhs: Coord) -> Self::Output {
        Coord {
            i: self.i.wrapping_mul(rhs.i),
            j: self.j.wrapping_mul(rhs.j),
        }
    }
}

pub fn calc_manhattan_dist(a: Coord, b: Coord) -> usize {
    a.i.abs_diff(b.i) + a.j.abs_diff(b.j)
}
