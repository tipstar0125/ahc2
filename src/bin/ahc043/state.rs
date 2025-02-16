use std::cmp::Reverse;

use crate::{
    bfs::{bfs_revert, A_star, BfsGenerator, CANNOT_VISIT, NOT_VISITED},
    coord::{calc_manhattan_dist, Coord, ADJ, NEG},
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
    best_money: i64,
    income: i64,
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
            best_money: input.K as i64,
            income: 0,
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
    pub fn make_station(&mut self, pos: Coord, node_num: usize, input: &Input) -> bool {
        let home_workspace = self.get_new_nodes(pos, input);
        // 周辺に自宅や職場が少なく、計算量の都合でスキップしたい場合
        if home_workspace.len() < node_num {
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
        assert!(self.money >= 0);
        self.money += self.income;
        self.field[pos.i][pos.j] = Entity::Station;
        self.station_postion.push(pos);
        self.actions.push(format!("0 {} {}", pos.i, pos.j));
        self.turn += 1;
        // eprintln!(
        //     "{} Score = {}, income = {}",
        //     self.turn, self.money, self.income
        // );
        true
    }
    pub fn make_rail(&mut self, pos: Coord, t: RailType) {
        self.money -= RAIL_COST;
        assert!(self.money >= 0);
        self.money += self.income;
        self.field[pos.i][pos.j] = Entity::Rail(t);
        self.actions
            .push(format!("{} {} {}", t as i64, pos.i, pos.j,));
        self.turn += 1;
        // eprintln!(
        //     "{} Score = {}, income = {}",
        //     self.turn, self.money, self.income
        // );
    }
    pub fn wait(&mut self) {
        self.actions.push("-1".to_string());
        self.money += self.income;
        self.turn += 1;
        // eprintln!(
        //     "{} Score = {}, income = {}",
        //     self.turn, self.money, self.income
        // );
    }
    pub fn make_path(&mut self, start: Coord, goal: Coord, input: &Input) {
        let new_nodes = self.get_new_nodes(goal, input);
        let mut dist = vec![vec![NOT_VISITED; self.N]; self.N];
        for i in 0..self.N {
            for j in 0..self.N {
                if !self.is_empty(Coord::new(i, j)) {
                    dist[i][j] = CANNOT_VISIT;
                }
            }
        }
        A_star(start, goal, &mut dist);
        let route = bfs_revert(start, goal, &dist);

        for i in 1..route.len() - 1 {
            let prev = route[i - 1];
            let next = route[i + 1];
            let now = route[i];
            let prev_dij = now - prev;
            let next_dij = next - now;

            let t = match (prev_dij, next_dij) {
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
            };
            self.make_rail(now, t);
        }
        let added_income = self.calc_added_income(&new_nodes, input);
        self.income += added_income;
        self.make_station(*route.last().unwrap(), 1, input);
        self.best_money = (self.T as i64 - self.turn as i64) * self.income + self.money;
    }
    pub fn is_empty(&self, pos: Coord) -> bool {
        matches!(self.field[pos.i][pos.j], Entity::Empty)
    }
    pub fn is_done(&self) -> bool {
        self.turn >= self.T
    }
    pub fn greedy(&mut self, start_station: Coord, input: &Input) -> i64 {
        // 計算量の都合で、周辺に自宅や職場が少ない場合はスキップ
        if !self.make_station(start_station, 2, input) {
            return 0;
        }

        let mut bfs_cnt = 0;
        let mut visited = vec![vec![0; self.N]; self.N];

        while !self.is_done() {
            let mut cand = vec![];
            let station_position = self.station_postion.clone();
            // 既に駅や線路がある場所は訪れないようにする
            for i in 0..self.N {
                for j in 0..self.N {
                    if !self.is_empty(Coord::new(i, j)) {
                        visited[i][j] = CANNOT_VISIT;
                    }
                }
            }
            for &station in station_position.iter() {
                let mut bfs = BfsGenerator::new(station, &mut bfs_cnt, &mut visited);
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
                    // 新規の自宅や職場がない場合はスキップ
                    if new_nodes.is_empty() {
                        continue;
                    }
                    let added_income = self.calc_added_income(&new_nodes, input);
                    let money = self.calc_future_money(added_income, cost, period);
                    // 資金が増える場合のみ候補に追加
                    if money > self.best_money {
                        cand.push((money, new_nodes.len(), Reverse(period), station, next));
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
            let (_, _, _, station, next) = cand[0];
            self.make_path(station, next, input);

            // 駅が設置できるる資金になるまで待機
            while !self.is_done() && self.money < STATION_COST + RAIL_COST * 20 {
                self.wait();
            }
        }

        self.money
    }
    pub fn calc_added_income(&mut self, new_nodes: &Vec<usize>, input: &Input) -> i64 {
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
    pub fn calc_future_money(&self, added_income: i64, cost: i64, period: i64) -> i64 {
        self.money - cost
            + (self.T as i64 - self.turn as i64) * self.income
            + (self.T as i64 - self.turn as i64 - period + 1) * added_income
    }
    pub fn output(&mut self) {
        while self.actions.len() < self.T {
            self.wait();
        }
        eprintln!("Score = {}", self.money);
        for action in self.actions.iter() {
            println!("{}", action);
        }
    }
}
