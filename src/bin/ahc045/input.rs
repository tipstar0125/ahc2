use proconio::input_interactive;

use crate::coord::{calc_dist2, Coord};

pub fn read_input() -> Input {
    input_interactive! {
        N: usize,
        M: usize,
        Q: usize,
        L: usize,
        W: usize,
        G: [usize; M],
        range: [(usize, usize, usize, usize,); N],
    }

    eprintln!("M = {}", M);
    eprintln!("L = {}", L);
    eprintln!("W = {}", W);

    let mut xy = range
        .iter()
        .map(|(lx, rx, ly, ry)| Coord::new((lx + rx) / 2, (ly + ry) / 2))
        .collect::<Vec<Coord>>();
    let mut xy2 = xy.clone();
    #[cfg(feature = "local")]
    {
        input_interactive! {
            xy_: [(usize, usize); N],
        }
        xy2 = xy_
            .iter()
            .map(|(x, y)| Coord::new(*x, *y))
            .collect::<Vec<Coord>>();
        xy = xy2.clone();
    }

    let mut dist = vec![vec![0; N]; N];
    for i in 0..N {
        let pos0 = xy[i];
        for j in 0..N {
            let pos1 = xy[j];
            dist[i][j] = calc_dist2(pos0, pos1);
        }
    }

    println!("? 3 0 1 2");
    input_interactive! {
        edge: [(usize, usize); 2]
    }
    eprintln!("edge = {:?}", edge);

    Input {
        N,
        M,
        Q,
        L,
        W,
        G,
        range,
        xy,
        xy2,
        dist,
    }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub Q: usize,
    pub L: usize,
    pub W: usize,
    pub G: Vec<usize>,
    pub range: Vec<(usize, usize, usize, usize)>,
    pub xy: Vec<Coord>,
    pub xy2: Vec<Coord>,
    pub dist: Vec<Vec<usize>>,
}
