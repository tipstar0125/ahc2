use itertools::Itertools;
use proconio::input;

use crate::coord::Coord;

pub fn read_input() -> Input {
    input! {
        N: usize,
        _saba: [(usize, usize); N],
        _iwashi: [(usize, usize); N],
    }

    Input {
        N,
        size: 100000,
        saba: _saba.iter().map(|v| Coord::new(v.0, v.1)).collect_vec(),
        iwashi: _iwashi.iter().map(|v| Coord::new(v.0, v.1)).collect_vec(),
    }
}

pub fn parse_input(f: &str) -> Input {
    let f = proconio::source::once::OnceSource::from(f);
    input! {
        from f,
        N: usize,
        _saba: [(usize, usize); N],
        _iwashi: [(usize, usize); N],
    }
    Input {
        N,
        size: 100000,
        saba: _saba.iter().map(|v| Coord::new(v.0, v.1)).collect_vec(),
        iwashi: _iwashi.iter().map(|v| Coord::new(v.0, v.1)).collect_vec(),
    }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub size: usize,
    pub saba: Vec<Coord>,
    pub iwashi: Vec<Coord>,
}
