#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub y: usize,
    pub x: usize,
}

impl Coord {
    pub fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y: {}, x: {}", self.y, self.x)?;
        Ok(())
    }
}

impl std::ops::Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}