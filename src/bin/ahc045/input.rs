use crate::rectangle::Rect;
use proconio::input_interactive;

use crate::coord::Coord;

pub fn read_input(is_local: bool) -> Input {
    input_interactive! {
        N: usize,
        M: usize,
        Q: usize,
        L: usize,
        W: usize,
        G: [usize; M],
        range: [(usize, usize, usize, usize); N],
    }

    eprintln!("M = {}", M);
    eprintln!("L = {}", L);
    eprintln!("W = {}", W);

    let xy = if is_local {
        input_interactive! {
            xy: [(usize, usize); N],
        }
        xy.into_iter().map(|(x, y)| Coord::new(x, y)).collect()
    } else {
        vec![Coord::new(0, 0); N]
    };

    let rects: Vec<Rect> = range
        .iter()
        .map(|(lx, rx, ly, ry)| Rect {
            x_min: *lx,
            x_max: *rx,
            y_min: *ly,
            y_max: *ry,
        })
        .collect();

    Input {
        size: 10000,
        N,
        M,
        Q,
        L,
        W,
        G,
        rects,
        xy,
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub size: usize,
    pub N: usize,
    pub M: usize,
    pub Q: usize,
    pub L: usize,
    pub W: usize,
    pub G: Vec<usize>,
    pub rects: Vec<Rect>,
    pub xy: Vec<Coord>,
}
