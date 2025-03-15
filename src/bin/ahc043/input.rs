use proconio::input;

use crate::coord::{calc_manhattan_dist, Coord, ADJ};

pub fn read_input() -> Input {
    input! {
        N: usize,
        M: usize,
        K: usize,
        T: usize,
        pos: [(usize, usize, usize, usize,); M],
    }

    let mut home = vec![];
    let mut workspace = vec![];
    let mut pair_dist = vec![];
    let mut home_workspace_field = vec![vec![vec![]; N]; N];

    for (idx, &(ih, jh, iw, jw)) in pos.iter().enumerate() {
        let home_coord = Coord::new(ih, jh);
        let workspace_coord = Coord::new(iw, jw);
        let dist = calc_manhattan_dist(&home_coord, &workspace_coord);
        home.push(home_coord);
        workspace.push(workspace_coord);
        home_workspace_field[ih][jh].push(idx);
        home_workspace_field[iw][jw].push(idx + M);
        pair_dist.push(dist);
    }

    // 各マスからマンハッタン距離2以下のマスにある自宅と会社を列挙
    // ただし、空の場合は除く
    let mut covers = vec![];
    let mut cover_field = vec![vec![vec![]; N]; N];

    for i in 0..N {
        for j in 0..N {
            let pos = Coord::new(i, j);
            let mut cover = vec![];

            for &dij in ADJ.iter() {
                let nxt = pos + dij;
                if nxt.in_map(N) {
                    cover.extend(home_workspace_field[nxt.i][nxt.j].iter().copied());
                }
            }
            if !cover.is_empty() {
                covers.push((pos, cover.clone()));
                cover_field[i][j] = cover;
            }
        }
    }

    eprintln!("M = {}", M);
    eprintln!("K = {}", K);

    Input {
        N,
        M,
        K,
        T,
        home,
        workspace,
        covers,
        cover_field,
        pair_dist,
        home_workspace_field,
        TLE: 2.9,
    }
}

#[derive(Debug)]
pub struct Input {
    pub N: usize,
    pub M: usize,
    pub K: usize,
    pub T: usize,
    pub home: Vec<Coord>,
    pub workspace: Vec<Coord>,
    pub covers: Vec<(Coord, Vec<usize>)>,
    pub cover_field: Vec<Vec<Vec<usize>>>,
    pub pair_dist: Vec<usize>,
    pub home_workspace_field: Vec<Vec<Vec<usize>>>,
    pub TLE: f64,
}
