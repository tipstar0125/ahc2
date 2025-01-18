use itertools::iproduct;

use crate::coord::{Coord, DXY4, TWO_BY_TWO};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Edge {
    RightUp,
    LeftUp,
    RightDown,
    LeftDown,
    Other,
}

fn usize_to_edge(n: usize) -> Edge {
    match n {
        0 => Edge::RightUp,
        1 => Edge::LeftUp,
        2 => Edge::RightDown,
        3 => Edge::LeftDown,
        _ => panic!(),
    }
}

pub fn polygon_grid_to_vertex_coords(grid: &Vec<Vec<bool>>) -> Vec<Coord> {
    let n = grid.len();
    let mut ret = vec![];
    // 左下から時計回りに辿る
    let start = {
        let mut start = None;
        for (y, x) in iproduct!(0..n + 1, 0..n + 1) {
            if is_vertex(Coord::new(x, y), grid) {
                start = Some(Coord::new(x, y));
                break;
            }
        }
        start
    };
    if start.is_none() {
        return ret;
    }

    let start = start.unwrap();
    let mut pos = start;
    let mut dir = Direction::Left;
    ret.push(start);

    loop {
        let edge = to_edge(pos, grid);
        // 向きと辺の対応を見て次の向きを決める
        match (dir, edge) {
            (Direction::Up, Edge::RightDown) => dir = Direction::Right,
            (Direction::Up, Edge::LeftDown) => dir = Direction::Left,
            (Direction::Right, Edge::LeftDown) => dir = Direction::Down,
            (Direction::Right, Edge::LeftUp) => dir = Direction::Up,
            (Direction::Down, Edge::RightUp) => dir = Direction::Right,
            (Direction::Down, Edge::LeftUp) => dir = Direction::Left,
            (Direction::Left, Edge::RightUp) => dir = Direction::Up,
            (Direction::Left, Edge::RightDown) => dir = Direction::Down,
            (_, Edge::Other) => {}
            _ => panic!(),
        }
        pos = pos + DXY4[dir as usize];

        // 最初の点に戻ったら終了
        if pos == start {
            break;
        }
        if is_vertex(pos, grid) {
            ret.push(pos);
        }
    }

    ret
}

// 2x2の領域に頂点があるかどうか
fn is_vertex(coord: Coord, grid: &Vec<Vec<bool>>) -> bool {
    let cnt = count_two_by_two(coord, grid);
    cnt == 1 || cnt == 3
}

// 2x2の領域のタイルの数を数える
fn count_two_by_two(coord: Coord, grid: &Vec<Vec<bool>>) -> usize {
    let n = grid.len();
    let mut cnt = 0;
    for dxy in TWO_BY_TWO.iter() {
        let nxt = coord + *dxy;
        if nxt.in_map(n) {
            cnt += grid[nxt.x][nxt.y] as usize;
        }
    }
    cnt
}

// 2x2の領域の辺の種類を返す
fn to_edge(coord: Coord, grid: &Vec<Vec<bool>>) -> Edge {
    let n = grid.len();
    let cnt = count_two_by_two(coord, grid);
    if cnt == 1 {
        for (i, dxy) in TWO_BY_TWO.iter().enumerate() {
            let nxt = coord + *dxy;
            if nxt.in_map(n) && grid[nxt.x][nxt.y] {
                return usize_to_edge(i);
            }
        }
        panic!();
    }
    if cnt == 3 {
        for (i, dxy) in TWO_BY_TWO.iter().enumerate() {
            let nxt = coord + *dxy;
            if nxt.in_map(n) && !grid[nxt.x][nxt.y] {
                return usize_to_edge(i);
            }
        }
        panic!();
    }
    Edge::Other
}

#[cfg(test)]
mod tests {

    use std::vec;

    use colored::*;
    use itertools::iproduct;

    use crate::coord::{calc_manhattan_dist, Coord};

    use super::polygon_grid_to_vertex_coords;

    #[test]
    fn polygon() {
        let grid = vec![
            vec![false, false, false, false, false, false, false, false],
            vec![false, true, true, true, false, true, true, false],
            vec![false, true, true, true, true, true, true, false],
            vec![false, false, true, false, false, true, false, false],
            vec![false, true, true, true, false, false, false, false],
            vec![false, true, true, true, false, false, false, false],
            vec![false, false, true, true, false, false, false, false],
            vec![false, false, false, false, false, false, false, false],
        ];

        let n = grid.len();
        for (y, x) in iproduct!((0..n).rev(), 0..n) {
            if grid[x][y] {
                print!("{}", "■ ".to_string().green());
            } else {
                print!("{}", "□ ".to_string().white());
            }
            if x == n - 1 {
                println!();
            }
        }
        let points = polygon_grid_to_vertex_coords(&grid);
        let ans = vec![
            Coord::new(1, 1),
            Coord::new(1, 4),
            Coord::new(2, 4),
            Coord::new(2, 5),
            Coord::new(1, 5),
            Coord::new(1, 7),
            Coord::new(3, 7),
            Coord::new(3, 6),
            Coord::new(4, 6),
            Coord::new(4, 5),
            Coord::new(3, 5),
            Coord::new(3, 3),
            Coord::new(4, 3),
            Coord::new(4, 4),
            Coord::new(7, 4),
            Coord::new(7, 2),
            Coord::new(6, 2),
            Coord::new(6, 1),
            Coord::new(4, 1),
            Coord::new(4, 2),
            Coord::new(3, 2),
            Coord::new(3, 1),
        ];
        assert!(points == ans);
        let mut edge_sum = 0;
        for i in 0..points.len() {
            let pos0 = points[i];
            let pos1 = points[(i + 1) % points.len()];
            edge_sum += calc_manhattan_dist(pos0, pos1);
        }
        assert!(edge_sum == 32);
    }
}
