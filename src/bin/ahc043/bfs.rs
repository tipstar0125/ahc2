use std::collections::VecDeque;

use crate::coord::{Coord, DIJ4};

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
                    // 通れない箇所
                    && visited[next.i][next.j] != !0 
                    // 既に訪れた箇所
                    && visited[next.i][next.j] != bfs_cnt
                {
                    self.queue.push_back((next, dist + 1));
                    visited[next.i][next.j] = bfs_cnt;
                }
            }

            return Some((coord, dist));
        }
        None
    }
}
