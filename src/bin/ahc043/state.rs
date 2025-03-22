use crate::{
    common::get_time,
    coord::{calc_manhattan_dist, Coord, NEG},
    input::Input,
};

const INF: i64 = 1 << 30;
const STATION_COST: i64 = 5000;
const RAIL_COST: i64 = 100;
const EMPTY: i8 = -1;
const WAIT: i8 = -1;
const STATION: i8 = 0;

pub fn to_rail_type(prev: Coord, now: Coord, next: Coord) -> i8 {
    let prev_dij = now - prev;
    let next_dij = next - now;
    // 1: ─
    // 2: │
    // 3: ┐
    // 4: ┘
    // 5: └
    // 6: ┌
    match (prev_dij, next_dij) {
        (Coord { i: 1, j: 0 }, Coord { i: 1, j: 0 }) => 2,
        (Coord { i: NEG, j: 0 }, Coord { i: NEG, j: 0 }) => 2,
        (Coord { i: 0, j: 1 }, Coord { i: 0, j: 1 }) => 1,
        (Coord { i: 0, j: NEG }, Coord { i: 0, j: NEG }) => 1,
        (Coord { i: 1, j: 0 }, Coord { i: 0, j: 1 }) => 5,
        (Coord { i: 1, j: 0 }, Coord { i: 0, j: NEG }) => 4,
        (Coord { i: NEG, j: 0 }, Coord { i: 0, j: 1 }) => 6,
        (Coord { i: NEG, j: 0 }, Coord { i: 0, j: NEG }) => 3,
        (Coord { i: 0, j: 1 }, Coord { i: 1, j: 0 }) => 3,
        (Coord { i: 0, j: 1 }, Coord { i: NEG, j: 0 }) => 4,
        (Coord { i: 0, j: NEG }, Coord { i: 1, j: 0 }) => 6,
        (Coord { i: 0, j: NEG }, Coord { i: NEG, j: 0 }) => 5,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
pub struct Op {
    pub t: i8,
    pub pos: Coord,
}

impl Op {
    pub fn output(&self) {
        if self.t == -1 {
            println!("-1");
        } else {
            println!("{} {} {}", self.t, self.pos.i, self.pos.j);
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
    pub field: Vec<Vec<i8>>,
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
            field: vec![vec![EMPTY; input.N]; input.N],
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
                S[i][j] = if self.field[i + top][j + left] != EMPTY {
                    -INF
                } else {
                    input.cover_field[i + top][j + left].1.len() as i64
                };
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
            t: STATION,
            pos: route[0],
        });

        for i in 1..route.len() - 1 {
            let prev = route[i - 1];
            let now = route[i];
            let next = route[i + 1];
            ops.push(Op {
                t: to_rail_type(prev, now, next),
                pos: route[i],
            });
        }

        ops.push(Op {
            t: STATION,
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
                t: WAIT,
                pos: Coord::new(!0, !0)
            };
            wait_num
        ];

        for i in 1..route.len() - 1 {
            let prev = route[i - 1];
            let now = route[i];
            let next = route[i + 1];
            ops.push(Op {
                t: to_rail_type(prev, now, next),
                pos: route[i],
            });
        }

        ops.push(Op {
            t: STATION,
            pos: to,
        });

        ops
    }
    pub fn cand(&self, input: &Input) -> Vec<Vec<Op>> {
        let mut cands = vec![];
        if self.turn == 0 {
            const MAX_INITIAL_CAND_NUM: usize = 2000;
            const MIN_NODE_NUM: usize = 2;
            let L = input.covers.len();
            let max_dist = (self.money - STATION_COST * 2) / RAIL_COST + 1;
            let mut rough_cands = vec![];
            let mut connected = vec![0; input.M * 2];
            let mut connected_cnt = 0;
            for i in 0..L {
                let (from, cover0) = &input.covers[i];
                if cover0.len() < MIN_NODE_NUM {
                    continue;
                }
                for j in i + 1..L {
                    let (to, cover1) = &input.covers[j];
                    if cover1.len() < MIN_NODE_NUM {
                        continue;
                    }

                    let dist = calc_manhattan_dist(from, to) as i64;
                    // 資金が足りない場合はスキップ
                    if dist > max_dist {
                        continue;
                    }

                    let mut income = 0;
                    connected_cnt += 1;
                    for &idx in cover0 {
                        connected[idx] = connected_cnt;
                    }
                    for &idx in cover1 {
                        let pair_idx = (idx + input.M) % (input.M * 2);
                        let dist = input.pair_dist[idx % input.M];
                        if connected[pair_idx] == connected_cnt {
                            income += dist;
                        }
                    }

                    if income > 0 {
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
            eprintln!("first cand elapsed: {:.3}", get_time());
        } else {
            const MAX_CAND_NUM: usize = 50;
            let mut rough_cands = vec![];

            for (to, cover) in input.covers.iter() {
                if self.field[to.i][to.j] == STATION {
                    continue;
                }
                let mut income = 0;
                for &idx in cover.iter() {
                    if self.connected[idx] {
                        continue;
                    }
                    let pair_idx = (idx + input.M) % (input.M * 2);
                    let dist = input.pair_dist[idx % input.M];
                    if self.connected[pair_idx] {
                        income += 5 * dist;
                    } else {
                        income += dist;
                    }
                }
                if income > 0 {
                    rough_cands.push((income, to));
                }
            }

            rough_cands.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            rough_cands.truncate(MAX_CAND_NUM);

            for (_, to) in rough_cands {
                let from = {
                    if self.field[to.i][to.j] != EMPTY {
                        Coord::new(!0, !0)
                    } else {
                        let mut best_from = Coord::new(!0, !0);
                        let mut best_dist = INF as usize;
                        for i in 0..input.N {
                            for j in 0..input.N {
                                if self.field[i][j] != STATION {
                                    continue;
                                }
                                let from = Coord::new(i, j);
                                let dist = calc_manhattan_dist(&from, &to);
                                if dist < best_dist {
                                    best_dist = dist;
                                    best_from = from;
                                }
                            }
                        }
                        best_from
                    }
                };
                let dist = if from == Coord::new(!0, !0) {
                    1 // 線路上に駅を建てる場合
                } else {
                    calc_manhattan_dist(&from, &to)
                };
                // 資金が足りない場合は待機
                let wait_num = calc_wait_num(dist, self.money, self.income);
                // 最大ターン数を超える場合はスキップ
                if self.turn + wait_num + dist > input.T {
                    continue;
                }

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
                STATION => {
                    new_state.field[op.pos.i][op.pos.j] = op.t;
                    new_state.money -= STATION_COST;
                    for &idx in input.cover_field[op.pos.i][op.pos.j].1.iter() {
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
                // RAIL
                1..=6 => {
                    new_state.field[op.pos.i][op.pos.j] = op.t;
                    new_state.money -= RAIL_COST;
                }
                WAIT => {}
                _ => unreachable!(),
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
