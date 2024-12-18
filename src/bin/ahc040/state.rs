use crate::common::SetMinMax;
use crate::input::Input;

#[derive(Clone, Copy, Debug)]
pub struct Op {
    pub p: usize,
    pub r: bool,
    pub d: char,
    pub b: i32,
    pub pos: Pos,
    pub row: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x1: i64,
    pub x2: i64,
    pub y1: i64,
    pub y2: i64,
    pub r: bool,
    pub t: i32,
}

pub const P0: Pos = Pos {
    x1: -1,
    x2: -1,
    y1: -1,
    y2: -1,
    r: false,
    t: -1,
};

#[derive(Debug, Clone)]
pub struct Shelf {
    width: i64,
    height: i64,
    box_num: usize,
    right_edge: i32,
    margin: i64,
}

impl Shelf {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            box_num: 0,
            right_edge: -1,
            margin: 0,
        }
    }
    pub fn calc_margin(&self, idx: usize, rotate: bool, input: &Input) -> i64 {
        let mut w = input.wh2[idx].0;
        let mut h = input.wh2[idx].1;
        if rotate {
            std::mem::swap(&mut w, &mut h);
        }

        let shelf_width = self.width + w;
        let shelf_height = self.height.max(h);

        if input.width_limit - shelf_width < 1e4 as i64 {
            return 0;
        }

        let margin = (input.width_limit - shelf_width) * shelf_height;
        margin
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub n: usize,
    pub pos: Vec<Pos>, // (x1, x2, y1, y2, rot, t)
    pub lines: Vec<Shelf>,
    pub W2: i64,
    pub H2: i64,
    pub shelf_height: i64,
    pub shelf_margin: i64,
    pub S: i64,
    pub score: usize,
    pub hash: usize,
}

impl State {
    pub fn new(input: &Input) -> Self {
        let mut init_hash = 0;
        for i in 0..input.calc_hash.MAX {
            init_hash ^= input.calc_hash.hash_map[i][0];
        }
        let mut S = 0;
        for i in 0..input.N {
            S += input.wh2[i].0 as i64 * input.wh2[i].1 as i64;
        }

        Self {
            n: input.N,
            pos: vec![],
            lines: vec![Shelf::new()],
            W2: 0,
            H2: 0,
            shelf_height: 0,
            S,
            shelf_margin: 0,
            score: 0,
            hash: init_hash,
        }
    }
    pub fn calc_length(&self, c: Op, input: &Input) -> (i64, i64, Pos) {
        let (mut w, mut h) = input.wh2[c.p];
        if c.r {
            std::mem::swap(&mut w, &mut h);
        }
        let pos = if c.d == 'U' {
            let x1 = if c.b < 0 {
                0
            } else {
                self.pos[c.b as usize].x2
            };
            let x2 = x1 + w;
            let y1 = self
                .pos
                .iter()
                .filter_map(|q| {
                    if q.t >= 0 && x1.max(q.x1) < x2.min(q.x2) {
                        Some(q.y2)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0);
            let y2 = y1 + h;
            Pos {
                x1,
                x2,
                y1,
                y2,
                r: c.r,
                t: c.p as i32,
            }
        } else {
            let y1 = if c.b < 0 {
                0
            } else {
                self.pos[c.b as usize].y2
            };
            let y2 = y1 + h;
            let x1 = self
                .pos
                .iter()
                .filter_map(|q| {
                    if q.t >= 0 && y1.max(q.y1) < y2.min(q.y2) {
                        Some(q.x2)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or(0);
            let x2 = x1 + w;
            Pos {
                x1,
                x2,
                y1,
                y2,
                r: c.r,
                t: c.p as i32,
            }
        };
        let mut W2 = self.W2;
        let mut H2 = self.H2;
        W2.setmax(pos.x2);
        H2.setmax(pos.y2);
        (W2, H2, pos)
    }
    pub fn cand(
        &self,
        input: &Input,
    ) -> Vec<(
        usize, // added score
        usize, // hash
        Op,
        bool, // is_done
    )> {
        let mut cand = vec![];
        let turn = self.pos.len();
        let w = input.wh2[turn].0;
        let h = input.wh2[turn].1;
        for (i, line) in self.lines.iter().enumerate() {
            let &Shelf {
                width,
                height,
                box_num: _,
                right_edge,
                margin,
            } = line;

            let mut append_cand = |rot: bool| {
                let mut op = Op {
                    p: turn,
                    r: rot,
                    d: 'U',
                    b: right_edge,
                    pos: P0,
                    row: i,
                };
                let (w, h, pos) = self.calc_length(op, input);
                let raw_score = w + h;

                // 更新後の棚の高さの合計を計算
                let next_box_height = if op.r {
                    input.wh2[op.p].0
                } else {
                    input.wh2[op.p].1
                };
                let next_height = height.max(next_box_height);
                let delta_height = next_height - height;
                let shelf_height = self.shelf_height + delta_height;

                // 更新後の棚の余白の合計を計算
                let row_margin = self.lines[i].calc_margin(op.p, op.r, input);
                let delta_margin = row_margin - margin;
                let next_margin = self.shelf_margin + delta_margin;

                // まだ置いていない箱の面積の合計から、既に置かれている棚の余白の合計を引き、
                // 将来追加する棚の面積を求める
                let rest_S = (self.S - next_margin).max(0);

                // 棚の高さの予測から、予測スコアを計算
                let predict_height = shelf_height + (rest_S / input.width_limit);
                let predict_score = predict_height + input.width_limit;

                // ターンの進行度に応じて、予測スコアと実際のスコアを切り替える
                let score = if (turn as f64) < input.N as f64 * 0.7 {
                    predict_score
                } else {
                    raw_score
                };

                let delta_score = score as usize - self.score;
                let after_width = width
                    + if rot {
                        input.wh2[turn].1
                    } else {
                        input.wh2[turn].0
                    };
                let hash = input.calc_hash.calc(self.hash, i, width, after_width);
                op.pos = pos;
                cand.push((delta_score, hash, op, turn + 1 == input.N));
            };

            if w < h {
                if i == 0 && width + w <= input.width_limit {
                    append_cand(false);
                }
            } else {
                if i == 0 && width + h <= input.width_limit {
                    append_cand(true);
                }
            }
            if i > 0 {
                let sigma = input.sigma * 5;
                let up_length_sum = self.lines[i - 1].width;
                if i <= 5 {
                    if w < h {
                        if (width + w + sigma <= up_length_sum
                            || up_length_sum + w >= input.width_limit)
                            && width + w <= input.width_limit
                        {
                            append_cand(false);
                        }
                    } else {
                        if (width + h + sigma <= up_length_sum
                            || up_length_sum + h >= input.width_limit)
                            && width + h <= input.width_limit
                        {
                            append_cand(true);
                        }
                    }
                } else {
                    if (width + w + sigma <= up_length_sum
                        || up_length_sum + w >= input.width_limit)
                        && width + w <= input.width_limit
                    {
                        append_cand(false);
                    }
                    if (width + h + sigma <= up_length_sum
                        || up_length_sum + h >= input.width_limit)
                        && width + h <= input.width_limit
                    {
                        append_cand(true);
                    }
                }
            }
        }
        cand
    }
    pub fn apply(&mut self, score: usize, hash: usize, op: &Op, input: &Input) {
        let row = op.row;
        if self.lines[row].box_num == 0 {
            assert!(self.lines.len() == row + 1);
            self.lines.push(Shelf::new());
        }

        // 更新した棚の余白の差分を計算
        let margin = self.lines[row].calc_margin(op.p, op.r, input);
        let delta_margin = margin - self.lines[row].margin;

        // 更新した棚の高さの差分を計算
        let h = if op.r {
            input.wh2[op.p].0
        } else {
            input.wh2[op.p].1
        };
        let height = self.lines[row].height.max(h);
        let delta_height = height - self.lines[row].height;

        // 棚の更新
        self.lines[row].height = height;
        self.lines[row].width = op.pos.x2;
        self.lines[row].right_edge = op.p as i32;
        self.lines[row].box_num += 1;
        self.lines[row].margin = margin;

        self.pos.push(op.pos);
        self.score = score;
        self.hash = hash;
        self.shelf_height += delta_height; // 棚の高さの合計を更新
        self.shelf_margin += delta_margin; // 棚の余白の合計を更新
        assert!(self.shelf_margin >= 0);
        self.W2.setmax(op.pos.x2);
        self.H2.setmax(op.pos.y2);
        self.S -= input.wh2[op.p].0 * input.wh2[op.p].1; // 残りの面積を更新
    }
}
