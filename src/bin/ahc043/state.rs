use crate::{
    bfs,
    coord::{Coord, ADJ},
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
    money: i64,
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
            money: input.K as i64,
            uf: UnionFind::new(input.M * 2),
            root: !0,
            station_postion: vec![],
            field: vec![vec![Entity::Empty; input.N]; input.N],
            actions: vec![],
        }
    }
    pub fn make_station(&mut self, pos: Coord, input: &Input) -> bool {
        let mut home_workspace = vec![];
        for &dij in ADJ.iter() {
            let next = pos + dij;
            if next.in_map(input.N) {
                home_workspace.extend(input.home_workspace_field[next.i][next.j].clone());
            }
        }
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
        self.turn == 800
    }
    pub fn greedy(&mut self, start_station: Coord, input: &Input) -> i64 {
        // 周辺に自宅や職場がない場合はスキップ
        if !self.make_station(start_station, input) {
            return 0;
        }

        // 現在ある駅からBFSをして、資金がつきない箇所に駅を設置することを考える
        // 駅を設置した場合に最終的に得られる資金を計算する

        let mut bfs_cnt = 0;
        let mut visited = vec![vec![0; input.N]; input.N];

        // 既に駅や線路がある場所は訪れないようにする
        for i in 0..input.N {
            for j in 0..input.N {
                if !self.is_empty(Coord::new(i, j)) {
                    visited[i][j] = !0;
                }
            }
        }

        while !self.is_done() {
            // let mut cand = vec![];
            for &station in self.station_postion.iter() {
                let mut bfs = bfs::BfsGenerator::new(station, &mut bfs_cnt, &mut visited);
                while let Some((next, dist)) = bfs.next(bfs_cnt, &mut visited) {
                    // スタート駅はスキップ
                    if dist == 0 {
                        continue;
                    }
                }
            }
        }

        self.money
    }
}
