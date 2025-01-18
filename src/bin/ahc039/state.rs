use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{
    common::{get_mask9, get_time},
    coord::{Coord, DXY4},
    input::Input,
};

pub struct State {
    pub grid_num: usize,
    pub dl: i64,
    pub grid: Vec<Vec<bool>>,
    pub length: i64,
    pub score: i64,
    pub score_map: Vec<Vec<i64>>,
    pub best_dl: i64,
    pub best_length: i64,
    pub best_grid: Vec<Vec<bool>>,
    pub best_score: i64,
}

impl State {
    pub fn new(grid_num: usize, input: &Input) -> Self {
        assert!(input.size % grid_num == 0);
        let grid = vec![vec![true; grid_num]; grid_num];
        let dl = input.size / grid_num;
        let score_map = calc_score_map(grid_num, input);

        Self {
            grid_num,
            dl: dl as i64,
            grid,
            length: input.size as i64 * 4,
            score: 0,
            score_map,
            best_dl: dl as i64,
            best_length: input.size as i64 * 4,
            best_grid: vec![],
            best_score: 0,
        }
    }
    pub fn annealing(&mut self, rng: &mut Pcg64Mcg, connect9: &Vec<bool>, tle: f64) {
        let mut iter = 0;
        let T0 = 1e4 / self.grid_num as f64 / self.grid_num as f64;
        let T1 = T0 * 0.1;
        while get_time() < tle {
            // ランダムにグリッドを選んで、エッジに隣接していれば、追加または削除を行う
            let x = rng.gen_range(0..self.grid_num);
            let y = rng.gen_range(0..self.grid_num);
            let pos = Coord::new(x, y);
            if !self.is_next_to_edge(pos) {
                continue;
            }
            let added = !self.grid[x][y];
            if !self.legal_action(pos, added, connect9) {
                continue;
            }
            let diff_length = self.calc_diff_length(pos);
            if self.length + diff_length > 4e5 as i64 {
                continue;
            }
            let diff_score = self.calc_diff_score(pos);
            let temp = T0 + (T1 - T0) * get_time() / tle;
            if diff_score >= 0 || rng.gen_bool((diff_score as f64 / temp).exp()) {
                iter += 1;
                self.length += diff_length;
                self.score += diff_score;
                self.grid[pos.x][pos.y] ^= true;
                if self.score > self.best_score {
                    self.best_score = self.score;
                    self.best_grid = self.grid.clone();
                    self.best_length = self.length;
                    self.best_dl = self.dl;
                    #[cfg(feature = "local")]
                    eprintln!(
                        "grid num: {} score: {} elapsed: {}",
                        self.grid_num,
                        self.best_score,
                        get_time()
                    );
                }
            }
        }
        eprintln!("Iter = {}", iter);
    }
    pub fn is_next_to_edge(&self, pos: Coord) -> bool {
        // エッジに隣接しているグリッドかどうか
        for dxy in &DXY4 {
            let nxt = pos + *dxy;
            if self.grid[pos.x][pos.y] && !nxt.in_map(self.grid_num) {
                return true;
            }
            if !nxt.in_map(self.grid_num) {
                continue;
            }
            if self.grid[pos.x][pos.y] != self.grid[nxt.x][nxt.y] {
                return true;
            }
        }
        false
    }
    pub fn legal_action(&self, pos: Coord, added: bool, connect9: &Vec<bool>) -> bool {
        // 削除しても連結かどうか
        // 追加する場合は、ビット反転して網ではない方で考え、削除しても連結かどうか
        let mut mask = get_mask9(&self.grid, pos.x, pos.y);
        if added {
            mask ^= 0x1FF;
        }
        connect9[mask]
    }
    pub fn calc_diff_length(&self, pos: Coord) -> i64 {
        // 多角形の長さの差分を計算
        // 隣接しているグリッドが同じ状態の数を数える(外周の場合は網ではないとする)
        // 1つだけ隣接している場合は-2、3つ隣接している場合は+2
        let mut cnt = 0;
        for dxy in &DXY4 {
            let nxt = pos + *dxy;
            if !self.grid[pos.x][pos.y] && !nxt.in_map(self.grid_num) {
                cnt += 1;
                continue;
            }
            if !nxt.in_map(self.grid_num) {
                continue;
            }
            if self.grid[pos.x][pos.y] == self.grid[nxt.x][nxt.y] {
                cnt += 1;
            }
        }
        if cnt == 1 {
            self.dl * (-2)
        } else if cnt == 3 {
            self.dl * 2
        } else {
            0
        }
    }
    pub fn calc_diff_score(&self, pos: Coord) -> i64 {
        // スコアの差分を計算
        let mut diff_score = self.score_map[pos.x][pos.y];
        if self.grid[pos.x][pos.y] {
            diff_score *= -1;
        }
        diff_score
    }
    pub fn to_next_grid(&mut self, grid_num: usize, input: &Input) {
        // 次のグリッド分割に移行
        let dl = input.size / grid_num;
        let score_map = calc_score_map(grid_num, input);
        let mut grid = vec![vec![false; grid_num]; grid_num];
        let mut score = 0;
        for x in 0..grid_num {
            for y in 0..grid_num {
                let bx = x * dl / (self.dl as usize);
                let by = y * dl / (self.dl as usize);
                if self.grid[bx][by] {
                    grid[x][y] = true;
                    score += score_map[x][y];
                }
            }
        }
        let mut length = 0;
        for x in 0..grid_num {
            for y in 0..grid_num {
                if !grid[x][y] {
                    continue;
                }
                let pos = Coord::new(x, y);
                for dxy in &DXY4 {
                    let nxt = pos + *dxy;
                    if !nxt.in_map(grid_num) || !grid[nxt.x][nxt.y] {
                        length += dl;
                    }
                }
            }
        }
        self.grid = grid;
        self.dl = dl as i64;
        self.score_map = score_map;
        self.grid_num = grid_num;
        self.score = score;
        self.length = length as i64;
    }
}

fn calc_score_map(grid_num: usize, input: &Input) -> Vec<Vec<i64>> {
    let dl = input.size / grid_num;
    let mut score_map = vec![vec![0; grid_num]; grid_num];
    for pos in input.saba.iter() {
        let x = pos.x / dl;
        let y = pos.y / dl;
        score_map[x][y] += 1;
    }
    for pos in input.iwashi.iter() {
        let x = pos.x / dl;
        let y = pos.y / dl;
        score_map[x][y] -= 1;
    }
    score_map
}
