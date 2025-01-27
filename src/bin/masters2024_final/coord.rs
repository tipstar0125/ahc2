#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
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
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Mul<i64> for Coord {
    type Output = Coord;
    fn mul(self, rhs: i64) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
