use crate::common::SetMinMax;
use crate::input::Input;

#[derive(Clone, Copy, Debug)]
pub struct Cmd {
    pub p: usize,
    pub r: bool,
    pub d: char,
    pub b: i32,
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

#[derive(Clone)]
pub struct State {
    pub turn: usize,
    /// (x1, x2, y1, y2, rot, t)
    pub pos: Vec<Pos>,
    pub W2: i32,
    pub H2: i32,
    pub length: i32,
    pub score: i32,
}

impl State {
    pub fn new(input: &Input) -> Self {
        let score = input.wh2.iter().map(|&(w, h)| w + h).sum::<i32>();
        Self {
            turn: 0,
            pos: vec![P0; input.N],
            W2: 0,
            H2: 0,
            length: 0,
            score,
        }
    }
    pub fn calc_length(&self, input: &Input, cmd: &[Cmd]) -> i32 {
        let mut pos = vec![P0; input.N];
        let mut W2 = 0;
        let mut H2 = 0;
        let mut prev = -1;
        for (t, c) in cmd.iter().enumerate() {
            if !prev.setmax(c.p as i32) {
                panic!("p must be in ascending order.");
            }
            if pos[c.p].t >= 0 {
                panic!("Rectangle {} is already used", c.p);
            }
            if c.b >= 0 && pos[c.b as usize].t < 0 {
                panic!("Rectangle {} is not used", c.b);
            }
            let (mut w, mut h) = input.wh2[c.p];
            if c.r {
                std::mem::swap(&mut w, &mut h);
            }
            if c.d == 'U' {
                let x1 = if c.b < 0 { 0 } else { pos[c.b as usize].x2 };
                let x2 = x1 + w;
                let y1 = pos
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
                pos[c.p] = Pos {
                    x1,
                    x2,
                    y1,
                    y2,
                    r: c.r,
                    t: t as i32,
                };
            } else {
                let y1 = if c.b < 0 { 0 } else { pos[c.b as usize].y2 };
                let y2 = y1 + h;
                let x1 = pos
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
                pos[c.p] = Pos {
                    x1,
                    x2,
                    y1,
                    y2,
                    r: c.r,
                    t: t as i32,
                };
            }
            W2.setmax(pos[c.p].x2);
            H2.setmax(pos[c.p].y2);
        }
        W2 + H2
    }
    pub fn query(&mut self, input: &Input, cmd: &[Cmd]) -> Result<(), String> {
        self.pos.fill(P0);
        self.W2 = 0;
        self.H2 = 0;
        let mut prev = -1;
        for (t, c) in cmd.iter().enumerate() {
            if !prev.setmax(c.p as i32) {
                return Err(format!("p must be in ascending order."));
            }
            if self.pos[c.p].t >= 0 {
                return Err(format!("Rectangle {} is already used", c.p));
            }
            if c.b >= 0 && self.pos[c.b as usize].t < 0 {
                return Err(format!("Rectangle {} is not used", c.b));
            }
            let (mut w, mut h) = input.wh2[c.p];
            if c.r {
                std::mem::swap(&mut w, &mut h);
            }
            if c.d == 'U' {
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
                self.pos[c.p] = Pos {
                    x1,
                    x2,
                    y1,
                    y2,
                    r: c.r,
                    t: t as i32,
                };
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
                self.pos[c.p] = Pos {
                    x1,
                    x2,
                    y1,
                    y2,
                    r: c.r,
                    t: t as i32,
                };
            }
            self.W2.setmax(self.pos[c.p].x2);
            self.H2.setmax(self.pos[c.p].y2);
        }
        self.length = self.W2 + self.H2;
        self.score = self.length;
        for i in 0..input.N {
            if self.pos[i].t < 0 {
                self.score += input.wh2[i].0 + input.wh2[i].1;
            }
        }
        self.turn += 1;
        Ok(())
    }
}
