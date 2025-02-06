use crate::input::Input;

pub struct State {
    N: usize,
    field: Vec<Vec<char>>,
}

impl State {
    pub fn new(input: &Input) -> Self {
        Self {
            N: input.N,
            field: input.C.clone(),
        }
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
    pub fn greedy(&mut self) {
        let mut cnt = 0;
        let mut ans = vec![];

        while cnt < self.N * 2 {
            let mut candidates = vec![];
            for i in 0..self.N {
                let mut x_pos_right_idx = !0;
                let mut x_cnt = 0;
                for j in 0..self.N {
                    if self.field[i][j] == 'o' {
                        break;
                    }
                    if self.field[i][j] == 'x' {
                        x_pos_right_idx = j;
                        x_cnt += 1;
                    }
                }
                if x_pos_right_idx != !0 {
                    let shift_num = x_pos_right_idx + 1;
                    candidates.push((x_cnt as f64 / shift_num as f64, shift_num, x_cnt, 'L', i));
                }

                let mut x_pos_left_idx = !0;
                let mut x_cnt = 0;
                for j in (0..self.N).rev() {
                    if self.field[i][j] == 'o' {
                        break;
                    }
                    if self.field[i][j] == 'x' {
                        x_pos_left_idx = j;
                        x_cnt += 1;
                    }
                }
                if x_pos_left_idx != !0 {
                    let shift_num = self.N - x_pos_left_idx;
                    candidates.push((x_cnt as f64 / shift_num as f64, shift_num, x_cnt, 'R', i));
                }
            }
            for j in 0..self.N {
                let mut x_pos_down_idx = !0;
                let mut x_cnt = 0;
                for i in 0..self.N {
                    if self.field[i][j] == 'o' {
                        break;
                    }
                    if self.field[i][j] == 'x' {
                        x_pos_down_idx = i;
                        x_cnt += 1;
                    }
                }
                if x_pos_down_idx != !0 {
                    let shift_num = x_pos_down_idx + 1;
                    candidates.push((x_cnt as f64 / shift_num as f64, shift_num, x_cnt, 'U', j));
                }

                let mut x_pos_up_idx = !0;
                let mut x_cnt = 0;
                for i in (0..self.N).rev() {
                    if self.field[i][j] == 'o' {
                        break;
                    }
                    if self.field[i][j] == 'x' {
                        x_pos_up_idx = i;
                        x_cnt += 1;
                    }
                }
                if x_pos_up_idx != !0 {
                    let shift_num = self.N - x_pos_up_idx;
                    candidates.push((x_cnt as f64 / shift_num as f64, shift_num, x_cnt, 'D', j));
                }
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
            match dir {
                'L' => ans.extend(self.shift_right(idx, shift_num)),
                'R' => ans.extend(self.shift_left(idx, shift_num)),
                'U' => ans.extend(self.shift_down(idx, shift_num)),
                'D' => ans.extend(self.shift_up(idx, shift_num)),
                _ => unreachable!(),
            }
        }

        for a in ans {
            println!("{}", a);
        }
    }
}
