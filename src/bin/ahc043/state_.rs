use std::{cmp::Reverse, collections::BinaryHeap};

use rustc_hash::FxHashSet;

use crate::{
    bfs::{
        bfs_revert, A_star, BfsGenerator, Dijkstra_multi_start, Dijkstra_multi_start_revert,
        CANNOT_VISIT, NOT_VISITED,
    },
    coord::{calc_manhattan_dist, Coord, ADJ, DIJ4, NEG},
    dsu::UnionFind,
    get_dij,
    input::Input,
};

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
    Empty,
}

#[derive(Debug, Clone)]
pub struct Stat {
    turn: usize,
    N: usize,
    M: usize,
    T: usize,
    money: i64,
    best_money: i64,
    income: i64,
    uf: UnionFind,
    root: usize,
    station_position: Vec<Coord>,
    rail_position: Vec<Coord>,
    field: Vec<Vec<Entity>>,
    actions: Vec<String>,
}

impl Stat {
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
            station_position: vec![],
            rail_position: vec![],
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
        self.station_position.push(pos);
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
        self.rail_position.push(pos);
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
    pub fn replace_rail_to_empty(&mut self, pos: Coord, input: &Input) {
        let idx = self.rail_position.iter().position(|&x| x == pos).unwrap();
        self.rail_position.remove(idx);
        let new_nodes = self.get_new_nodes(pos, input);
        let added_income = self.calc_added_income(&new_nodes, input);
        self.income += added_income;
        self.make_station(pos, 1, input);
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
            let mut cand2 = vec![];
            let station_position = self.station_position.clone();
            // 既に駅や線路がある場所は訪れないようにする
            for pos in self
                .station_position
                .iter()
                .chain(self.rail_position.iter())
            {
                visited[pos.i][pos.j] = CANNOT_VISIT;
            }
            for &station in station_position.iter() {
                let mut bfs = BfsGenerator::new(station, &mut bfs_cnt, &mut visited);
                while let Some((next, dist)) = bfs.next(bfs_cnt, &mut visited) {
                    // スタート駅はスキップ
                    if dist == 0 {
                        continue;
                    }
                    if dist > 50 {
                        break;
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
                        let future_added_income = (self.T as i64 - self.turn as i64 - period + 1)
                            * (new_nodes.len() as i64);
                        cand.push((
                            money + future_added_income,
                            new_nodes.len(),
                            Reverse(period),
                            station,
                            next,
                            false,
                        ));
                    } else {
                        cand2.push((Reverse(period), new_nodes.len(), station, next, false));
                    }
                }
            }
            let rail_position = self.rail_position.clone();
            for &next in rail_position.iter() {
                let period = 1;
                let cost = STATION_COST;
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
                    let future_added_income =
                        (self.T as i64 - self.turn as i64) * (new_nodes.len() as i64);
                    cand.push((
                        money + future_added_income,
                        new_nodes.len(),
                        Reverse(period),
                        Coord::new(0, 0),
                        next,
                        true,
                    ));
                } else {
                    cand2.push((
                        Reverse(period),
                        new_nodes.len(),
                        Coord::new(0, 0),
                        next,
                        true,
                    ));
                }
            }

            if !cand.is_empty() {
                cand.sort();
                cand.reverse();
                let (_, _, _, station, next, flag) = cand[0];
                if flag {
                    self.replace_rail_to_empty(next, input);
                } else {
                    self.make_path(station, next, input);
                }
            } else {
                if self.turn < 500 && !cand2.is_empty() {
                    cand2.sort();
                    cand2.reverse();
                    let (_, _, station, next, flag) = cand2[0];
                    if flag {
                        self.replace_rail_to_empty(next, input);
                    } else {
                        self.make_path(station, next, input);
                    }
                } else {
                    while !self.is_done() {
                        self.wait();
                    }
                }
            }

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

#[derive(Debug, Clone)]
pub struct Op {
    pub from: Option<(Coord, usize)>,
    pub to: (Coord, usize),
    pub period: usize,
    pub route: Vec<usize>,
    pub is_wait: bool,
    pub score: i64,
}

#[derive(Debug, Clone)]
pub struct State {
    pub turn: usize,
    pub income: i64,
    pub used: Vec<usize>, // 0: empty, 1: rail, 2: station
    pub connected: Vec<bool>,
    pub money: i64,
    pub hash: usize,
}

impl State {
    pub fn new(input: &Input, rail_tree: &RailTree) -> Self {
        Self {
            turn: 0,
            income: 0,
            used: vec![0; rail_tree.station_position.len()],
            connected: vec![false; input.M * 2],
            money: input.K as i64,
            hash: 0,
        }
    }
    pub fn get_new_nodes(&self, pos: Coord, input: &Input) -> Vec<usize> {
        let mut nodes = vec![];
        for &dij in ADJ.iter() {
            let next = pos + dij;
            if next.in_map(input.N) {
                for &idx in input.home_workspace_field[next.i][next.j].iter() {
                    if !self.connected[idx] {
                        nodes.push(idx);
                    }
                }
            }
        }
        nodes
    }
    pub fn calc_added_income(&self, nodes: &Vec<usize>, input: &Input) -> i64 {
        let mut income = 0;
        for &node in nodes.iter() {
            let pair_node = if node < input.M {
                node + input.M
            } else {
                node - input.M
            };
            if self.connected[pair_node] {
                let new_coord = if node < input.M {
                    input.home[node]
                } else {
                    input.workspace[node - input.M]
                };
                let pair_coord = if pair_node < input.M {
                    input.home[pair_node]
                } else {
                    input.workspace[pair_node - input.M]
                };
                income += calc_manhattan_dist(new_coord, pair_coord) as i64;
            }
        }
        income
    }
    // return: score, hash, op, is_done
    pub fn cand(&self, input: &Input, rail_tree: &RailTree) -> Vec<(i64, usize, Op, bool)> {
        let mut cand = vec![];

        if self.turn == input.T {
            return cand;
        }

        if self.turn == 0 {
            for (idx, &to) in rail_tree.station_position.iter().enumerate() {
                let period = 1;
                let mut score = self.money - STATION_COST;
                let op = Op {
                    from: None,
                    to: (to, idx),
                    period,
                    route: vec![],
                    score,
                    is_wait: false,
                };
                score *= input.T as i64;
                score /= self.turn as i64 + period as i64;
                let hash = input.calc_hash.calc(self.hash, to);
                cand.push((score, hash, op, false));
            }
        } else if self.money >= STATION_COST {
            let L = rail_tree.station_position.len();
            let mut dist = vec![NOT_VISITED; L];
            let mut from_station = vec![];
            let mut ng_station = FxHashSet::default();
            for idx in 0..L {
                // 1: rail, 線路に駅を設置
                if self.used[idx] == 1 {
                    dist[idx] = CANNOT_VISIT; // 線路を伸ばして駅を設置するときに通らないように設定しておく
                    ng_station.insert(idx);
                    if self.money < STATION_COST {
                        continue;
                    }
                    let to = rail_tree.station_position[idx];
                    let period = 1;
                    let hash = input.calc_hash.calc(self.hash, to);
                    let is_done = self.turn + period == input.T;
                    let new_nodes = self.get_new_nodes(to, input);
                    let added_income = self.calc_added_income(&new_nodes, input);
                    let mut score = self.money - STATION_COST
                        + (input.T - self.turn) as i64 * (self.income + added_income);
                    let op = Op {
                        from: None,
                        to: (to, idx),
                        period,
                        route: vec![],
                        score,
                        is_wait: false,
                    };
                    score *= input.T as i64;
                    score /= self.turn as i64 + period as i64;
                    cand.push((score, hash, op, is_done));
                }
                // 2: station, 線路を伸ばして駅を設置
                // 駅を置く予定の箇所に線路を伸ばしても可
                if self.used[idx] == 2 {
                    from_station.push(idx); // 多始点スタート
                }
            }
            Dijkstra_multi_start(from_station, &mut dist, &rail_tree.G);
            let routes = Dijkstra_multi_start_revert(&dist, &rail_tree.G);
            for (from_idx, to_idx, period, route) in routes {
                if self.turn + period > input.T {
                    continue;
                }
                if self.money < STATION_COST + (period as i64 - 1) * RAIL_COST {
                    continue;
                }
                if ng_station.contains(&to_idx) {
                    continue;
                }
                let from = rail_tree.station_position[from_idx];
                let to = rail_tree.station_position[to_idx];
                let hash = input.calc_hash.calc(self.hash, to);
                let is_done = self.turn + period == input.T;
                let new_nodes = self.get_new_nodes(to, input);
                let added_income = self.calc_added_income(&new_nodes, input);
                let mut score = self.money - STATION_COST - (period as i64 - 1) * RAIL_COST
                    + (input.T - self.turn) as i64 * self.income
                    + (input.T - self.turn - period + 1) as i64 * added_income;
                let op = Op {
                    from: Some((from, from_idx)),
                    to: (to, to_idx),
                    period,
                    route,
                    score,
                    is_wait: false,
                };
                score *= input.T as i64;
                score /= self.turn as i64 + period as i64;
                cand.push((score, hash, op, is_done));
            }
        }
        // wait
        if self.turn > 1 {
            let period = 1;
            let mut score = self.money + (input.T - self.turn) as i64 * self.income;
            let op = Op {
                from: None,
                to: (Coord::new(!0, !0), !0),
                period,
                route: vec![],
                score,
                is_wait: true,
            };
            score *= input.T as i64;
            score /= self.turn as i64 + period as i64;
            cand.push((score, self.hash, op, self.turn + period == input.T));
        }
        cand
    }
    pub fn apply(
        &mut self,
        _score: i64,
        hash: usize,
        op: &Op,
        input: &Input,
        _rail_tree: &RailTree,
    ) {
        if op.is_wait {
            self.money += self.income;
        } else {
            let new_nodes = self.get_new_nodes(op.to.0, input);
            let added_income = self.calc_added_income(&new_nodes, input);
            self.money -= STATION_COST;
            self.money -= (op.period as i64 - 1) * RAIL_COST;
            assert!(self.money >= 0);
            self.money += self.income * op.period as i64;
            self.money += added_income;
            self.income += added_income;
            for node in new_nodes.iter() {
                assert!(self.connected[*node] == false);
                self.connected[*node] = true;
            }
            self.used[op.to.1] = 2;
            for &idx in op.route.iter() {
                assert!(self.used[idx] == 0);
                self.used[idx] = 1;
            }
            if op.from.is_some()
                && op.from.unwrap().0 == Coord::new(22, 43)
                && op.to.0 == Coord::new(8, 9)
            {
                for &idx in op.route.iter() {
                    eprintln!("{}", _rail_tree.station_position[idx]);
                }
            }
        }

        self.turn += op.period;
        self.hash = hash;
    }
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

pub struct RailTree {
    pub station_position: Vec<Coord>,
    pub rail_position: Vec<Coord>,
    pub field: Vec<Vec<Entity>>,
    pub G: Vec<Vec<(usize, usize)>>,
}

impl RailTree {
    pub fn new(input: &Input) -> Self {
        Self {
            station_position: vec![],
            rail_position: vec![],
            field: vec![vec![Entity::Empty; input.N]; input.N],
            G: vec![],
        }
    }
    pub fn greedy_station(&mut self, input: &Input) {
        let mut cnt = 0;
        let mut used_pos = vec![vec![false; input.N]; input.N];
        let mut used_home_workspace = vec![false; input.M * 2];
        while cnt < input.M * 2 {
            let mut cand = vec![];
            for i in 1..input.N - 1 {
                for j in 1..input.N - 1 {
                    if used_pos[i][j] {
                        continue;
                    }
                    let pos = Coord::new(i, j);
                    let mut added = 0;
                    for &dij in ADJ.iter() {
                        let next = pos + dij;
                        if next.in_map(input.N) {
                            for &idx in input.home_workspace_field[next.i][next.j].iter() {
                                if !used_home_workspace[idx] {
                                    added += 1;
                                }
                            }
                        }
                    }
                    cand.push((added, pos));
                }
            }
            cand.sort();
            cand.reverse();
            assert!(!cand.is_empty());
            let (added, pos) = cand[0];
            self.station_position.push(pos);
            used_pos[pos.i][pos.j] = true;
            for &dij in ADJ.iter() {
                let next = pos + dij;
                if next.in_map(input.N) {
                    for &idx in input.home_workspace_field[next.i][next.j].iter() {
                        used_home_workspace[idx] = true;
                    }
                }
            }
            cnt += added;
        }
        self.station_position = self.station_position.iter().take(100).cloned().collect();
    }
    pub fn can_connect(&self, station_pos: Coord, input: &Input) -> bool {
        // 駅が設置されていない場所には線路を設置できない
        if self.field[station_pos.i][station_pos.j] != Entity::Station {
            return false;
        }
        let mut cnt = 0;
        for &dij in DIJ4.iter() {
            let next = station_pos + dij;
            if next.in_map(input.N) {
                if self.field[next.i][next.j] != Entity::Empty {
                    cnt += 1;
                }
            } else {
                cnt += 1;
            }
        }
        // 駅が設置されている場所の周囲が線路または壁で埋まっている場合は線路を設置できない
        cnt < 4
    }
    pub fn prim(&mut self, input: &Input) {
        let mut dist_manhattan =
            vec![vec![!0; self.station_position.len()]; self.station_position.len()];
        for i in 0..self.station_position.len() {
            for j in 0..self.station_position.len() {
                if i == j {
                    continue;
                }
                dist_manhattan[i][j] =
                    calc_manhattan_dist(self.station_position[i], self.station_position[j]);
            }
        }

        let mut G = vec![vec![]; self.station_position.len()];
        let added_station_position = vec![];
        let mut used = vec![false; self.station_position.len()];
        let start = self.station_position[0];
        used[0] = true;
        self.field[start.i][start.j] = Entity::Station;

        while used.iter().any(|&x| !x) {
            let mut cand = vec![];
            for from in 0..self.station_position.len() {
                if !self.can_connect(self.station_position[from], input) {
                    continue;
                }
                for to in 0..self.station_position.len() {
                    if used[to] {
                        continue;
                    }
                    cand.push((dist_manhattan[from][to], from, to));
                }
            }
            // 実際の最短距離はマンハッタン距離と異なる場合があるので、駅に線路を敷く場合がある
            cand.sort();
            let mut ok = false;
            for (_, from, mut to) in cand {
                let prev = self.station_position[from];
                let mut next = self.station_position[to];
                let mut dist = vec![vec![NOT_VISITED; input.N]; input.N];
                for pos in added_station_position
                    .iter()
                    .chain(self.rail_position.iter())
                {
                    dist[pos.i][pos.j] = CANNOT_VISIT;
                }
                A_star(prev, next, &mut dist);
                if dist[next.i][next.j] == NOT_VISITED {
                    continue;
                }
                let mut route = bfs_revert(prev, next, &dist);

                // 線路の途中に駅がある場合があるので、最後の駅から一つ手前の駅までを取り出す
                let idx = {
                    let mut ret = 0;
                    for i in (0..route.len() - 1).rev() {
                        let pos = route[i];
                        if self.field[pos.i][pos.j] == Entity::Station {
                            ret = i;
                            break;
                        }
                    }
                    ret
                };
                route = route[idx..].to_vec();
                let from = self
                    .station_position
                    .iter()
                    .position(|&x| x == route[0])
                    .unwrap();

                let mut d = 1;
                for i in 1..route.len() - 1 {
                    let prev = route[i - 1];
                    let now = route[i];
                    if self.station_position.iter().any(|&x| x == now) {
                        to = self
                            .station_position
                            .iter()
                            .position(|&x| x == now)
                            .unwrap();
                        next = now;
                        break;
                    }
                    let next = route[i + 1];
                    let t = to_rail_type(prev, now, next);
                    self.rail_position.push(now);
                    self.field[now.i][now.j] = Entity::Rail(t);
                    d += 1;
                }
                used[to] = true;
                self.field[next.i][next.j] = Entity::Station;
                G[from].push((to, d));
                G[to].push((from, d));
                ok = true;
                break;
            }
            if !ok {
                break;
            }
        }
        self.G = G;
        self.visualize();
    }
    pub fn make_tree(&mut self, input: &Input) {
        let mut connected = vec![false; input.M * 2];
        let mut nodes = vec![vec![vec![]; input.N]; input.N];
        for i in 1..input.N - 1 {
            for j in 1..input.N - 1 {
                let pos = Coord::new(i, j);
                for &dij in ADJ.iter() {
                    let next = pos + dij;
                    if next.in_map(input.N) {
                        nodes[i][j].extend(input.home_workspace_field[next.i][next.j].clone());
                    }
                }
            }
        }
        let mut cand = vec![];

        // let step = 2;
        // for i0 in (1..input.N - 1).step_by(step) {
        //     for j0 in (1..input.N - 1).step_by(step) {
        //         for i1 in (i0 + 1..input.N - 1).step_by(step) {
        //             for j1 in (j0 + 1..input.N - 1).step_by(step) {
        //                 let pos0 = Coord::new(i0, j0);
        //                 let pos1 = Coord::new(i1, j1);
        //                 if pos0 == pos1 {
        //                     continue;
        //                 }
        //                 let dist = calc_manhattan_dist(pos0, pos1);
        //                 if dist > (input.N - 10000) / 100 {
        //                     continue;
        //                 }
        //                 let homes = nodes[i0][j0]
        //                     .iter()
        //                     .chain(nodes[i1][j1].iter())
        //                     .filter(|&&x| x < input.M)
        //                     .cloned();
        //                 let works = nodes[i0][j0]
        //                     .iter()
        //                     .chain(nodes[i1][j1].iter())
        //                     .filter(|&&x| x >= input.M)
        //                     .cloned();
        //                 let mut score = 0;
        //                 for home in homes {
        //                     for work in works.clone() {
        //                         if home + input.M == work {
        //                             let home_pos = input.home[home];
        //                             let work_pos = input.workspace[work - input.M];
        //                             score += calc_manhattan_dist(home_pos, work_pos);
        //                         }
        //                     }
        //                 }
        //                 cand.push((score * 1000 / dist, pos0, pos1));
        //             }
        //         }
        //     }
        // }

        // cand.sort();
        // cand.reverse();
        // let (_, from, to) = cand[0];
        // let mut dist = vec![vec![NOT_VISITED; input.N]; input.N];
        // A_star(from, to, &mut dist);
        // let route = bfs_revert(from, to, &dist);
        // assert!(route.len() == calc_manhattan_dist(from, to) + 1);
        // for pos in nodes[from.i][from.j].iter().chain(nodes[to.i][to.j].iter()) {
        //     connected[*pos] = true;
        // }
        // self.field[from.i][from.j] = Entity::Station;
        // self.field[to.i][to.j] = Entity::Station;
        // self.station_position.push(from);
        // self.station_position.push(to);
        // for i in 1..route.len() - 1 {
        //     let prev = route[i - 1];
        //     let now = route[i];
        //     let next = route[i + 1];
        //     let t = to_rail_type(prev, now, next);
        //     self.field[now.i][now.j] = Entity::Rail(t);
        //     self.rail_position.push(now);
        // }
        // eprintln!("initial");
        // eprintln!("from: {}, to: {}", from, to);

        for i in 1..input.N - 1 {
            for j in 1..input.N - 1 {
                cand.push((nodes[i][j].len(), Coord::new(i, j)));
            }
        }
        cand.sort();
        cand.reverse();
        let (_, pos) = cand[0];
        self.station_position.push(pos);
        self.field[pos.i][pos.j] = Entity::Station;

        while self.station_position.len() < 250 {
            let mut cand = vec![];
            'outer: for &pos in self.rail_position.iter() {
                for &dij in DIJ4.iter() {
                    let next = pos + dij;
                    if next.in_map(input.N) && self.field[next.i][next.j] == Entity::Station {
                        continue 'outer;
                    }
                }
                let mut income = 0;
                let mut new_nodes_cnt = 0;
                for &node in nodes[pos.i][pos.j].iter() {
                    if connected[node] {
                        continue;
                    }
                    new_nodes_cnt += 1;
                    let pair_node = if node < input.M {
                        node + input.M
                    } else {
                        node - input.M
                    };
                    if connected[pair_node] {
                        let new_coord = if node < input.M {
                            input.home[node]
                        } else {
                            input.workspace[node - input.M]
                        };
                        let pair_coord = if pair_node < input.M {
                            input.home[pair_node]
                        } else {
                            input.workspace[pair_node - input.M]
                        };
                        income += calc_manhattan_dist(new_coord, pair_coord);
                    }
                }
                if new_nodes_cnt > 0 {
                    cand.push((income * 1000, Coord::new(!0, !0), pos));
                }
            }

            for &from in self.station_position.iter() {
                let mut visited = vec![vec![0; input.N]; input.N];
                for pos in self
                    .rail_position
                    .iter()
                    .chain(self.station_position.iter())
                {
                    visited[pos.i][pos.j] = CANNOT_VISIT;
                }
                let mut bfs = BfsGenerator::new(from, &mut 0, &mut visited);
                while let Some((to, dist)) = bfs.next(1, &mut visited) {
                    // スタート駅はスキップ
                    if dist == 0 {
                        continue;
                    }
                    if dist > 20 {
                        break;
                    }
                    let mut income = 0;
                    let mut new_nodes_cnt = 0;
                    for &node in nodes[to.i][to.j].iter() {
                        if connected[node] {
                            continue;
                        } else {
                            new_nodes_cnt += 1;
                        }
                        let pair_node = if node < input.M {
                            node + input.M
                        } else {
                            node - input.M
                        };
                        if connected[pair_node] {
                            let new_coord = if node < input.M {
                                input.home[node]
                            } else {
                                input.workspace[node - input.M]
                            };
                            let pair_coord = if pair_node < input.M {
                                input.home[pair_node]
                            } else {
                                input.workspace[pair_node - input.M]
                            };
                            income += calc_manhattan_dist(new_coord, pair_coord);
                        }
                    }
                    if new_nodes_cnt > 0 {
                        cand.push((income * 1000 / calc_manhattan_dist(from, to), from, to));
                    }
                }
            }
            if cand.is_empty() {
                break;
            }
            cand.sort();
            cand.reverse();
            let (_, from, to) = cand[0];
            if from == Coord::new(!0, !0) {
                //  線路上に駅を設置
                self.field[to.i][to.j] = Entity::Station;
                self.station_position.push(to);
                let idx = self.rail_position.iter().position(|&x| x == to).unwrap();
                self.rail_position.remove(idx);
            } else {
                let mut dist = vec![vec![NOT_VISITED; input.N]; input.N];
                for pos in self.rail_position.iter() {
                    dist[pos.i][pos.j] = CANNOT_VISIT;
                }
                A_star(from, to, &mut dist);
                let mut route = bfs_revert(from, to, &dist);

                // // 線路の途中に駅がある場合があるので、最後の駅から一つ手前の駅までを取り出す
                let idx = {
                    let mut ret = 0;
                    for i in (0..route.len() - 1).rev() {
                        let pos = route[i];
                        if self.field[pos.i][pos.j] == Entity::Station {
                            ret = i;
                            break;
                        }
                    }
                    ret
                };
                route = route[idx..].to_vec();
                for i in 1..route.len() - 1 {
                    let prev = route[i - 1];
                    let now = route[i];
                    let next = route[i + 1];
                    let t = to_rail_type(prev, now, next);
                    self.rail_position.push(now);
                    self.field[now.i][now.j] = Entity::Rail(t);
                }
                self.station_position.push(to);
                self.field[to.i][to.j] = Entity::Station;
            }
            for node in nodes[to.i][to.j].iter() {
                connected[*node] = true;
            }
            if connected.iter().all(|&x| x) {
                break;
            }
        }
        self.G = vec![vec![]; self.station_position.len()];
        for &start in self.station_position.iter() {
            let start_idx = self
                .station_position
                .iter()
                .position(|&x| x == start)
                .unwrap();
            let mut dist = vec![vec![CANNOT_VISIT; input.N]; input.N];
            for pos in self
                .station_position
                .iter()
                .chain(self.rail_position.iter())
            {
                dist[pos.i][pos.j] = NOT_VISITED;
            }
            dist[start.i][start.j] = 0;
            let mut queue = BinaryHeap::new();
            queue.push((Reverse(0), start));
            while let Some((Reverse(d), pos)) = queue.pop() {
                if dist[pos.i][pos.j] < d {
                    continue;
                }
                let dijx = get_dij(self.field[pos.i][pos.j]);
                for &dij in dijx.iter() {
                    let next = pos + dij;
                    if next.in_map(input.N)
                        && dist[next.i][next.j] != CANNOT_VISIT
                        && dist[next.i][next.j] > d + 1
                    {
                        let can_go = {
                            let mut ok = false;
                            let dijx2 = get_dij(self.field[next.i][next.j]);
                            for &dij2 in dijx2.iter() {
                                let nn = next + dij2;
                                if nn.in_map(input.N) && nn == pos {
                                    ok = true;
                                }
                            }
                            ok
                        };

                        if can_go {
                            dist[next.i][next.j] = d + 1;
                            if self.field[next.i][next.j] == Entity::Station {
                                let next_idx = self
                                    .station_position
                                    .iter()
                                    .position(|&x| x == next)
                                    .unwrap();
                                self.G[start_idx].push((next_idx, d + 1));
                                // self.G[next_idx].push((start_idx, d + 1));
                            } else {
                                queue.push((Reverse(d + 1), next));
                            }
                        }
                    }
                }
            }
        }
        self.visualize();
    }
    pub fn visualize(&self) {
        for row in self.field.iter() {
            for &x in row.iter() {
                if x == Entity::Empty {
                    eprint!(".,");
                } else if x == Entity::Station {
                    eprint!("0,");
                } else {
                    eprint!(
                        "{},",
                        match x {
                            Entity::Rail(RailType::LeftToRight) => RailType::LeftToRight as i64,
                            Entity::Rail(RailType::UpToDown) => RailType::UpToDown as i64,
                            Entity::Rail(RailType::LeftToDown) => RailType::LeftToDown as i64,
                            Entity::Rail(RailType::LeftToUp) => RailType::LeftToUp as i64,
                            Entity::Rail(RailType::UpToRight) => RailType::UpToRight as i64,
                            Entity::Rail(RailType::DownToRight) => RailType::DownToRight as i64,
                            _ => unreachable!(),
                        }
                    );
                }
            }
            eprintln!()
        }
    }
}
