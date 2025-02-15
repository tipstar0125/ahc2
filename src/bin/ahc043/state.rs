use std::cmp::Reverse;

use crate::{
    bfs,
    coord::{calc_manhattan_dist, Coord, ADJ},
    dsu::UnionFind,
    input::Input,
};

const STATION_COST: i64 = 5000;
const RAIL_COST: i64 = 100;

#[derive(Debug, Clone, Copy)]
pub enum RailType {
    LeftToRight = 1,
    UpToDown = 2,
    LeftToDown = 3,
    LeftToUp = 4,
    UpToRight = 5,
    DownToRight = 6,
}

#[derive(Debug, Clone, Copy)]
pub enum Entity {
    Station,
    Rail(RailType),
    Empty,
}

#[derive(Debug, Clone)]
pub struct State {
    turn: usize,
    N: usize,
    M: usize,
    T: usize,
    money: i64,
    money_per_day: i64,
    uf: UnionFind,
    root: usize,
    station_postion: Vec<Coord>,
    field: Vec<Vec<Entity>>,
    actions: Vec<String>,
}

impl State {
    pub fn new(input: &Input) -> Self {
        Self {
            turn: 0,
            N: input.N,
            M: input.M,
            T: input.T,
            money: input.K as i64,
            money_per_day: 0,
            uf: UnionFind::new(input.M * 2),
            root: !0,
            station_postion: vec![],
            field: vec![vec![Entity::Empty; input.N]; input.N],
            actions: vec![],
        }
    }
    pub fn get_new_nodes(&mut self, pos: Coord, input: &Input) -> Vec<usize> {
        let mut home_workspace = vec![];
        for &dij in ADJ.iter() {
            let next = pos + dij;
            if next.in_map(self.N) {
                for &idx in input.home_workspace_field[next.i][next.j].iter() {
                    if self.root == !0 || !self.uf.is_same(self.root, idx) {
                        home_workspace.push(idx);
                    }
                }
            }
        }
        home_workspace
    }
    pub fn make_station(&mut self, pos: Coord, input: &Input) -> bool {
        let home_workspace = self.get_new_nodes(pos, input);
        // 周辺に自宅や職場がない場合は駅を置いても意味がない
        if home_workspace.is_empty() {
            return false;
        }

        // 根が未定義の場合は、0番目を根として設定
        if self.root == !0 {
            for &idx in home_workspace.iter().skip(1) {
                self.uf.unite(home_workspace[0], idx);
            }
            self.root = self.uf.find(home_workspace[0]);
        } else {
            for &idx in home_workspace.iter() {
                self.uf.unite(self.root, idx);
            }
        }

        self.money -= STATION_COST;
        self.field[pos.i][pos.j] = Entity::Station;
        self.station_postion.push(pos);
        self.actions.push(format!("0 {} {}", pos.i, pos.j));
        self.turn += 1;
        true
    }
    pub fn make_rail(&mut self, pos: Coord, t: RailType) {
        self.money -= RAIL_COST;
        self.field[pos.i][pos.j] = Entity::Rail(t);
        self.actions
            .push(format!("{} {} {}", t as i64, pos.i, pos.j,));
        self.turn += 1;
    }
    pub fn wait(&mut self) {
        self.actions.push("-1".to_string());
        self.turn += 1;
    }
    pub fn is_empty(&self, pos: Coord) -> bool {
        matches!(self.field[pos.i][pos.j], Entity::Empty)
    }
    pub fn is_done(&self) -> bool {
        self.turn >= self.T
    }
    pub fn greedy(&mut self, start_station: Coord, input: &Input) -> i64 {
        // 周辺に自宅や職場がない場合はスキップ
        if !self.make_station(start_station, input) {
            return 0;
        }

        let mut bfs_cnt = 0;
        let mut visited = vec![vec![0; self.N]; self.N];

        // 既に駅や線路がある場所は訪れないようにする
        for i in 0..self.N {
            for j in 0..self.N {
                if !self.is_empty(Coord::new(i, j)) {
                    visited[i][j] = !0;
                }
            }
        }

        while !self.is_done() {
            let mut cand = vec![];
            let station_position = self.station_postion.clone();
            for &station in station_position.iter() {
                let mut bfs = bfs::BfsGenerator::new(station, &mut bfs_cnt, &mut visited);
                while let Some((next, dist)) = bfs.next(bfs_cnt, &mut visited) {
                    // スタート駅はスキップ
                    if dist == 0 {
                        continue;
                    }
                    // 設置期間が最大ターン数を超える場合は設置しない
                    if self.turn + dist > self.T {
                        break;
                    }
                    let period = dist as i64; // 線路と駅の設置にかかる期間
                    let cost = (period - 1) * RAIL_COST + STATION_COST;
                    // 資金が足りない場合は設置しない
                    if cost > self.money {
                        break;
                    }
                    let new_nodes = self.get_new_nodes(next, input);
                    let added_money_per_day = self.calc_added_money_per_day(&new_nodes, input);
                    let money = self.calc_future_money(added_money_per_day, cost, period);
                    // 資金が増える場合のみ候補に追加
                    if money > self.money {
                        cand.push((money, new_nodes.len(), Reverse(period), next));
                    }
                }
            }
            // 資金がない、もしくは設置期間が足りない、資金が増えない場合は待機
            if cand.is_empty() {
                self.wait();
                continue;
            }
            cand.sort();
            cand.reverse();
            eprintln!("best: {:?}", cand);

            break;
        }

        self.money
    }
    pub fn calc_added_money_per_day(&mut self, new_nodes: &Vec<usize>, input: &Input) -> i64 {
        let mut ret = 0;
        for &new in new_nodes.iter() {
            let pair_node = if new < self.M {
                new + self.M
            } else {
                new - self.M
            };
            if self.uf.is_same(self.root, pair_node) {
                let new_coord = if new < self.M {
                    input.home[new]
                } else {
                    input.workspace[new - self.M]
                };
                let pair_coord = if pair_node < self.M {
                    input.home[pair_node]
                } else {
                    input.workspace[pair_node - self.M]
                };
                ret += calc_manhattan_dist(new_coord, pair_coord) as i64;
            }
        }
        ret
    }
    pub fn calc_future_money(&self, added_money_per_day: i64, cost: i64, period: i64) -> i64 {
        self.money - cost
            + (self.T as i64 - self.turn as i64) * self.money_per_day
            + (self.T as i64 - self.turn as i64 - period) * added_money_per_day
    }
    pub fn output(&self) {
        eprintln!("Score = {}", self.money);
        let mut actions = self.actions.clone();
        while actions.len() < self.T {
            actions.push("-1".to_string());
        }
        for action in actions.iter() {
            println!("{}", action);
        }
    }
}
