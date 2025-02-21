#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{cmp::Reverse, collections::BinaryHeap, vec};

use beam::{BeamSearch, ScoreOrder};
use bfs::{CANNOT_VISIT, NOT_VISITED};
use coord::{calc_manhattan_dist, Coord, DIJ4, NEG};
use input::Input;
use state::{to_rail_type, Entity, Op, RailTree};

use crate::{common::get_time, input::read_input};

mod beam;
mod bfs;
mod common;
mod coord;
mod dsu;
mod hash;
mod input;
mod state;
mod test;

const TLE: f64 = 2.95;

fn solve(input: &Input) {
    let mut rail_tree = RailTree::new(input);
    rail_tree.greedy_station(input);
    rail_tree.prim(input);
    let mut beam = BeamSearch::new(input, &rail_tree);
    let width = 100;
    let ops = beam.solve(width, input.T, input, &rail_tree, ScoreOrder::Descending);
    output(&ops, input, &rail_tree);
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}

fn output(ops: &Vec<Op>, input: &Input, rail_tree: &RailTree) {
    let mut turn = 0;
    let mut make_status = vec![vec![false; input.N]; input.N];

    for op in ops.iter() {
        if op.is_wait {
            turn += 1;
            println!("# turn: {}, wait", turn);
            println!("-1");
            // eprintln!("-1");
        } else if op.from.is_none() {
            let (to, _) = op.to;
            turn += 1;
            println!("# turn: {}, station: {}", turn, to);
            println!("0 {} {}", to.i, to.j);
            // eprintln!("0 {} {}", to.i, to.j);
        } else {
            let (from, _) = op.from.unwrap();
            let (to, _) = op.to;

            let mut dist = vec![vec![NOT_VISITED; input.N]; input.N];
            for i in 0..input.N {
                for j in 0..input.N {
                    if rail_tree.field[i][j] == Entity::Empty || make_status[i][j] {
                        dist[i][j] = CANNOT_VISIT;
                    }
                }
            }

            A_star_rail_tree(from, to, &mut dist, &rail_tree.field);
            let route = A_star_revert(from, to, &dist, &rail_tree.field);
            for i in 1..route.len() - 1 {
                let prev = route[i - 1];
                let next = route[i + 1];
                let now = route[i];
                let t = to_rail_type(prev, now, next);
                make_status[now.i][now.j] = true;
                turn += 1;
                println!("# turn: {}, rail: {} {}", turn, t as i64, now);
                println!("{} {} {}", t as i64, now.i, now.j);
            }

            turn += 1;
            println!("# turn: {}, station: {}", turn, to);
            println!("0 {} {}", to.i, to.j);
            // eprintln!("0 {} {}", to.i, to.j);
        }
    }

    while turn < input.T {
        turn += 1;
        println!("# turn: {}, wait added", turn);
        println!("-1");
        // eprintln!("-1");
    }
}

pub fn A_star_rail_tree(
    start: Coord,
    goal: Coord,
    dist: &mut Vec<Vec<usize>>,
    field: &Vec<Vec<Entity>>,
) {
    let N = dist.len();
    let mut queue = BinaryHeap::new();
    dist[start.i][start.j] = 0;
    queue.push((Reverse(calc_manhattan_dist(start, goal)), 0, start));

    while let Some((_, d, pos)) = queue.pop() {
        if dist[pos.i][pos.j] < d {
            continue;
        }
        if pos == goal {
            return;
        }

        let dijx = get_dij(field[pos.i][pos.j]);
        for &dij in dijx.iter() {
            let next = pos + dij;
            if next.in_map(N)
                && dist[next.i][next.j] != CANNOT_VISIT
                && d + 1 < dist[next.i][next.j]
            {
                let can_go = {
                    let mut ok = false;
                    let dij2 = get_dij(field[next.i][next.j]);
                    for &dij2 in dij2.iter() {
                        let nn = next + dij2;
                        if nn.in_map(N) && nn == pos {
                            ok = true;
                        }
                    }
                    ok
                };
                if can_go {
                    dist[next.i][next.j] = d + 1;
                    queue.push((
                        Reverse(d + 1 + calc_manhattan_dist(next, goal)),
                        d + 1,
                        next,
                    ));
                }
            }
        }
    }
}

pub fn A_star_revert(
    start: Coord,
    goal: Coord,
    dist: &Vec<Vec<usize>>,
    field: &Vec<Vec<Entity>>,
) -> Vec<Coord> {
    let N = dist.len();
    let mut ret = vec![];
    ret.push(goal);
    let mut pos = goal;
    let mut now = dist[goal.i][goal.j];
    while pos != start {
        let dijx = get_dij(field[pos.i][pos.j]);
        for dij in dijx {
            let next = pos + dij;
            if next.in_map(N) && dist[next.i][next.j] == now - 1 {
                let can_go = {
                    let mut ok = false;
                    let dijx2 = get_dij(field[next.i][next.j]);
                    for &dij2 in dijx2.iter() {
                        let nn = next + dij2;
                        if nn.in_map(N) && nn == pos {
                            ok = true;
                        }
                    }
                    ok
                };
                if can_go {
                    pos = next;
                    now -= 1;
                    ret.push(pos);
                    break;
                }
            }
        }
    }
    ret.reverse();
    ret
}

fn get_dij(entity: Entity) -> Vec<Coord> {
    if entity == Entity::Station {
        vec![
            Coord { i: 0, j: 1 },
            Coord { i: 1, j: 0 },
            Coord { i: 0, j: NEG },
            Coord { i: NEG, j: 0 },
        ]
    } else if entity == Entity::Rail(state::RailType::LeftToRight) {
        vec![Coord { i: 0, j: 1 }, Coord { i: 0, j: NEG }]
    } else if entity == Entity::Rail(state::RailType::UpToDown) {
        vec![Coord { i: 1, j: 0 }, Coord { i: NEG, j: 0 }]
    } else if entity == Entity::Rail(state::RailType::LeftToDown) {
        vec![Coord { i: 1, j: 0 }, Coord { i: 0, j: NEG }]
    } else if entity == Entity::Rail(state::RailType::LeftToUp) {
        vec![Coord { i: NEG, j: 0 }, Coord { i: 0, j: NEG }]
    } else if entity == Entity::Rail(state::RailType::UpToRight) {
        vec![Coord { i: 0, j: 1 }, Coord { i: NEG, j: 0 }]
    } else if entity == Entity::Rail(state::RailType::DownToRight) {
        vec![Coord { i: 0, j: 1 }, Coord { i: 1, j: 0 }]
    } else {
        unreachable!()
    }
}
