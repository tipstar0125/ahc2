use std::collections::BTreeMap;

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

    let xy_center = rects.iter().map(|rect| rect.center()).collect::<Vec<_>>();
    let mut x_positions = BTreeMap::default();
    let mut y_positions = BTreeMap::default();
    for i in 0..N {
        x_positions.entry(xy_center[i].x).or_insert(vec![]).push(i);
        y_positions.entry(xy_center[i].y).or_insert(vec![]).push(i);
    }

    Input {
        width: 10000,
        height: 10000,
        N,
        M,
        Q,
        L,
        W,
        G,
        rects,
        xy,
        x_positions,
        y_positions,
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub width: usize,
    pub height: usize,
    pub N: usize,
    pub M: usize,
    pub Q: usize,
    pub L: usize,
    pub W: usize,
    pub G: Vec<usize>,
    pub rects: Vec<Rect>,
    pub xy: Vec<Coord>,
    pub x_positions: BTreeMap<usize, Vec<usize>>,
    pub y_positions: BTreeMap<usize, Vec<usize>>,
}
