use std::{cmp::Reverse, collections::{BinaryHeap, VecDeque}, vec};

use crate::{coord::calc_manhattan_dist, Coord, DIJ4};

pub const NOT_VISITED: usize = 1 << 60;
pub const CANNOT_VISIT: usize = !0;

pub struct BfsGenerator {
    queue: VecDeque<(Coord, usize)>,
}

impl BfsGenerator {
    pub fn new(start: Coord, bfs_cnt: &mut usize, visited: &mut Vec<Vec<usize>>) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        *bfs_cnt += 1;
        visited[start.i][start.j] = *bfs_cnt;
        BfsGenerator { queue }
    }

    pub fn next(
        &mut self,
        bfs_cnt: usize,
        visited: &mut Vec<Vec<usize>>,
    ) -> Option<(Coord, usize)> {
        let N = visited.len();
        while let Some((coord, dist)) = self.queue.pop_front() {
            for &dij in DIJ4.iter() {
                let next = coord + dij;
                if next.in_map(N) 
                    // 通れる箇所
                    && visited[next.i][next.j] != CANNOT_VISIT
                    // まだ訪れていない箇所
                    && visited[next.i][next.j] != bfs_cnt
                    // 端でない箇所
                {
                    self.queue.push_back((next, dist + 1));
                    visited[next.i][next.j] = bfs_cnt;
                }
            }

            if coord.i != 0 && coord.i != N - 1 && coord.j != 0 && coord.j != N - 1 {
                return Some((coord, dist));
            }
        }
        None
    }
}


pub fn A_star(start: Coord, goal: Coord, dist: &mut Vec<Vec<usize>>) {
    let N = dist.len();
    let mut queue = BinaryHeap::new();
    dist[start.i][start.j] = 0;
    queue.push((Reverse(calc_manhattan_dist(start, goal)), 0, start));
    
    while let Some((_,d, pos)) = queue.pop() {
        if dist[pos.i][pos.j] < d {
            continue;
        }
        if pos == goal {
            return;
        }
        
        for &dij in DIJ4.iter() {
            let next = pos + dij;
            if next.in_map(N) && dist[next.i][next.j] != CANNOT_VISIT && d+1 < dist[next.i][next.j] {
                dist[next.i][next.j] = d+1;
                queue.push((Reverse(d+1 + calc_manhattan_dist(next, goal)), d+1, next));
            }
        }
    }
    
}
                
pub fn bfs_revert(start: Coord, goal: Coord, dist: &Vec<Vec<usize>>)-> Vec<Coord> {
    let N = dist.len();
    let mut ret = vec![];
    ret.push(goal);
    let mut pos = goal;
    let mut now = dist[goal.i][goal.j];
    'outer: while pos != start {
        for &dij in DIJ4.iter() {
            let next = pos + dij;
            if next.in_map(N) && dist[next.i][next.j] == now - 1 {
                pos = next;
                now -= 1;
                ret.push(pos);
                if pos == start {
                    break 'outer;
                }
            }
        }
    }
    ret.reverse();
    ret
}


pub fn Dijkstra_multi_start(starts: Vec<usize>, dist: &mut Vec<usize>, G: &Vec<Vec<(usize, usize)>>) {
    let mut queue = BinaryHeap::new();
    for idx in starts {
        dist[idx] = 0;
        queue.push((Reverse(0), idx));
    }
    
    while let Some((Reverse(d),from)) = queue.pop() {
        if d > 50 {
            break;
        }
        if dist[from] < d {
            continue;
        }
        
        for &(to, cost) in G[from].iter() {
            if dist[to] == CANNOT_VISIT {
                continue;
            }
            if dist[to] > d + cost {
                dist[to] = d + cost;
                queue.push((Reverse(d+cost), to));
            }            
        }
    }
}

pub fn Dijkstra_multi_start_revert(dist: &Vec<usize>, G: &Vec<Vec<(usize, usize)>>) -> Vec<(usize, usize, usize, Vec<usize>)> {
    let mut routes = vec![];
    for (idx, &d) in dist.iter().enumerate() {
        if d==0 || d == CANNOT_VISIT || d == NOT_VISITED {
            continue;
        }
        let mut to = idx;
        let mut now = d; 
        let mut route = vec![];
        while now > 0 {
            for &(from, cost) in G[to].iter() {
                if dist[from] == CANNOT_VISIT || dist[from] == NOT_VISITED {
                    continue;
                }
                if dist[from] == now - cost {
                    to = from;
                    now -= cost;
                    if now > 0 {
                        route.push(to);
                    }
                    break;
                }
            }
        }
        routes.push((to, idx, d, route));
        
    }
    routes
}