use crate::{
    common::get_time,
    coord::{calc_manhattan_dist, Coord, ADJ, NEG},
    input::Input,
};

const INF: i64 = 1 << 30;
const STATION_COST: i64 = 5000;
const RAIL_COST: i64 = 100;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RailType {
    LeftToRight = 1,
    UpToDown = 2,
    LeftToDown = 3,
    LeftToUp = 4,
    UpToRight = 5,
    DownToRight = 6,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Entity {
    Station,
    Rail(RailType),
    Empty, // Opではwaitとして扱う
}

pub fn to_rail_type(prev: Coord, now: Coord, next: Coord) -> RailType {
    let prev_dij = now - prev;
    let next_dij = next - now;
    match (prev_dij, next_dij) {
        (Coord { i: 1, j: 0 }, Coord { i: 1, j: 0 }) => RailType::UpToDown,
        (Coord { i: NEG, j: 0 }, Coord { i: NEG, j: 0 }) => RailType::UpToDown,
        (Coord { i: 0, j: 1 }, Coord { i: 0, j: 1 }) => RailType::LeftToRight,
        (Coord { i: 0, j: NEG }, Coord { i: 0, j: NEG }) => RailType::LeftToRight,
        (Coord { i: 1, j: 0 }, Coord { i: 0, j: 1 }) => RailType::UpToRight,
        (Coord { i: 1, j: 0 }, Coord { i: 0, j: NEG }) => RailType::LeftToUp,
        (Coord { i: NEG, j: 0 }, Coord { i: 0, j: 1 }) => RailType::DownToRight,
        (Coord { i: NEG, j: 0 }, Coord { i: 0, j: NEG }) => RailType::LeftToDown,
        (Coord { i: 0, j: 1 }, Coord { i: 1, j: 0 }) => RailType::LeftToDown,
        (Coord { i: 0, j: 1 }, Coord { i: NEG, j: 0 }) => RailType::LeftToUp,
        (Coord { i: 0, j: NEG }, Coord { i: 1, j: 0 }) => RailType::DownToRight,
        (Coord { i: 0, j: NEG }, Coord { i: NEG, j: 0 }) => RailType::UpToRight,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
pub struct Op {
    pub t: Entity, // Entity::Emptyの場合はwait
    pub pos: Coord,
}

impl Op {
    pub fn to_i8(&self) -> i8 {
        match self.t {
            Entity::Station => 0,
            Entity::Rail(RailType::LeftToRight) => 1,
            Entity::Rail(RailType::UpToDown) => 2,
            Entity::Rail(RailType::LeftToDown) => 3,
            Entity::Rail(RailType::LeftToUp) => 4,
            Entity::Rail(RailType::UpToRight) => 5,
            Entity::Rail(RailType::DownToRight) => 6,
            Entity::Empty => -1,
        }
    }
    pub fn output(&self) {
        let t = self.to_i8();
        if t == -1 {
            println!("-1");
        } else {
            println!("{} {} {}", t, self.pos.i, self.pos.j);
        }
    }
}

pub fn calc_wait_num(dist: usize, money: i64, income: i64) -> usize {
    let cost = STATION_COST + RAIL_COST * (dist as i64 - 1);
    if cost <= money {
        0
    } else {
        ((cost - money + income - 1) / income) as usize
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub turn: usize,
    pub money: i64,
    pub income: i64,
    pub score: i64,
    pub connected: Vec<bool>,
    pub field: Vec<Vec<Entity>>,
    pub ops: Vec<Op>,
}

impl State {
    pub fn new(input: &Input) -> Self {
        Self {
            turn: 0,
            money: input.K as i64,
            income: 0,
            score: 0,
            connected: vec![false; input.M * 2],
            field: vec![vec![Entity::Empty; input.N]; input.N],
            ops: vec![],
        }
    }
    pub fn make_route(&self, from: &Coord, to: &Coord, input: &Input) -> Vec<Coord> {
        // 線路上に駅を立てる場合
        if from == &Coord::new(!0, !0) {
            return vec![*from, *to];
        }

        let h = (from.i as i64 - to.i as i64).abs() as usize;
        let w = (from.j as i64 - to.j as i64).abs() as usize;

        // 各マスに対してスコアを計算
        let mut S = vec![vec![0; w + 1]; h + 1];
        let left = if from.j < to.j { from.j } else { to.j };
        let top = if from.i < to.i { from.i } else { to.i };
        for i in 0..=h {
            for j in 0..=w {
                let mut score = 0;
                let coord = Coord::new(top + i, left + j);
                // すでに線路や駅がある場合はスキップ
                if self.field[coord.i][coord.j] != Entity::Empty {
                    S[i][j] = -INF;
                    continue;
                }
                for &dij in ADJ.iter() {
                    let nxt = coord + dij;
                    if nxt.in_map(input.N) {
                        for &new_idx in input.cover_field[nxt.i][nxt.j].iter() {
                            // すでに繋がっている場合はスコアに影響しない
                            if self.connected[new_idx] {
                                continue;
                            }
                            let pair_idx = if new_idx < input.M {
                                new_idx + input.M
                            } else {
                                new_idx - input.M
                            };

                            // それぞれの座標を取得
                            let new_coord = if new_idx < input.M {
                                input.workspace[new_idx]
                            } else {
                                input.home[new_idx - input.M]
                            };
                            let pair_coord = if pair_idx < input.M {
                                input.workspace[pair_idx]
                            } else {
                                input.home[pair_idx - input.M]
                            };

                            // ペアが既に繋がっている場合は即時スコアに影響するので、繋がっていない場合のスコアを5倍にする
                            if self.connected[pair_idx] {
                                score += 5 * calc_manhattan_dist(&new_coord, &pair_coord) as i64;
                            } else {
                                score += calc_manhattan_dist(&new_coord, &pair_coord) as i64;
                            }
                        }
                    }
                }
                S[i][j] = score;
            }
        }

        let from_i = from.i - top;
        let from_j = from.j - left;
        let mut i = from_i;
        let mut j = from_j;
        let dir_i = if from.i < to.i { 1 } else { NEG };
        let dir_j = if from.j < to.j { 1 } else { NEG };

        let mut dp = vec![vec![-INF; w + 1]; h + 1];
        dp[i][j] = 0;
        while i <= h {
            while j <= w {
                if i - dir_i <= h {
                    dp[i][j] = dp[i][j].max(dp[i - dir_i][j] + S[i][j]);
                }
                if j - dir_j <= w {
                    dp[i][j] = dp[i][j].max(dp[i][j - dir_j] + S[i][j]);
                }
                j += dir_j;
            }
            j = from_j;
            i += dir_i;
        }

        // 経路復元
        let to_i = to.i - top;
        let to_j = to.j - left;
        if dp[to_i][to_j] <= 0 {
            return vec![];
        }

        let mut i = to_i;
        let mut j = to_j;
        let mut route = vec![Coord::new(top + i, left + j)];
        while i != from_i || j != from_j {
            if i - dir_i <= h && dp[i - dir_i][j] + S[i][j] == dp[i][j] {
                i -= dir_i;
            } else if j - dir_j <= w && dp[i][j - dir_j] + S[i][j] == dp[i][j] {
                j -= dir_j;
            } else {
                unreachable!();
            }
            route.push(Coord::new(top + i, left + j));
        }
        route.reverse();
        route
    }
    pub fn to_initial_ops(&self, route: &Vec<Coord>) -> Vec<Op> {
        assert!(route.len() >= 2);
        let mut ops = vec![];
        ops.push(Op {
            t: Entity::Station,
            pos: route[0],
        });

        for i in 1..route.len() - 1 {
            let prev = route[i - 1];
            let now = route[i];
            let next = route[i + 1];
            ops.push(Op {
                t: Entity::Rail(to_rail_type(prev, now, next)),
                pos: route[i],
            });
        }

        ops.push(Op {
            t: Entity::Station,
            pos: route[route.len() - 1],
        });

        ops
    }
    pub fn to_ops(&self, route: &Vec<Coord>) -> Vec<Op> {
        let from = route[0];
        let to = route[route.len() - 1];
        let dist = if from == Coord::new(!0, !0) {
            1 // 線路上に駅を建てる場合
        } else {
            calc_manhattan_dist(&from, &to)
        };
        let wait_num = calc_wait_num(dist, self.money, self.income);
        let mut ops = vec![
            Op {
                t: Entity::Empty,
                pos: Coord::new(!0, !0)
            };
            wait_num
        ];

        for i in 1..route.len() - 1 {
            let prev = route[i - 1];
            let now = route[i];
            let next = route[i + 1];
            ops.push(Op {
                t: Entity::Rail(to_rail_type(prev, now, next)),
                pos: route[i],
            });
        }

        ops.push(Op {
            t: Entity::Station,
            pos: to,
        });

        ops
    }
    pub fn cand(&self, input: &Input) -> Vec<Vec<Op>> {
        let mut cands = vec![];
        if self.turn == 0 {
            const MAX_INITIAL_CAND_NUM: usize = 2000;
            let L = input.covers.len();
            let mut rough_cands = vec![];
            for i in 0..L {
                let (from, cover0) = &input.covers[i];
                for j in i + 1..L {
                    let (to, cover1) = &input.covers[j];

                    // 資金が足りない場合はスキップ
                    let dist = calc_manhattan_dist(from, to) as i64;
                    if STATION_COST * 2 + RAIL_COST * (dist - 1) > self.money {
                        continue;
                    }

                    let mut income = 0;
                    for mut c0 in cover0 {
                        for mut c1 in cover1 {
                            if c0 > c1 {
                                std::mem::swap(&mut c0, &mut c1);
                            }
                            // 対応する自宅と会社がある場合
                            if *c0 + input.M == *c1 {
                                let pos_home = input.home[*c0];
                                let pos_workspace = input.workspace[*c1 - input.M];
                                income += calc_manhattan_dist(&pos_home, &pos_workspace) as i64;
                            }
                        }
                    }
                    if income > 0 {
                        // let income_per_turn = income as f64 / dist as f64;
                        // rough_cands.push((income_per_turn, from, to));
                        rough_cands.push((income, from, to));
                    }
                }
            }
            rough_cands.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            rough_cands.truncate(MAX_INITIAL_CAND_NUM);

            for (_, from, to) in rough_cands {
                let route = self.make_route(from, to, input);
                if !route.is_empty() {
                    cands.push(self.to_initial_ops(&route));
                }
            }
        } else {
            const MAX_CAND_NUM: usize = 50;
            let L = input.covers.len();
            let mut rough_cands = vec![];

            // 線路上に駅を建てる
            for i in 0..input.N {
                for j in 0..input.N {
                    match self.field[i][j] {
                        Entity::Rail(_) => {
                            let to = Coord::new(i, j);
                            let cover = &input.cover_field[i][j];
                            // 資金が足りない場合は待機
                            let dist = 1;
                            let wait_num = calc_wait_num(dist, self.money, self.income);
                            // 最大ターン数を超える場合はスキップ
                            if self.turn + wait_num + dist > input.T {
                                continue;
                            }
                            let mut income = 0;
                            for &idx in cover {
                                let pair_idx = if idx < input.M {
                                    idx + input.M
                                } else {
                                    idx - input.M
                                };
                                if !self.connected[idx] && self.connected[pair_idx] {
                                    let coord = if idx < input.M {
                                        input.home[idx]
                                    } else {
                                        input.workspace[idx - input.M]
                                    };
                                    let pair_coord = if pair_idx < input.M {
                                        input.home[pair_idx]
                                    } else {
                                        input.workspace[pair_idx - input.M]
                                    };
                                    income += calc_manhattan_dist(&coord, &pair_coord) as i64;
                                }
                            }
                            if income > 0 {
                                let income_per_turn = income as f64 / (wait_num + dist) as f64;
                                rough_cands.push((income_per_turn, Coord::new(!0, !0), to));
                            }
                        }
                        _ => {}
                    }
                }
            }

            // 線路を敷いて駅を建てる
            'a: for i in 0..input.N {
                for j in 0..input.N {
                    if self.field[i][j] != Entity::Station {
                        continue;
                    }
                    let from = Coord::new(i, j);
                    for k in 0..L {
                        let (to, cover) = &input.covers[k];

                        // 資金が足りない場合は待機
                        let dist = calc_manhattan_dist(&from, to);
                        let wait_num = calc_wait_num(dist, self.money, self.income);
                        // 最大ターン数を超える場合はスキップ
                        if self.turn + wait_num + dist > input.T {
                            continue;
                        }

                        let mut income = 0;
                        for &idx in cover {
                            let pair_idx = if idx < input.M {
                                idx + input.M
                            } else {
                                idx - input.M
                            };
                            if !self.connected[idx] && self.connected[pair_idx] {
                                let coord = if idx < input.M {
                                    input.home[idx]
                                } else {
                                    input.workspace[idx - input.M]
                                };
                                let pair_coord = if pair_idx < input.M {
                                    input.home[pair_idx]
                                } else {
                                    input.workspace[pair_idx - input.M]
                                };
                                income += calc_manhattan_dist(&coord, &pair_coord) as i64;
                            }
                        }
                        if income > 0 {
                            let income_per_turn = income as f64 / (wait_num + dist) as f64;
                            rough_cands.push((income_per_turn, from, *to));
                        }
                        if get_time() > input.TLE {
                            break 'a;
                        }
                    }
                }
            }
            rough_cands.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            rough_cands.truncate(MAX_CAND_NUM);

            for (_, from, to) in rough_cands {
                let route = self.make_route(&from, &to, input);
                if !route.is_empty() {
                    cands.push(self.to_ops(&route));
                }
            }
        }
        cands
    }
    pub fn apply(&self, ops: Vec<Op>, input: &Input) -> Self {
        let mut new_state = self.clone();
        for op in ops.iter() {
            // 建設フェーズ
            match op.t {
                Entity::Station => {
                    new_state.field[op.pos.i][op.pos.j] = op.t;
                    new_state.money -= STATION_COST;
                    for &idx in input.cover_field[op.pos.i][op.pos.j].iter() {
                        let pair_idx = if idx < input.M {
                            idx + input.M
                        } else {
                            idx - input.M
                        };
                        if !new_state.connected[idx] && new_state.connected[pair_idx] {
                            let coord = if idx < input.M {
                                input.home[idx]
                            } else {
                                input.workspace[idx - input.M]
                            };
                            let pair_coord = if pair_idx < input.M {
                                input.home[pair_idx]
                            } else {
                                input.workspace[pair_idx - input.M]
                            };
                            new_state.income += calc_manhattan_dist(&coord, &pair_coord) as i64;
                        }
                        new_state.connected[idx] = true;
                    }
                }
                Entity::Rail(_) => {
                    new_state.field[op.pos.i][op.pos.j] = op.t;
                    new_state.money -= RAIL_COST;
                }
                Entity::Empty => {
                    // wait
                }
            }
            assert!(new_state.money >= 0);

            // 集金フェーズ
            new_state.money += new_state.income;
            new_state.turn += 1;
        }

        new_state.ops.extend(ops);
        new_state.score =
            new_state.money + (input.T as i64 - new_state.turn as i64) * new_state.income;

        new_state
    }
}
