use std::cmp::Reverse;

use crate::input::Input;

#[derive(Clone)]
pub struct State {
    pub N: usize,
    pub field: Vec<Vec<char>>,
    pub shift_cnt: usize,
}

impl State {
    pub fn new(input: &Input) -> Self {
        Self {
            N: input.N,
            field: input.C.clone(),
            shift_cnt: 0,
        }
    }
    pub fn get_score(&self) -> usize {
        8 * self.N * self.N - self.shift_cnt
    }
    pub fn shift_left(&mut self, row: usize, shift_num: usize) -> Vec<String> {
        let mut ret = vec![];
        for _ in 0..shift_num {
            for j in 1..self.N {
                self.field[row][j - 1] = self.field[row][j];
            }
            self.field[row][self.N - 1] = '.';
            ret.push(format!("L {}", row));
        }
        ret
    }
    pub fn shift_right(&mut self, row: usize, shift_num: usize) -> Vec<String> {
        let mut ret = vec![];
        for _ in 0..shift_num {
            for j in (1..self.N).rev() {
                self.field[row][j] = self.field[row][j - 1];
            }
            self.field[row][0] = '.';
            ret.push(format!("R {}", row));
        }
        ret
    }
    pub fn shift_up(&mut self, col: usize, shift_num: usize) -> Vec<String> {
        let mut ret = vec![];
        for _ in 0..shift_num {
            for i in 1..self.N {
                self.field[i - 1][col] = self.field[i][col];
            }
            self.field[self.N - 1][col] = '.';
            ret.push(format!("U {}", col));
        }
        ret
    }
    pub fn shift_down(&mut self, col: usize, shift_num: usize) -> Vec<String> {
        let mut ret = vec![];
        for _ in 0..shift_num {
            for i in (1..self.N).rev() {
                self.field[i][col] = self.field[i - 1][col];
            }
            self.field[0][col] = '.';
            ret.push(format!("D {}", col));
        }
        ret
    }
    pub fn get_greedy_legal_action(&self) -> Vec<(f64, usize, usize, char, usize)> {
        let mut candidates = vec![];
        for i in 0..self.N {
            let mut x_cnt = 0;
            for j in 0..self.N {
                if self.field[i][j] == 'o' {
                    break;
                }
                if self.field[i][j] == 'x' {
                    x_cnt += 1;
                    let shift_num = j + 1;
                    candidates.push((x_cnt as f64 / shift_num as f64, shift_num, x_cnt, 'L', i));
                }
            }

            let mut x_cnt = 0;
            for j in (0..self.N).rev() {
                if self.field[i][j] == 'o' {
                    break;
                }
                if self.field[i][j] == 'x' {
                    x_cnt += 1;
                    let shift_num = self.N - j;
                    candidates.push((x_cnt as f64 / shift_num as f64, shift_num, x_cnt, 'R', i));
                }
            }
        }
        for j in 0..self.N {
            let mut x_cnt = 0;
            for i in 0..self.N {
                if self.field[i][j] == 'o' {
                    break;
                }
                if self.field[i][j] == 'x' {
                    x_cnt += 1;
                    let shift_num = i + 1;
                    candidates.push((x_cnt as f64 / shift_num as f64, shift_num, x_cnt, 'U', j));
                }
            }

            let mut x_cnt = 0;
            for i in (0..self.N).rev() {
                if self.field[i][j] == 'o' {
                    break;
                }
                if self.field[i][j] == 'x' {
                    x_cnt += 1;
                    let shift_num = self.N - i;
                    candidates.push((x_cnt as f64 / shift_num as f64, shift_num, x_cnt, 'D', j));
                }
            }
        }
        candidates
    }
    pub fn greedy(&mut self) {
        let mut cnt = 0;
        let mut ans = vec![];

        while cnt < self.N * 2 {
            let mut candidates = self.get_greedy_legal_action();
            if candidates.is_empty() {
                break;
            }
            candidates.sort_by(|a, b| b.partial_cmp(a).unwrap());
            let (_, shift_num, x_cnt, dir, idx) = candidates[0];
            cnt += x_cnt;

            match dir {
                'L' => ans.extend(self.shift_left(idx, shift_num)),
                'R' => ans.extend(self.shift_right(idx, shift_num)),
                'U' => ans.extend(self.shift_up(idx, shift_num)),
                'D' => ans.extend(self.shift_down(idx, shift_num)),
                _ => unreachable!(),
            }
            self.shift_cnt += shift_num;
        }

        for a in ans {
            println!("{}", a);
        }

        let mut cnt = 2 * self.N;
        for i in 0..self.N {
            for j in 0..self.N {
                if self.field[i][j] == 'x' {
                    cnt -= 1;
                }
            }
        }

        self.greedy_dist(true, 0, cnt);
    }
    pub fn get_min_dist(&self, row: usize, col: usize) -> usize {
        let mut dist = 1 << 60;

        // left
        let mut d = 1;
        for j in 0..col {
            d += 1;
            if self.field[row][j] == 'o' {
                d += 1;
            }
        }
        dist = dist.min(d);

        // right
        let mut d = 1;
        for j in (col + 1)..self.N {
            d += 1;
            if self.field[row][j] == 'o' {
                d += 1;
            }
        }
        dist = dist.min(d);

        // up
        let mut d = 1;
        for i in 0..row {
            d += 1;
            if self.field[i][col] == 'o' {
                d += 1;
            }
        }
        dist = dist.min(d);

        // down
        let mut d = 1;
        for i in (row + 1)..self.N {
            d += 1;
            if self.field[i][col] == 'o' {
                d += 1;
            }
        }
        dist = dist.min(d);
        dist
    }
    pub fn get_field_min_dist(&self) -> usize {
        let mut dist = 0;
        for i in 0..self.N {
            for j in 0..self.N {
                if self.field[i][j] == 'x' {
                    dist += self.get_min_dist(i, j);
                }
            }
        }
        dist
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
    pub fn get_greedy_dist_legal_action(
        &mut self,
    ) -> Vec<(usize, Reverse<usize>, char, usize, bool)> {
        let mut candidates = vec![];

        for i in 0..self.N {
            let mut x_cnt = 0;
            for j in 0..self.N {
                if self.field[i][j] == 'x' {
                    x_cnt += 1;
                }
            }
            if self.can_shift('L', i) {
                let remove_x = self.field[i][0] == 'x';
                self.shift_left(i, 1);
                let s = self.get_field_min_dist();
                candidates.push((s, Reverse(x_cnt), 'L', i, remove_x));
                self.shift_right(i, 1);
                if remove_x {
                    self.field[i][0] = 'x';
                }
            }
            if self.can_shift('R', i) {
                let remove_x = self.field[i][self.N - 1] == 'x';
                self.shift_right(i, 1);
                let s = self.get_field_min_dist();
                candidates.push((s, Reverse(x_cnt), 'R', i, remove_x));
                self.shift_left(i, 1);
                if remove_x {
                    self.field[i][self.N - 1] = 'x';
                }
            }
        }
        for j in 0..self.N {
            let mut x_cnt = 0;
            for i in 0..self.N {
                if self.field[i][j] == 'x' {
                    x_cnt += 1;
                }
            }
            if self.can_shift('U', j) {
                let remove_x = self.field[0][j] == 'x';
                self.shift_up(j, 1);
                let s = self.get_field_min_dist();
                candidates.push((s, Reverse(x_cnt), 'U', j, remove_x));
                self.shift_down(j, 1);
                if remove_x {
                    self.field[0][j] = 'x';
                }
            }
            if self.can_shift('D', j) {
                let remove_x = self.field[self.N - 1][j] == 'x';
                self.shift_down(j, 1);
                let s = self.get_field_min_dist();
                candidates.push((s, Reverse(x_cnt), 'D', j, remove_x));
                self.shift_up(j, 1);
                if remove_x {
                    self.field[self.N - 1][j] = 'x';
                }
            }
        }
        candidates
    }
    pub fn greedy_dist(&mut self, verbose: bool, prune_score: usize, mut cnt: usize) -> bool {
        let mut ans = vec![];

        while cnt < self.N * 2 {
            if prune_score > self.get_score() {
                return false;
            }
            let mut candidates = self.get_greedy_dist_legal_action();
            candidates.sort();
            let (_, _, dir, idx, remove_x) = candidates[0];
            if remove_x {
                cnt += 1;
            }
            match dir {
                'L' => ans.extend(self.shift_left(idx, 1)),
                'R' => ans.extend(self.shift_right(idx, 1)),
                'U' => ans.extend(self.shift_up(idx, 1)),
                'D' => ans.extend(self.shift_down(idx, 1)),
                _ => unreachable!(),
            };
            self.shift_cnt += 1;
        }

        if verbose {
            for a in ans {
                println!("{}", a);
            }
        }
        true
    }
}
