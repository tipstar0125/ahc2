pub const NEG: usize = usize::MAX;

const SIN120: f64 = 0.8660254037844386;
const COS120: f64 = -0.5;

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
    pub fn rotate_120deg(self, other: Coord) -> Option<Coord> {
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        let x = self.x as f64 + dx * COS120 - dy * SIN120;
        let y = self.y as f64 + dx * SIN120 + dy * COS120;
        if x < 0.0 || x > 10000.0 || y < 0.0 || y > 10000.0 {
            None
        } else {
            Some(Coord::new(x as usize, y as usize))
        }
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
