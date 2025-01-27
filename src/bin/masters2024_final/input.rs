use proconio::input_interactive;

use crate::coord::Coord;

pub fn read_input() -> Input {
    input_interactive! {
        N: usize,
        M: usize,
        eps: f64,
        delta: f64,
        s_: (i64, i64),
        ps_: [(i64, i64); N],
        walls_: [(i64, i64, i64, i64); M],
    }

    Input {
        N,
        M,
        eps,
        delta,
        s: Coord { x: s_.0, y: s_.1 },
        ps: ps_.iter().map(|&(x, y)| Coord { x, y }).collect(),
        walls: walls_
            .iter()
            .map(|&(x1, y1, x2, y2)| (Coord { x: x1, y: y1 }, Coord { x: x2, y: y2 }))
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
