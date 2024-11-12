use std::collections::VecDeque;

use itertools::{iproduct, Itertools};
use rustc_hash::FxHashSet;

use crate::{
    coord::{Coord, DXY4, TWO_BY_TWO},
    input::Input,
};

pub struct State {
    pub grid_num: usize,
    pub delta: usize,
    pub score_map: Vec<Vec<isize>>,
    pub score_map_negative_no_score: Vec<Vec<isize>>,
}

impl State {
    pub fn new(grid_num: usize, input: &Input) -> Self {
        assert!(input.size % grid_num == 0);
        let delta = input.size / grid_num;
        let mut score_map = vec![vec![0; grid_num + 2]; grid_num + 2];
        for (x, y) in iproduct!(0..grid_num, 0..grid_num) {
            score_map[x + 1][y + 1] = calc_grid_score(Coord::new(x, y), delta, input);
        }

        Self {
            grid_num,
            delta,
            score_map,
            score_map_negative_no_score: vec![vec![]],
        }
    }
    pub fn make_no_score_coords(&self) -> FxHashSet<Coord> {
        make_connected_area(Coord::new(0, 0), &self.score_map)
    }
    pub fn grouping_saba_area(&mut self) -> Vec<(isize, Vec<Coord>)> {
        let mut unused_coords = FxHashSet::default();
        for (x, y) in iproduct!(0..self.grid_num + 2, 0..self.grid_num + 2) {
            unused_coords.insert(Coord::new(x, y));
        }
        let mut unused_coords = (&unused_coords - &self.make_no_score_coords())
            .iter()
            .cloned()
            .collect_vec();
        let mut score_map_negative_no_score = vec![vec![0; self.grid_num + 2]; self.grid_num + 2];
        for coord in unused_coords.iter() {
            score_map_negative_no_score[coord.x][coord.y] = 1;
        }
        let mut group: Vec<(isize, FxHashSet<Coord>)> = vec![];
        'outer: while let Some(coord) = unused_coords.pop() {
            for (_, g) in group.iter() {
                if g.contains(&coord) {
                    continue 'outer;
                }
            }
            let coords = make_connected_area(coord, &score_map_negative_no_score);
            let score_sum = coords
                .iter()
                .map(|coord| self.score_map[coord.x][coord.y])
                .sum::<isize>();
            group.push((score_sum, coords));
        }
        self.score_map_negative_no_score = score_map_negative_no_score;
        group
            .iter()
            .map(|v| (v.0, v.1.iter().cloned().collect_vec()))
            .collect_vec()
    }
    pub fn make_polygon(&self, coords: &Vec<Coord>) -> (Vec<Coord>, usize) {
        let mut area_map = vec![vec![0; self.grid_num + 2]; self.grid_num + 2];
        for coord in coords.iter() {
            area_map[coord.x][coord.y] = 1;
        }

        // 頂点
        let mut edges = vec![vec![false; self.grid_num + 1]; self.grid_num + 1];
        let mut edge_cnt = 0;
        for (x, y) in iproduct!(1..self.grid_num + 2, 1..self.grid_num + 2) {
            let coord = Coord::new(x, y);
            if is_edge(coord, &area_map) {
                edges[x - 1][y - 1] = true;
                edge_cnt += 1;
            }
        }

        // 辺の長さ
        let mut verticle_sum = 0;
        for (x, y) in iproduct!(0..self.grid_num + 2, 0..self.grid_num + 2) {
            let coord = Coord::new(x, y);
            verticle_sum += count_verticle(coord, &area_map);
        }
        verticle_sum /= 2;
        verticle_sum *= self.delta;
        if verticle_sum > 4e5 as usize {
            return (vec![], verticle_sum);
        }

        // 垂直方向の辺
        let mut v = vec![vec![false; self.grid_num + 1]; self.grid_num + 1];
        for (x, y) in iproduct!(0..self.grid_num + 1, 1..self.grid_num + 1) {
            if area_map[x][y] != area_map[x + 1][y] {
                v[x][y] = true;
            }
        }

        // 水平方向の辺
        let mut h = vec![vec![false; self.grid_num + 1]; self.grid_num + 1];
        for (x, y) in iproduct!(1..self.grid_num + 1, 0..self.grid_num + 1) {
            if area_map[x][y] != area_map[x][y + 1] {
                h[x][y] = true;
            }
        }

        // 頂点の連結リスト
        // 頂点からDOWN, LEFT方向に辺を通って動かして、頂点があれば連結リストに追加
        let mut G = vec![vec![vec![]; self.grid_num + 1]; self.grid_num + 1];
        for (x, y) in iproduct!(0..self.grid_num + 1, 0..self.grid_num + 1) {
            if !edges[x][y] {
                continue;
            }
            let pos = Coord::new(x, y);
            let dy = Coord::new(0, !0);
            let mut nxt = pos;
            loop {
                if !v[nxt.x][nxt.y] {
                    break;
                }
                nxt = nxt + dy;
                if !nxt.in_map(self.grid_num + 1) {
                    break;
                }
                if edges[nxt.x][nxt.y] {
                    G[pos.x][pos.y].push(nxt);
                    G[nxt.x][nxt.y].push(pos);
                    break;
                }
            }
            let dx = Coord::new(!0, 0);
            let mut nxt = pos;
            loop {
                if !h[nxt.x][nxt.y] {
                    break;
                }
                nxt = nxt + dx;
                if !nxt.in_map(self.grid_num + 1) {
                    break;
                }
                if edges[nxt.x][nxt.y] {
                    G[pos.x][pos.y].push(nxt);
                    G[nxt.x][nxt.y].push(pos);
                    break;
                }
            }
        }

        let mut cnt = 0;
        let mut start = Coord::new(!0, !0);
        for (x, y) in iproduct!(0..self.grid_num + 1, 0..self.grid_num + 1) {
            assert!(G[x][y].len() == 0 || G[x][y].len() == 2);
            cnt += G[x][y].len() / 2;
            if G[x][y].len() > 0 {
                start = G[x][y][0];
            }
        }
        assert!(cnt == edge_cnt);

        // どっち周りでもよいので、片方削除してBFSが片側だけに伸びるようにする
        G[start.x][start.y].remove(0);
        let mut visited = vec![vec![false; self.grid_num + 1]; self.grid_num + 1];
        let mut polygon = vec![start];
        visited[start.x][start.y] = true;
        let mut Q = VecDeque::new();
        Q.push_back(start);
        while let Some(pos) = Q.pop_front() {
            for &nxt in G[pos.x][pos.y].iter() {
                if visited[nxt.x][nxt.y] {
                    continue;
                }
                visited[nxt.x][nxt.y] = true;
                polygon.push(nxt);
                Q.push_back(nxt);
            }
        }
        (polygon, verticle_sum)
    }
}

pub fn calc_grid_score(grid_idx: Coord, delta: usize, input: &Input) -> isize {
    let xmin = grid_idx.x * delta;
    let ymin = grid_idx.y * delta;
    let xmax = (grid_idx.x + 1) * delta;
    let ymax = (grid_idx.y + 1) * delta;
    let mut saba_cnt = 0isize;
    let mut iwashi_cnt = 0isize;
    for (saba_pos, iwashi_pos) in input.saba.iter().zip(input.iwashi.iter()) {
        if xmin <= saba_pos.x && saba_pos.x <= xmax && ymin <= saba_pos.y && saba_pos.y <= ymax {
            saba_cnt += 1;
        }
        if xmin <= iwashi_pos.x
            && iwashi_pos.x <= xmax
            && ymin <= iwashi_pos.y
            && iwashi_pos.y <= ymax
        {
            iwashi_cnt += 1;
        }
    }
    saba_cnt - iwashi_cnt
}

fn make_connected_area(start: Coord, area: &Vec<Vec<isize>>) -> FxHashSet<Coord> {
    let n = area.len();
    let st = area[start.x][start.y] > 0;
    let mut visited = FxHashSet::default();
    visited.insert(start);
    let mut Q = VecDeque::new();
    Q.push_back(start);
    while let Some(pos) = Q.pop_front() {
        for dxy in DXY4.iter() {
            let nxt = pos + *dxy;
            if !nxt.in_map(n) || visited.contains(&nxt) || (area[nxt.x][nxt.y] > 0) != st {
                continue;
            }
            visited.insert(nxt);
            Q.push_back(nxt);
        }
    }
    visited
}

fn is_edge(coord: Coord, map: &Vec<Vec<usize>>) -> bool {
    let mut cnt = 0;
    for dxy in TWO_BY_TWO.iter() {
        let nxt = coord + *dxy;
        cnt += map[nxt.x][nxt.y];
    }
    cnt == 1 || cnt == 3
}

fn count_verticle(coord: Coord, map: &Vec<Vec<usize>>) -> usize {
    let mut cnt = 0;
    for dxy in DXY4.iter() {
        let nxt = coord + *dxy;
        if !nxt.in_map(map.len()) {
            continue;
        }
        if map[coord.x][coord.y] != map[nxt.x][nxt.y] {
            cnt += 1;
        }
    }
    cnt
}

#[cfg(test)]
mod tests {

    use colored::*;

    use crate::{coord::Coord, input::read_input};

    use super::State;

    #[test]
    fn state() {
        let input = &read_input();
        let grid_num = 20;
        let state = State::new(grid_num, input);
        let no_score_coords = state.make_no_score_coords();
        for y in (0..grid_num + 2).rev() {
            for x in 0..grid_num + 2 {
                if no_score_coords.contains(&Coord::new(x, y)) {
                    print!("{}", "■ ".to_string().white());
                } else {
                    print!("{}", "□ ".to_string().white());
                }
            }
            println!();
        }
    }
}
