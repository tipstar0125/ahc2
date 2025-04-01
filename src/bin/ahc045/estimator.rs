use crate::{coord::Coord, input::Input};

pub struct Estimator {
    pub xy: Vec<Coord>,
}

impl Estimator {
    pub fn new(input: &Input) -> Self {
        // 座標範囲の中心を点群の座標と仮定
        let xy = input
            .range
            .iter()
            .map(|(lx, rx, ly, ry)| Coord::new((lx + rx) / 2, (ly + ry) / 2))
            .collect::<Vec<Coord>>();
        Self { xy }
    }
}
