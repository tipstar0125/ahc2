use std::collections::BTreeSet;

use crate::input::Input;

#[derive(Debug, Clone, Copy)]
pub struct Op {
    pub dir: char,
    pub idx: usize,
}

#[derive(Debug, Clone)]
pub struct State {
    pub N: usize,
    pub field: Vec<Vec<char>>,
    pub score: i64, // 各鬼から壁までの距離の総和(最小化)
    pub hash: usize,
    pub x_positions: BTreeSet<(usize, usize)>,
}

impl State {
    pub fn new(input: &Input) -> Self {
        let mut x_positions = BTreeSet::new();
        let mut score = 0;
        let mut hash = 0;
        for i in 0..input.N {
            for j in 0..input.N {
                if input.C[i][j] == 'x' {
                    x_positions.insert((i, j));
                    score += calc_dist(i, j, input.N);
                    hash ^= input.calc_hash.hash_map[i][j];
                }
            }
        }
        Self {
            N: input.N,
            field: input.C.clone(),
            score,
            hash,
            x_positions,
        }
    }
    pub fn can_shift(&self, dir: char, idx: usize) -> bool {
        match dir {
            'L' => self.field[idx][0] != 'o',
            'R' => self.field[idx][self.N - 1] != 'o',
            'U' => self.field[0][idx] != 'o',
            'D' => self.field[self.N - 1][idx] != 'o',
            _ => unreachable!(),
        }
    }
    pub fn cand(&self, input: &Input) -> Vec<(i64, usize, Op, bool)> {
        let mut cand = vec![];

        for i in 0..self.N {
            if self.can_shift('L', i) {
                let mut prev = 0;
                let mut next = 0;
                let mut next_hash = self.hash;
                for &(row, col) in self.x_positions.iter() {
                    if row == i {
                        prev += calc_dist(row, col, self.N);
                        if col > 0 {
                            next += calc_dist(row, col - 1, self.N);
                        }
                        next_hash = input.calc_hash.calc(next_hash, row, col, 'L');
                    }
                }
                let diff = next - prev;
                let next_score = self.score + diff;
                cand.push((
                    next_score,
                    next_hash,
                    Op { dir: 'L', idx: i },
                    next_score == 0,
                ));
            }
            if self.can_shift('R', i) {
                let mut prev = 0;
                let mut next = 0;
                let mut next_hash = self.hash;
                for &(row, col) in self.x_positions.iter() {
                    if row == i {
                        prev += calc_dist(row, col, self.N);
                        if col + 1 < self.N {
                            next += calc_dist(row, col + 1, self.N);
                        }
                        next_hash = input.calc_hash.calc(next_hash, row, col, 'R');
                    }
                }
                let diff = next - prev;
                let next_score = self.score + diff;
                cand.push((
                    next_score,
                    next_hash,
                    Op { dir: 'R', idx: i },
                    next_score == 0,
                ));
            }
        }

        for j in 0..self.N {
            if self.can_shift('U', j) {
                let mut prev = 0;
                let mut next = 0;
                let mut next_hash = self.hash;
                for &(row, col) in self.x_positions.iter() {
                    if col == j {
                        prev += calc_dist(row, col, self.N);
                        if row > 0 {
                            next += calc_dist(row - 1, col, self.N);
                        }
                        next_hash = input.calc_hash.calc(next_hash, row, col, 'U');
                    }
                }
                let diff = next - prev;
                let next_score = self.score + diff;
                cand.push((
                    next_score,
                    next_hash,
                    Op { dir: 'U', idx: j },
                    next_score == 0,
                ));
            }
            if self.can_shift('D', j) {
                let mut prev = 0;
                let mut next = 0;
                let mut next_hash = self.hash;
                for &(row, col) in self.x_positions.iter() {
                    if col == j {
                        prev += calc_dist(row, col, self.N);
                        if row + 1 < self.N {
                            next += calc_dist(row + 1, col, self.N);
                        }
                        next_hash = input.calc_hash.calc(next_hash, row, col, 'D');
                    }
                }
                let diff = next - prev;
                let next_score = self.score + diff;
                cand.push((
                    next_score,
                    next_hash,
                    Op { dir: 'D', idx: j },
                    next_score == 0,
                ));
            }
        }
        cand
    }
    pub fn shift(&mut self, op: &Op) {
        if op.dir == 'L' {
            for j in 1..self.N {
                self.field[op.idx][j - 1] = self.field[op.idx][j];
            }
            self.field[op.idx][self.N - 1] = '.';
        } else if op.dir == 'R' {
            for j in (1..self.N).rev() {
                self.field[op.idx][j] = self.field[op.idx][j - 1];
            }
            self.field[op.idx][0] = '.';
        } else if op.dir == 'U' {
            for i in 1..self.N {
                self.field[i - 1][op.idx] = self.field[i][op.idx];
            }
            self.field[self.N - 1][op.idx] = '.';
        } else if op.dir == 'D' {
            for i in (1..self.N).rev() {
                self.field[i][op.idx] = self.field[i - 1][op.idx];
            }
            self.field[0][op.idx] = '.';
        }
    }
    pub fn apply(&mut self, score: i64, hash: usize, op: &Op, _input: &Input) {
        self.score = score;
        self.hash = hash;
        self.shift(&op);
        self.x_positions.clear();

        for i in 0..self.N {
            for j in 0..self.N {
                if self.field[i][j] == 'x' {
                    self.x_positions.insert((i, j));
                }
            }
        }
    }
}

fn calc_dist(row: usize, col: usize, N: usize) -> i64 {
    (row + 1).min(N - row).min(col + 1).min(N - col) as i64
}
