pub const DXY4: [Coord; 4] = [
    Coord { x: 0, y: 1 },  // Up
    Coord { x: 1, y: 0 },  // Right
    Coord { x: 0, y: !0 }, // Down
    Coord { x: !0, y: 0 }, // Left
];

pub const TWO_BY_TWO: [Coord; 4] = [
    Coord { x: 0, y: 0 },
    Coord { x: !0, y: 0 },
    Coord { x: 0, y: !0 },
    Coord { x: !0, y: !0 },
];

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
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.x, self.y)?;
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

pub fn calc_manhattan_dist(a: Coord, b: Coord) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

pub fn calc_dist2(a: Coord, b: Coord) -> usize {
    a.x.wrapping_sub(b.x).wrapping_pow(2) + a.y.wrapping_sub(b.y).wrapping_pow(2)
}
