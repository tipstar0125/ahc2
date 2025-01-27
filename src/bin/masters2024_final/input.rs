use proconio::input_interactive;

use crate::coord::Coord;

pub fn read_input() -> Input {
    input_interactive! {
        N: usize,
        M: usize,
        eps: f64,
        delta: f64,
        _s: (i64, i64),
        _ps: [(i64, i64); N],
        _walls: [(i64, i64, i64, i64); M],
    }

    Input {
        N,
        M,
        eps,
        delta,
        s: Coord::new(_s.0, _s.1),
        ps: _ps.iter().map(|&(x, y)| Coord::new(x, y)).collect(),
        walls: _walls
            .iter()
            .map(|&(x1, y1, x2, y2)| (Coord::new(x1, y1), Coord::new(x2, y2)))
            .collect(),
        width: 1e5 as i64,
        height: 1e5 as i64,
        max_turn: 5000,
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub eps: f64,
    pub delta: f64,
    pub s: Coord,
    pub ps: Vec<Coord>,
    pub walls: Vec<(Coord, Coord)>,
    pub width: i64,
    pub height: i64,
    pub max_turn: usize,
}
