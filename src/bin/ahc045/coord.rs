pub const NEG: usize = usize::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn in_map(self, size: usize) -> bool {
        self.x < size && self.y < size
    }
    pub fn manhattan_dist(self, other: Coord) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
    pub fn euclidean_dist2(self, other: Coord) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        dx * dx + dy * dy
    }
    pub fn euclidean_dist(self, other: Coord) -> usize {
        let dist2 = self.euclidean_dist2(other) as f64;
        dist2.sqrt() as usize
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)?;
        Ok(())
    }
}

impl std::ops::Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
        }
    }
}

impl std::ops::Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x.wrapping_sub(rhs.x),
            y: self.y.wrapping_sub(rhs.y),
        }
    }
}

impl std::ops::Mul<Coord> for Coord {
    type Output = Coord;
    fn mul(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x.wrapping_mul(rhs.x),
            y: self.y.wrapping_mul(rhs.y),
        }
    }
}
