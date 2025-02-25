use itertools::Itertools;

use crate::{
    bfs::{bfs_revert, A_star, NOT_VISITED},
    coord::{calc_manhattan_dist, Coord, ADJ, NEG},
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

pub fn solve(input: &Input) {
    let stations = make_station_cand(input);
    eprintln!("L = {}", stations.len());
    make_initial_state(input, &stations);
}

#[derive(Debug)]
pub struct Station {
    pub idx: usize,
    pub pos: Coord,
    pub home: Vec<usize>,
    pub workspace: Vec<usize>,
}

pub fn make_station_cand(input: &Input) -> Vec<Station> {
    let mut stations = vec![];
    let mut used_pos = vec![vec![false; input.N]; input.N];
    let mut used_home_workspace = vec![false; input.M * 2];
    let mut home_workspace_info = vec![vec![vec![]; input.N]; input.N];

    for i in 1..input.N - 1 {
        for j in 1..input.N - 1 {
            let pos = Coord::new(i, j);
            for &dij in ADJ.iter() {
                let next = pos + dij;
                if next.in_map(input.N) {
                    home_workspace_info[i][j]
                        .extend(input.home_workspace_field[next.i][next.j].clone());
                }
            }
        }
    }
    let mut idx = 0;
    while !used_home_workspace.iter().all(|&x| x) {
        let mut cand = vec![];
        for i in 1..input.N - 1 {
            for j in 1..input.N - 1 {
                if used_pos[i][j] {
                    continue;
                }
                let mut added = 0;
                for &idx in home_workspace_info[i][j].iter() {
                    if used_home_workspace[idx] {
                        continue;
                    }
                    added += 1;
                }
                cand.push((added, Coord::new(i, j)));
            }
        }
        assert!(!cand.is_empty());
        cand.sort();
        cand.reverse();
        let (_, pos) = cand[0];
        used_pos[pos.i][pos.j] = true;
        let station = Station {
            idx,
            pos,
            home: home_workspace_info[pos.i][pos.j]
                .iter()
                .filter(|&&x| {
                    used_home_workspace[x] = true;
                    x < input.M
                })
                .cloned()
                .collect_vec(),
            workspace: home_workspace_info[pos.i][pos.j]
                .iter()
                .filter(|&&x| {
                    used_home_workspace[x] = true;
                    x >= input.M
                })
                .map(|&x| x - input.M)
                .collect_vec(),
        };
        stations.push(station);
        idx += 1;
    }
    stations
}

#[derive(Debug)]
pub struct State {
    pub money: i64,
    pub income: i64,
    pub score: i64,
    pub connected: Vec<bool>,
    pub used_station: Vec<bool>,
    pub field: Vec<Vec<Entity>>,
}

impl State {
    pub fn new(input: &Input, stations: &Vec<Station>, from: usize, to: usize) -> Self {
        let mut used_station = vec![false; stations.len()];
        let mut field = vec![vec![Entity::Empty; input.N]; input.N];
        used_station[from] = true;
        used_station[to] = true;
        field[stations[from].pos.i][stations[from].pos.j] = Entity::Station;
        field[stations[to].pos.i][stations[to].pos.j] = Entity::Station;

        let dist = calc_manhattan_dist(stations[from].pos, stations[to].pos);
        let mut income = 0;
        let mut connected = vec![false; input.M * 2];
        for idx in stations[from].home.iter().chain(stations[to].home.iter()) {
            connected[*idx] = true;
        }
        for idx in stations[from]
            .workspace
            .iter()
            .chain(stations[to].workspace.iter())
        {
            if connected[*idx] {
                income += calc_manhattan_dist(input.home[*idx], input.workspace[*idx]) as i64;
            }
            connected[*idx + input.M] = true;
        }
        let money = input.K as i64 - STATION_COST * 2 - RAIL_COST * (dist as i64 - 1) + income;
        let score = money + (input.T as i64 - dist as i64 - 1) * income;
        let mut dist = vec![vec![NOT_VISITED; input.N]; input.N];
        A_star(stations[from].pos, stations[to].pos, &mut dist);
        let route = bfs_revert(stations[from].pos, stations[to].pos, &dist);

        for i in 1..route.len() - 1 {
            let prev = route[i - 1];
            let now = route[i];
            let next = route[i + 1];
            let t = to_rail_type(prev, now, next);
            field[now.i][now.j] = Entity::Rail(t);
        }

        Self {
            money,
            income,
            score,
            connected,
            used_station,
            field,
        }
    }
    pub fn visualize(&self) {
        for row in self.field.iter() {
            for &x in row.iter() {
                if x == Entity::Empty {
                    eprint!(".,");
                } else if x == Entity::Station {
                    eprint!("X,");
                } else {
                    eprint!(
                        "{},",
                        match x {
                            Entity::Rail(RailType::LeftToRight) => '─',
                            Entity::Rail(RailType::UpToDown) => '│',
                            Entity::Rail(RailType::LeftToDown) => '┐',
                            Entity::Rail(RailType::LeftToUp) => '┘',
                            Entity::Rail(RailType::UpToRight) => '┼',
                            Entity::Rail(RailType::DownToRight) => '┌',
                            _ => unreachable!(),
                        }
                    );
                }
            }
            eprintln!()
        }
    }
}

pub fn make_initial_state(input: &Input, stations: &Vec<Station>) {
    let mut states = vec![];
    let L = stations.len();
    for from in 0..L {
        for to in from + 1..L {
            let dist = calc_manhattan_dist(stations[from].pos, stations[to].pos);
            let cost = STATION_COST * 2 + RAIL_COST * (dist as i64 - 1);
            if input.K < cost as usize {
                continue;
            }
            let state = State::new(input, stations, from, to);
            states.push(state);
        }
    }
}
