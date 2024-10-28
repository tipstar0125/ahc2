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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveAction {
    None,
    Right,
    Opposite,
    Left,
    Down,
    Up,
}

pub fn to_rotate_direction(x: usize) -> MoveAction {
    if x == 0 {
        MoveAction::None
    } else if x == 1 {
        MoveAction::Right
    } else if x == 2 {
        MoveAction::Opposite
    } else if x == 3 {
        MoveAction::Left
    } else {
        panic!();
    }
}

pub fn to_move_direction(x: usize) -> MoveAction {
    if x == 0 {
        MoveAction::Right
    } else if x == 1 {
        MoveAction::Down
    } else if x == 2 {
        MoveAction::Left
    } else if x == 3 {
        MoveAction::Up
    } else if x == 4 {
        MoveAction::None
    } else {
        panic!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FingerAction {
    None,
    Init,
    Grab,
    Release,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FingerHas {
    NotHas,
    Has,
}

#[derive(Debug)]
pub struct State {
    start: Coord,
    root: Coord,
    arm_direction: Vec<Direction>,
    finger_status: Vec<(FingerAction, FingerHas)>,
    arm_length: Vec<usize>,
    finger_index: Vec<usize>,
    finger_parent: usize,
    S: Vec<Vec<char>>,
}

impl State {
    pub fn new(n: usize, v: usize, S: &Vec<Vec<char>>) -> Self {
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
            finger_status: vec![(FingerAction::Init, FingerHas::NotHas); arm_length.len()],
            arm_length,
            finger_index,
            finger_parent,
            S: S.clone(),
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
    pub fn next_finger_parent_position(&self) -> Vec<(Coord, Vec<MoveAction>)> {
        let finger_parent_depth = self.arm_length.len() - self.finger_index.len();
        // 現在の位置, 累積回転数, 深さ, 行動
        let mut Q = vec![(self.root, 0, 0, vec![])];
        let mut cands = vec![];
        while let Some((pos, rotate, depth, actions)) = Q.pop() {
            if depth == finger_parent_depth {
                // 上下左右に根が動く、または停止
                for i in 0..=4 {
                    let delta = DIJ5[i];
                    let move_action: MoveAction = to_move_direction(i);
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
    pub fn next_cands(
        &self,
        T: &Vec<Vec<char>>,
        // スコア、腕の行動、指の行動、指先の座標
    ) -> Vec<(usize, Vec<MoveAction>, Vec<FingerAction>, Vec<Coord>)> {
        let next_finger_parent_position = self.next_finger_parent_position();
        let not_figner_arm_num = self.arm_length.len() - self.finger_index.len();
        let mut cands = vec![];
        for (finger_parent_pos, finger_parent_action) in next_finger_parent_position.iter() {
            // 指がついた腕に伝搬される累積回転数を求める
            let rotate = finger_parent_action
                .iter()
                .skip(1) // 最初の操作は根の移動なのでスキップ
                .fold(0, |sum, &x| (sum + x as usize) % 4);

            let mut score = 0;
            let mut finger_rotate_actions = vec![];
            let mut finger_actions = vec![];
            let mut finger_coords = vec![];
            // 指を持たない腕のアクションは何もしないで埋めておく
            for _ in 0..not_figner_arm_num {
                // finger_rotate_actions.push(MoveAction::None); 既に計算済みなのでマージしない
                finger_actions.push(FingerAction::None);
                finger_coords.push(Coord::new(!0, !0));
            }

            for idx in self.finger_index.iter() {
                let len = self.arm_length[*idx];
                let dir: Direction = to_direction((self.arm_direction[*idx] as usize + rotate) % 4);
                let (finger_action, finger_has) = self.finger_status[*idx];
                let mut best_score = 0;
                let mut best_rotate_action = MoveAction::None;
                let mut best_finger_action = FingerAction::None;
                let mut best_finger_coord = Coord::new(!0, !0);

                for i in 0..=3 {
                    if i == 2 && finger_action != FingerAction::None {
                        // 反対方向には一手で行けない
                        // ただし、直線で何もしていない場合は、2回行動できる
                        continue;
                    }
                    let next_dir: Direction = to_direction((dir as usize + i) % 4);
                    let mut delta = DIJ4[next_dir as usize];
                    delta.i = delta.i.wrapping_mul(len);
                    delta.j = delta.j.wrapping_mul(len);
                    let finger_pos = *finger_parent_pos + delta;

                    // 掴んでいるモノを離す
                    if finger_has == FingerHas::Has
                        && finger_pos.in_map(self.S.len())
                        && self.S[finger_pos.i][finger_pos.j] == '0'
                        && T[finger_pos.i][finger_pos.j] == '1'
                        && best_score < 2
                    {
                        best_score = 2;
                        best_rotate_action = to_rotate_direction(i);
                        best_finger_action = FingerAction::Release;
                        best_finger_coord = finger_pos;

                    // 目的地に到達していないモノを掴む
                    } else if finger_has == FingerHas::NotHas
                        && finger_pos.in_map(self.S.len())
                        && self.S[finger_pos.i][finger_pos.j] == '1'
                        && T[finger_pos.i][finger_pos.j] == '0'
                        && best_score < 1
                    {
                        best_score = 1;
                        best_rotate_action = to_rotate_direction(i);
                        best_finger_action = FingerAction::Grab;
                        best_finger_coord = finger_pos;
                    }
                }
                score += best_score;
                finger_rotate_actions.push(best_rotate_action);
                finger_actions.push(best_finger_action);
                finger_coords.push(best_finger_coord);
            }
            let mut rotate_actions = finger_parent_action.clone();
            rotate_actions.extend(finger_rotate_actions);
            cands.push((score, rotate_actions, finger_actions, finger_coords));
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
        for i in 0..n {
            for j in 0..n {
                if input.S[i][j] == '1' {
                    print!("{}", "■ ".red());
                } else {
                    print!("□ ");
                }
            }
            println!();
        }

        let parents_cands = state.next_finger_parent_position();
        let mut cands = state.next_cands(&input.T);

        assert!(parents_cands.len() == cands.len());
        let mut score_cnt = 0;

        for (score, actions, _, finger_coords) in cands.iter() {
            let mut vis = vec![vec![false; n]; n];
            for coord in finger_coords.iter() {
                if !coord.in_map(n) {
                    continue;
                }
                vis[coord.i][coord.j] = true;
                assert!(input.S[coord.i][coord.j] == '1');
            }
            if *score == 0 {
                continue;
            }
            score_cnt += 1;
            let mut cnt = 0;
            println!();
            println!("Score: {}", score);
            println!("Actions: {:?}", actions);
            cnt += 3;
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
            thread::sleep(Duration::from_millis(100));
        }
        println!("{} / {}", score_cnt, cands.len());

        cands.sort();
        cands.reverse();
        let (best_score, best_actions, _, best_coords) = cands[0].clone();
        let mut vis = vec![vec![false; n]; n];
        println!("Best score: {}", best_score);
        println!("Action: {:?}", best_actions);
        for coord in best_coords.iter() {
            if !coord.in_map(n) {
                continue;
            }
            vis[coord.i][coord.j] = true;
            assert!(input.S[coord.i][coord.j] == '1');
        }
        for i in 0..n {
            for j in 0..n {
                if vis[i][j] {
                    print!("{}", "■ ".red());
                } else {
                    print!("□ ");
                }
            }
            println!();
        }
    }
}
