use std::{collections::VecDeque, vec};

use itertools::Itertools;

use crate::{
    bfs::{bfs_revert, A_star, CANNOT_VISIT, NOT_VISITED},
    common::get_time,
    coord::{calc_manhattan_dist, Coord, ADJ, DIJ4, NEG},
    input::Input,
};

const TLE: f64 = 2.9;
const STATION_COST: i64 = 5000;
const RAIL_COST: i64 = 100;
const BEAM_WIDTH: usize = 100;

pub fn solve(input: &Input) {
    let stations = make_station_cand(input);
    eprintln!("L = {}", stations.len());
    let states = make_initial_state(input, &stations);
    let mut best_state: State = states[0].clone();
    let mut beam = vec![vec![]; input.T + 1];
    for state in states {
        if state.income > 0 {
            beam[state.turn].push(state);
        }
    }

    while get_time() < TLE {
        for turn in 0..input.T {
            for _ in 0..BEAM_WIDTH {
                if beam[turn].is_empty() {
                    break;
                }
                beam[turn].sort_unstable_by_key(|s| s.score);
                beam[turn].truncate(BEAM_WIDTH);
                let state = beam[turn].pop().unwrap();
                let next_states = state.cand(input, &stations);
                for next_state in next_states {
                    beam[next_state.turn].push(next_state);
                }
                break;
            }
        }
    }
    for turn in 0..input.T {
        if beam[turn].is_empty() {
            continue;
        }
        beam[turn].sort_unstable_by_key(|s| s.score);
        let state = beam[turn].pop().unwrap();
        if state.score > best_state.score {
            best_state = state;
        }
    }
    best_state.visualize();
    for action in best_state.actions {
        println!("{}", action);
    }
    for _ in 0..input.T - best_state.turn {
        println!("-1");
    }
}

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

#[derive(Debug, Clone)]
pub struct State {
    pub turn: usize,
    pub money: i64,
    pub income: i64,
    pub score: i64,
    pub connected: Vec<bool>,
    pub used_station: Vec<bool>,
    pub field: Vec<Vec<Entity>>,
    pub actions: Vec<String>,
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
        let turn = dist + 1;
        let score = money + (input.T as i64 - dist as i64) * income;
        let mut dist = vec![vec![NOT_VISITED; input.N]; input.N];
        A_star(stations[from].pos, stations[to].pos, &mut dist);
        let route = bfs_revert(stations[from].pos, stations[to].pos, &dist);

        let mut actions = vec![];
        actions.push(format!(
            "0 {} {}",
            stations[from].pos.i, stations[from].pos.j
        ));
        for i in 1..route.len() - 1 {
            let prev = route[i - 1];
            let now = route[i];
            let next = route[i + 1];
            let t = to_rail_type(prev, now, next);
            field[now.i][now.j] = Entity::Rail(t);
            actions.push(format!("{} {} {}", t as i64, now.i, now.j));
        }
        actions.push(format!("0 {} {}", stations[to].pos.i, stations[to].pos.j));

        Self {
            turn,
            money,
            income,
            score,
            connected,
            used_station,
            field,
            actions,
        }
    }
    pub fn cand(&self, input: &Input, stations: &Vec<Station>) -> Vec<State> {
        let mut cand = vec![];
        let state = self.clone();

        // 駅から線路を敷いて新しい駅を建てる
        let starts = (0..stations.len())
            .filter(|&x| state.used_station[x])
            .map(|x| stations[x].pos)
            .collect_vec();
        let goals = (0..stations.len())
            .filter(|&x| {
                !state.used_station[x]
                    && self.field[stations[x].pos.i][stations[x].pos.j] == Entity::Empty
            })
            .map(|x| stations[x].pos)
            .collect_vec();
        let mut dist = vec![vec![NOT_VISITED; input.N]; input.N];
        for i in 0..input.N {
            for j in 0..input.N {
                if state.field[i][j] != Entity::Empty && state.field[i][j] != Entity::Station {
                    dist[i][j] = CANNOT_VISIT;
                }
            }
        }
        bfs_multi_start(starts, &mut dist);
        let routes = bfs_multi_start_revert(goals, &dist);
        for route in routes {
            let to = route[route.len() - 1];
            let period = route.len() - 1;
            let mut next_state = state.clone();

            // 資金が足りるまで待機
            let mut wait = 0;
            while next_state.money
                + (next_state.income - RAIL_COST) * (period as i64 - 1)
                + next_state.income
                - STATION_COST
                + next_state.income * wait
                < STATION_COST + RAIL_COST * (period as i64 - 1)
            {
                wait += 1;
                next_state.turn += 1;
                next_state.money += next_state.income;
                next_state.actions.push(format!("-1"));
            }

            // 線路を敷く
            for idx in 1..route.len() - 1 {
                let prev = route[idx - 1];
                let now = route[idx];
                let next = route[idx + 1];
                let t = to_rail_type(prev, now, next);
                next_state.turn += 1;
                next_state.field[now.i][now.j] = Entity::Rail(t);
                next_state
                    .actions
                    .push(format!("{} {} {}", t as i64, now.i, now.j));
                next_state.money += next_state.income - RAIL_COST;
            }

            // 駅を建てる
            next_state.turn += 1;
            next_state.field[to.i][to.j] = Entity::Station;
            next_state.actions.push(format!("0 {} {}", to.i, to.j));
            let to_idx = stations.iter().position(|x| x.pos == to).unwrap();
            next_state.used_station[to_idx] = true;
            let mut added_income = 0;
            for &idx in stations[to_idx].home.iter() {
                if !next_state.connected[idx] && next_state.connected[idx + input.M] {
                    added_income +=
                        calc_manhattan_dist(input.home[idx], input.workspace[idx]) as i64;
                }
                next_state.connected[idx] = true;
            }
            for &idx in stations[to_idx].workspace.iter() {
                if next_state.connected[idx] && !next_state.connected[idx + input.M] {
                    added_income +=
                        calc_manhattan_dist(input.home[idx], input.workspace[idx]) as i64;
                }
                next_state.connected[idx + input.M] = true;
            }
            if added_income == 0 {
                continue;
            }
            next_state.money -= STATION_COST;
            next_state.income += added_income;
            next_state.money += next_state.income;
            next_state.score =
                next_state.money + (input.T as i64 - next_state.turn as i64) * next_state.income;
            if next_state.turn <= input.T && next_state.score > self.score {
                cand.push(next_state);
            }
        }

        // 線路上に新しい駅を建てる
        for i in 0..input.N {
            for j in 0..input.N {
                let pos = Coord::new(i, j);
                if self.field[i][j] == Entity::Empty || self.field[i][j] == Entity::Station {
                    continue;
                }
                if stations.iter().position(|x| x.pos == pos).is_none() {
                    continue;
                }
                let to = Coord::new(i, j);
                let mut next_state = state.clone();

                // 資金が足りるまで待機
                let mut wait = 0;
                while next_state.money + next_state.income - STATION_COST + next_state.income * wait
                    < STATION_COST
                {
                    wait += 1;
                    next_state.turn += 1;
                    next_state.money += next_state.income;
                    next_state.actions.push(format!("-1"));
                }

                // 駅を建てる
                next_state.turn += 1;
                next_state.field[to.i][to.j] = Entity::Station;
                next_state.actions.push(format!("0 {} {}", to.i, to.j));
                let to_idx = stations.iter().position(|x| x.pos == to).unwrap();
                next_state.used_station[to_idx] = true;
                let mut added_income = 0;
                for &idx in stations[to_idx].home.iter() {
                    if !next_state.connected[idx] && next_state.connected[idx + input.M] {
                        added_income +=
                            calc_manhattan_dist(input.home[idx], input.workspace[idx]) as i64;
                    }
                    next_state.connected[idx] = true;
                }
                for &idx in stations[to_idx].workspace.iter() {
                    if next_state.connected[idx] && !next_state.connected[idx + input.M] {
                        added_income +=
                            calc_manhattan_dist(input.home[idx], input.workspace[idx]) as i64;
                    }
                    next_state.connected[idx + input.M] = true;
                }
                if added_income == 0 {
                    continue;
                }
                next_state.money -= STATION_COST;
                next_state.income += added_income;
                next_state.money += next_state.income;
                next_state.score = next_state.money
                    + (input.T as i64 - next_state.turn as i64) * next_state.income;
                if next_state.turn <= input.T && next_state.score > self.score {
                    cand.push(next_state);
                }
            }
        }

        cand
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
                            Entity::Rail(RailType::UpToRight) => '└',
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

pub fn make_initial_state(input: &Input, stations: &Vec<Station>) -> Vec<State> {
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
    states
}

pub fn bfs_multi_start(starts: Vec<Coord>, dist: &mut Vec<Vec<usize>>) {
    let N = dist.len();
    let mut queue = VecDeque::new();
    for start in starts {
        dist[start.i][start.j] = 0;
        queue.push_back(start);
    }

    while let Some(pos) = queue.pop_front() {
        for &dij in DIJ4.iter() {
            let next = pos + dij;
            if next.in_map(N)
                && dist[next.i][next.j] != CANNOT_VISIT
                && dist[pos.i][pos.j] + 1 < dist[next.i][next.j]
            {
                dist[next.i][next.j] = dist[pos.i][pos.j] + 1;
                queue.push_back(next);
            }
        }
    }
}

pub fn bfs_multi_start_revert(goals: Vec<Coord>, dist: &Vec<Vec<usize>>) -> Vec<Vec<Coord>> {
    let N = dist.len();
    let mut routes = vec![];

    for goal in goals {
        if dist[goal.i][goal.j] == NOT_VISITED {
            continue;
        }
        let mut route = vec![goal];
        let mut pos = goal;
        let mut d = dist[goal.i][goal.j];
        while d > 0 {
            for &dij in DIJ4.iter() {
                let next = pos + dij;
                if next.in_map(N) && dist[next.i][next.j] == d - 1 {
                    pos = next;
                    d -= 1;
                    route.push(pos);
                    break;
                }
            }
        }
        route.reverse();
        routes.push(route);
    }
    routes
}
