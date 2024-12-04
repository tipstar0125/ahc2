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
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
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
pub struct State {
    pub n: usize,
    pub pos: Vec<Pos>,                 // (x1, x2, y1, y2, rot, t)
    pub lines: Vec<(i32, i32, usize)>, // (length sum, right edge number, cnt)
    pub W2: i32,
    pub H2: i32,
    pub score: usize,
    pub hash: usize,
}

impl State {
    pub fn new(input: &Input) -> Self {
        Self {
            n: input.N,
            pos: vec![],
            lines: vec![(0, -1, 0)],
            W2: 0,
            H2: 0,
            score: 0,
            hash: 0,
        }
    }
    pub fn calc_length(&self, c: Op, input: &Input) -> (i32, i32, Pos) {
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
            let &(length_sum, right_edge, j) = line;
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
                let score = w + h;
                let delta_score = score as usize - self.score;
                // let hash = input.calc_hash.calc(self.hash, i, j + 1, turn, rot);
                op.pos = pos;
                cand.push((delta_score, score as usize, op, turn + 1 == input.N));
            };

            if i == 0 && length_sum + w <= input.width_limit && j + 1 < input.calc_hash.MAX {
                append_cand(false);
            }
            if i == 0 && length_sum + h <= input.width_limit && j + 1 < input.calc_hash.MAX {
                append_cand(true);
            }
            if i > 0 {
                let up_length_sum = self.lines[i - 1].0;
                if (length_sum + w + input.sigma * 100 <= up_length_sum
                    || up_length_sum + w >= input.width_limit)
                    && j + 1 < input.calc_hash.MAX
                {
                    append_cand(false);
                }
                if (length_sum + h + input.sigma * 100 <= up_length_sum
                    || up_length_sum + w >= input.width_limit)
                    && j + 1 < input.calc_hash.MAX
                {
                    append_cand(true);
                }
            }
        }
        cand
    }
    pub fn apply(&mut self, score: usize, hash: usize, op: &Op, input: &Input) {
        let row = op.row;
        if self.lines[row].2 == 0 && self.lines.len() < input.calc_hash.MAX {
            assert!(self.lines.len() == row + 1);
            self.lines.push((0, -1, 0));
        }
        self.lines[row] = (op.pos.x2 as i32, op.p as i32, self.lines[row].2 + 1);
        self.pos.push(op.pos);
        self.score = score;
        self.hash = hash;
        self.W2.setmax(op.pos.x2);
        self.H2.setmax(op.pos.y2);
    }
}
