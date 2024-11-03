use crate::{
    arm::Arm,
    coord::{Coord, DIJ4, DIJ5},
    input::Input,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
    None,
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

pub fn move_action_to_directon(x: MoveAction) -> Direction {
    if x == MoveAction::Right {
        Direction::Right
    } else if x == MoveAction::Down {
        Direction::Down
    } else if x == MoveAction::Left {
        Direction::Left
    } else if x == MoveAction::Up {
        Direction::Up
    } else if x == MoveAction::None {
        Direction::None
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FingerHas {
    NotHas,
    Has,
}

#[derive(Debug, Clone)]
pub struct State {
    pub root: Coord,
    pub arm_direction: Vec<Direction>,
    pub finger_status: Vec<(FingerAction, FingerHas)>,
    pub S: Vec<Vec<char>>,
}

impl State {
    const GRAB_SCORE: usize = 1;
    const RELEASE_SCORE: usize = 2;
    pub fn new(arm: &Arm, input: &Input) -> Self {
        Self {
            root: arm.start,
            arm_direction: vec![Direction::Right; arm.lengths.len()],
            finger_status: vec![(FingerAction::Init, FingerHas::NotHas); arm.lengths.len()],
            S: input.S.clone(),
        }
    }
    pub fn necessary_score(&self, M: usize) -> usize {
        M * (Self::GRAB_SCORE + Self::RELEASE_SCORE)
    }
    pub fn position(&self, arm: &Arm) -> Vec<Coord> {
        let mut position = vec![];
        let mut pos = self.root;
        for idx in 0..arm.not_finger_arm_num {
            let len = arm.lengths[idx];
            let dir = self.arm_direction[idx];
            let mut delta = DIJ4[dir as usize];
            delta.i = delta.i.wrapping_mul(len);
            delta.j = delta.j.wrapping_mul(len);
            pos = pos + delta;
            position.push(pos);
        }
        for idx in arm.not_finger_arm_num..arm.lengths.len() {
            let len = arm.lengths[idx];
            let dir = self.arm_direction[idx];
            let mut delta = DIJ4[dir as usize];
            delta.i = delta.i.wrapping_mul(len);
            delta.j = delta.j.wrapping_mul(len);
            position.push(pos + delta);
        }
        position
    }
    pub fn next_finger_parent_position(
        &self,
        arm: &Arm,
    ) -> Vec<(Coord, Vec<(MoveAction, Direction)>)> {
        let n = self.S.len();
        let finger_parent_depth = arm.not_finger_arm_num;
        // 現在の位置, 累積回転数, 深さ, 行動
        let mut Q = vec![(self.root, 0, 0, vec![])];
        let mut cands = vec![];
        while let Some((pos, rotate, depth, actions)) = Q.pop() {
            if depth == finger_parent_depth {
                // 上下左右に根が動く、または停止
                for i in 0..=4 {
                    let delta = DIJ5[i];
                    let move_action: MoveAction = to_move_direction(i);
                    let next = pos + delta;
                    let root_next = self.root + delta;
                    if !root_next.in_map(n) {
                        continue;
                    }
                    let mut next_actions = vec![(move_action, Direction::None)]; // 根に方向は存在しないので、Noneを入れておく
                    next_actions.extend(actions.clone()); // 腕回転の行動を追加
                    cands.push((next, next_actions));
                }
                continue;
            }
            let len = arm.lengths[depth];
            let dir: Direction = self.arm_direction[depth];
            for i in 0..=3 {
                if i == 2 {
                    // 反対方向には一手で行けない
                    continue;
                }
                let next_dir: Direction = to_direction((dir as usize + rotate + i) % 4); // 腕の累積回転数を加える必要がある
                let mut next_actions = actions.clone();
                next_actions.push((to_rotate_direction(i), next_dir)); // 回転行動を追加
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
    pub fn cand(
        &self,
        arm: &Arm,
        T: &Vec<Vec<char>>,
    ) -> Vec<(
        usize,                                 // スコア
        Vec<(MoveAction, Direction)>, // ルートと腕の行動と方向(ルートの方向は常にNoneとし使用しない)
        Vec<(FingerAction, FingerHas, Coord)>, // 指の行動と座標
    )> {
        let next_finger_parent_position = self.next_finger_parent_position(arm);
        let mut cands = vec![];
        let mut score_more_than_zero = false;
        for (finger_parent_pos, finger_parent_action) in next_finger_parent_position.iter() {
            // 指がついた腕に伝搬される累積回転数を求める
            let rotate = finger_parent_action
                .iter()
                .skip(1) // 最初の操作は根の移動なのでスキップ
                .fold(0, |sum, &x| (sum + x.0 as usize) % 4);

            let mut score = 0;
            let mut finger_rotate_actions_and_directions = vec![];
            let mut finger_actions = vec![];
            // 指を持たない腕のアクションは何もしないで埋めておく
            for _ in 0..arm.not_finger_arm_num {
                finger_actions.push((FingerAction::None, FingerHas::NotHas, Coord::new(!0, !0)));
            }

            for idx in arm.fingers.iter() {
                let len = arm.lengths[*idx];
                let dir: Direction = to_direction((self.arm_direction[*idx] as usize + rotate) % 4);
                let (finger_action, finger_has) = self.finger_status[*idx];
                let mut best_score = 0;
                let mut best_rotate_action = MoveAction::None;
                let mut best_finger_direction = dir;
                let mut best_finger_action = FingerAction::None;
                let mut best_finger_has = finger_has;
                let mut delta = DIJ4[dir as usize];
                delta.i = delta.i.wrapping_mul(len);
                delta.j = delta.j.wrapping_mul(len);
                let mut best_finger_coord = *finger_parent_pos + delta;

                for i in 0..=3 {
                    if i == 2 && finger_action != FingerAction::None {
                        // 反対方向には一手で行けない
                        // ただし、直前で何もしていない場合は、2回行動できる
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
                        && best_score < Self::RELEASE_SCORE
                    {
                        best_score = Self::RELEASE_SCORE;
                        best_rotate_action = to_rotate_direction(i);
                        best_finger_direction = next_dir;
                        best_finger_action = FingerAction::Release;
                        best_finger_has = FingerHas::NotHas;
                        best_finger_coord = finger_pos;

                    // 目的地に到達していないモノを掴む
                    } else if finger_has == FingerHas::NotHas
                        && finger_pos.in_map(self.S.len())
                        && self.S[finger_pos.i][finger_pos.j] == '1'
                        && T[finger_pos.i][finger_pos.j] == '0'
                        && best_score < Self::GRAB_SCORE
                    {
                        best_score = Self::GRAB_SCORE;
                        best_rotate_action = to_rotate_direction(i);
                        best_finger_direction = next_dir;
                        best_finger_action = FingerAction::Grab;
                        best_finger_has = FingerHas::Has;
                        best_finger_coord = finger_pos;
                    }
                }
                score += best_score;
                finger_rotate_actions_and_directions
                    .push((best_rotate_action, best_finger_direction));
                finger_actions.push((best_finger_action, best_finger_has, best_finger_coord));
            }
            if score > 0 {
                score_more_than_zero = true;
            }
            let mut rotate_actions = finger_parent_action.clone();
            rotate_actions.extend(finger_rotate_actions_and_directions);
            cands.push((score, rotate_actions, finger_actions));
        }
        if score_more_than_zero {
            cands
        } else {
            let mut ret = vec![];
            for i in 0..4 {
                let n = self.S.len();
                let delta = DIJ4[i];
                let move_action: MoveAction = to_move_direction(i);
                let root_next = self.root + delta;
                if !root_next.in_map(n) {
                    continue;
                }
                let mut actions_and_directions = vec![(move_action, Direction::None)];
                for dir in self.arm_direction.iter() {
                    actions_and_directions.push((MoveAction::None, dir.clone()));
                }
                let mut finger_actions = vec![];
                for (_, has) in self.finger_status.iter() {
                    finger_actions.push((FingerAction::None, has.clone(), Coord::new(!0, !0)));
                }
                ret.push((0, actions_and_directions, finger_actions));
            }
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::arm::Arm;
    use crate::input::read_input;
    use crate::state::FingerAction;
    use colored::*;

    use super::State;
    use std::thread;
    use std::time::Duration;

    // #[test]
    fn check() {
        let input = read_input();
        let arm = Arm::new(&input);
        let state = State::new(&arm, &input);
        println!("{:?}", state);
        let n = input.N;
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

        let parents_cands = state.next_finger_parent_position(&arm);
        let mut cands = state.cand(&arm, &input.T);

        assert!(parents_cands.len() == cands.len());
        let mut score_cnt = 0;

        for (score, arm, finger) in cands.iter() {
            if *score == 0 {
                continue;
            }

            let mut vis = vec![vec![false; n]; n];
            for (action, _, coord) in finger.iter() {
                if !coord.in_map(n) {
                    continue;
                }
                if *action != FingerAction::Grab && *action != FingerAction::Release {
                    continue;
                }
                vis[coord.i][coord.j] = true;
                assert!(input.S[coord.i][coord.j] == '1');
            }

            score_cnt += 1;
            let mut cnt = 0;
            println!();
            println!("Score: {}", score);
            println!("Actions: {:?}", arm);
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
        let (best_score, best_arm, best_finger) = cands[0].clone();
        let mut vis = vec![vec![false; n]; n];
        println!("Best score: {}", best_score);
        println!("Arm: {:?}", best_arm);
        println!("Finger: {:?}", best_finger);
        for (action, _, coord) in best_finger.iter() {
            if !coord.in_map(n) {
                continue;
            }
            if *action != FingerAction::Grab && *action != FingerAction::Release {
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
