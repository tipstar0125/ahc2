use rustc_hash::FxHashSet;

use crate::coord::{Coord, DIJ4, DIJ5};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub fn to_direction(x: usize) -> Direction {
    if x == 0 {
        Direction::Right
    } else if x == 1 {
        Direction::Down
    } else if x == 2 {
        Direction::Left
    } else if x == 3 {
        Direction::Up
    } else {
        panic!();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Stop,
    Right,
    Opposite,
    Left,
    Down,
    Up,
}

pub fn to_rotate_direction(x: usize) -> Move {
    if x == 0 {
        Move::Stop
    } else if x == 1 {
        Move::Right
    } else if x == 2 {
        Move::Opposite
    } else if x == 3 {
        Move::Left
    } else {
        panic!();
    }
}

pub fn to_move_direction(x: usize) -> Move {
    if x == 0 {
        Move::Right
    } else if x == 1 {
        Move::Down
    } else if x == 2 {
        Move::Left
    } else if x == 3 {
        Move::Up
    } else if x == 4 {
        Move::Stop
    } else {
        panic!()
    }
}

#[derive(Debug)]
pub struct State {
    start: Coord,
    root: Coord,
    arm_direction: Vec<Direction>,
    arm_length: Vec<usize>,
    finger_index: Vec<usize>,
    finger_parent: usize,
    // field: FxHashSet<(usize, usize)>,
}

impl State {
    pub fn new(n: usize, v: usize, field: &Vec<Vec<char>>) -> Self {
        // 長さ2から始めて、2冪の腕を、腕の総和がn以上になるまで追加
        let mut arm_length = vec![];
        let mut v_cnt = 1;
        let mut arm_length_sum = 0;
        while arm_length_sum < n / 2 && v_cnt < v {
            let length = 1 << v_cnt;
            arm_length.push(length);
            arm_length_sum += length;
            v_cnt += 1;
        }

        // 腕の先端に先端が指になる腕を追加
        let mut length = 0;
        let mut finger_index = vec![];
        let mut finger_parent = arm_length.len() - 1;
        while v_cnt < v {
            arm_length.push(length % n + 1); // 長さが1～nの範囲になるように制限
            finger_index.push(v_cnt - 1);
            length += 1;
            v_cnt += 1;
        }

        // 指がない場合は、最先端の腕を指とする
        if finger_index.is_empty() {
            finger_parent = arm_length.len() - 2;
            finger_index.push(arm_length.len() - 1);
        }
        Self {
            start: Coord::new(n / 2, n / 2),
            root: Coord::new(n / 2, n / 2),
            arm_direction: vec![Direction::Right; arm_length.len()],
            arm_length,
            finger_index,
            finger_parent,
        }
    }
    pub fn position(&self) -> Vec<Coord> {
        let mut position = vec![];
        let mut pos = self.root;
        for idx in 0..self.arm_length.len() - self.finger_index.len() {
            let len = self.arm_length[idx];
            let dir = self.arm_direction[idx];
            let mut delta = DIJ4[dir as usize];
            delta.i = delta.i.wrapping_mul(len);
            delta.j = delta.j.wrapping_mul(len);
            pos = pos + delta;
            position.push(pos);
        }
        for idx in self.arm_length.len() - self.finger_index.len()..self.arm_length.len() {
            let len = self.arm_length[idx];
            let dir = self.arm_direction[idx];
            let mut delta = DIJ4[dir as usize];
            delta.i = delta.i.wrapping_mul(len);
            delta.j = delta.j.wrapping_mul(len);
            position.push(pos + delta);
        }
        position
    }
    pub fn next_finger_parent_position(&self) -> Vec<(Coord, Vec<Move>)> {
        let finger_parent_depth = self.arm_length.len() - self.finger_index.len();
        // 現在の位置, 累積回転数, 深さ, 行動
        let mut Q = vec![(self.root, 0, 0, vec![])];
        let mut cands = vec![];
        while let Some((pos, rotate, depth, actions)) = Q.pop() {
            if depth == finger_parent_depth {
                // 上下左右に根が動く、または停止
                for i in 0..=4 {
                    let delta = DIJ5[i];
                    let move_action: Move = to_move_direction(i);
                    let mut next_actions = vec![move_action];
                    next_actions.extend(actions.clone()); // 腕回転の行動を追加
                    let next = pos + delta;
                    cands.push((next, next_actions));
                }
                continue;
            }
            let len = self.arm_length[depth];
            let dir: Direction = self.arm_direction[depth];
            for i in 0..=3 {
                if i == 2 {
                    // 反対方向には一手で行けない
                    continue;
                }
                let next_dir: Direction = to_direction((dir as usize + rotate + i) % 4); // 腕の累積回転数を加える必要がある
                let mut next_actions = actions.clone();
                next_actions.push(to_rotate_direction(i)); // 回転行動を追加
                let next_rotate = (rotate + i) % 4; // 伝搬させる累積回転数
                let mut delta = DIJ4[next_dir as usize];
                delta.i = delta.i.wrapping_mul(len);
                delta.j = delta.j.wrapping_mul(len);
                let next = pos + delta;
                Q.push((next, next_rotate, depth + 1, next_actions));
            }
        }
        cands
    }
    pub fn next_finger_position(&self) -> Vec<Vec<(usize, Vec<(Move, Coord)>)>> {
        let next_finger_parent_position = self.next_finger_parent_position();
        let mut cands = vec![];
        for (finger_parent_pos, finger_parent_move) in next_finger_parent_position.iter() {
            // 指がついた腕に伝搬される累積回転数を求める
            let rotate = finger_parent_move
                .iter()
                .skip(1) // 最初の操作は根の移動なのでスキップ
                .fold(0, |sum, &x| (sum + x as usize) % 4);
            let mut finger_cands = vec![];
            for idx in self.finger_index.iter() {
                let len = self.arm_length[*idx];
                let dir: Direction = to_direction((self.arm_direction[*idx] as usize + rotate) % 4);
                let mut finger_dir_cands = vec![];
                for i in 0..=3 {
                    if i == 2 {
                        // 反対方向には一手で行けない
                        continue;
                    }
                    let next_dir: Direction = to_direction((dir as usize + i) % 4);
                    let mut delta = DIJ4[next_dir as usize];
                    delta.i = delta.i.wrapping_mul(len);
                    delta.j = delta.j.wrapping_mul(len);
                    let finger_pos = *finger_parent_pos + delta;
                    finger_dir_cands.push((to_rotate_direction(i), finger_pos));
                }
                finger_cands.push((*idx, finger_dir_cands));
            }
            cands.push(finger_cands);
        }
        cands
    }
}

#[cfg(test)]
mod tests {
    use crate::input::read_input;
    use colored::*;

    use super::State;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn arm_check() {
        let input = read_input();
        let n = input.N;
        let state = State::new(input.N, input.V, &input.S);
        println!("{:?}", state);
        println!("{:?}", state.position());

        let parents_cands = state.next_finger_parent_position();
        let cands = state.next_finger_position();

        assert!(parents_cands.len() == cands.len());

        for finger_cands in cands.iter() {
            let mut vis = vec![vec![false; n]; n];
            let mut is_vis = false;
            for (_, finger_dir_cands) in finger_cands.iter() {
                for (_, coord) in finger_dir_cands.iter() {
                    if !coord.in_map(n) {
                        continue;
                    }
                    vis[coord.i][coord.j] = true;
                    is_vis = true;
                }
            }
            if !is_vis {
                continue;
            }
            let mut cnt = 0;
            println!();
            cnt += 1;
            for i in 0..n {
                for j in 0..n {
                    if vis[i][j] {
                        print!("{}", "■ ".red());
                    } else {
                        print!("□ ");
                    }
                }
                println!();
                cnt += 1;
            }
            print!("\x1b[{}A", cnt);
            thread::sleep(Duration::from_millis(300));
        }
    }
}
