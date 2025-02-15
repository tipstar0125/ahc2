use crate::{
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
            station_postion: vec![],
            field: vec![vec![Entity::Empty; input.N]; input.N],
            actions: vec![],
        }
    }
    pub fn make_station(&mut self, pos: Coord) {
        self.money -= STATION_COST;
        self.field[pos.i][pos.j] = Entity::Station;
        self.station_postion.push(pos);
        self.actions.push(format!("0 {} {}", pos.i, pos.j));
    }
    pub fn make_rail(&mut self, pos: Coord, t: RailType) {
        self.money -= RAIL_COST;
        self.field[pos.i][pos.j] = Entity::Rail(t);
        self.actions
            .push(format!("{} {} {}", t as i64, pos.i, pos.j,));
    }
    pub fn wait(&mut self) {
        self.actions.push("-1".to_string());
    }
    pub fn is_empty(&self, pos: Coord) -> bool {
        matches!(self.field[pos.i][pos.j], Entity::Empty)
    }
    pub fn is_done(&self) -> bool {
        self.turn == 800
    }
    pub fn greedy(&mut self, start_station: Coord, input: &Input) -> i64 {
        let mut home_workspace = vec![];
        for &dij in ADJ.iter() {
            let next = start_station + dij;
            if next.in_map(input.N) {
                home_workspace.extend(input.home_workspace_field[next.i][next.j].clone());
            }
        }
        // 周辺に自宅や職場がない場合はスキップ
        if home_workspace.is_empty() {
            return 0;
        }
        for &idx in home_workspace.iter().skip(1) {
            self.uf.unite(home_workspace[0], idx);
        }
        let root = self.uf.find(home_workspace[0]);
        self.make_station(start_station);

        self.money
    }
}
