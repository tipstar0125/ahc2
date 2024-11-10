use rustc_hash::FxHashSet;

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

pub fn move_action_to_direction(x: MoveAction) -> Direction {
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
pub struct Op {
    pub move_actions: Vec<(MoveAction, Direction)>,
    pub finger_actions: Vec<(FingerAction, FingerHas, Coord)>,
}

#[derive(Debug, Clone)]
pub struct State {
    pub root: Coord,
    pub arm_direction: Vec<Direction>,
    pub finger_status: Vec<(FingerAction, FingerHas)>,
    pub field: FxHashSet<Coord>,
    pub score: usize,
    pub hash: usize,
}

impl State {
    pub fn new(input: &Input) -> Self {
        let mut field = FxHashSet::default();
        for i in 0..input.N {
            for j in 0..input.N {
                if input.S[i][j] == '1' {
                    field.insert(Coord::new(i, j));
                }
            }
        }

        Self {
            root: input.arm.start,
            arm_direction: vec![Direction::Right; input.arm.lengths.len()],
            finger_status: vec![(FingerAction::Init, FingerHas::NotHas); input.arm.lengths.len()],
            field,
            score: 0,
            hash: input
                .calc_hash
                .init(input.N, input.V, &input.S, input.arm.start),
        }
    }
    pub fn is_done(&self, input: &Input, score: usize) -> bool {
        score == input.necessary_score
    }
    pub fn cand(
        &self,
        input: &Input,
    ) -> Vec<(
        usize, // スコア
        usize, // ハッシュ
        Op,
        bool, // is_done
    )> {
        // 直前で根以外が行動していなければ、反対方向へ移動可能
        let opposite = self.finger_status.iter().all(|x| x.0 == FingerAction::None);
        let finger_parent_relative_position = input
            .arm
            .finger_parent_relative_position(&self.arm_direction, opposite);
        let mut cands = vec![];

        // 上下左右に根が動く、または停止
        for i in 0..=4 {
            let delta = DIJ5[i];
            let move_action: MoveAction = to_move_direction(i);
            let root_next = self.root + delta;
            if !root_next.in_map(input.N) {
                continue;
            }
            let mut score_is_zero = false;
            let mut root_move_cands = vec![];

            for (finger_parent_relative_pos, finger_parent_action) in
                finger_parent_relative_position.iter()
            {
                let finger_parent_pos = self.root + *finger_parent_relative_pos + delta;
                // 指がついた腕に伝搬される累積回転数を求める
                let rotate = finger_parent_action
                    .iter()
                    .fold(0, |sum, &x| (sum + x.0 as usize) % 4);

                let mut score = 0;
                let mut finger_rotate_actions_and_directions = vec![];
                let mut finger_actions = vec![];
                // 指を持たない腕のアクションは何もしないで埋めておく
                for _ in 0..input.arm.not_finger_arm_num {
                    finger_actions.push((
                        FingerAction::None,
                        FingerHas::NotHas,
                        Coord::new(!0, !0),
                    ));
                }

                for idx in input.arm.fingers.iter() {
                    let len = input.arm.lengths[*idx];
                    let dir: Direction =
                        to_direction((self.arm_direction[*idx] as usize + rotate) % 4);
                    let (finger_action, finger_has) = self.finger_status[*idx];
                    let mut best_score = 0;
                    let mut best_rotate_action = MoveAction::None;
                    let mut best_finger_direction = dir;
                    let mut best_finger_action = FingerAction::None;
                    let mut best_finger_has = finger_has;
                    let delta = DIJ4[dir as usize] * Coord::new(len, len);
                    let mut best_finger_coord = finger_parent_pos + delta;

                    for i in 0..=3 {
                        if i == 2 && finger_action != FingerAction::None {
                            // 反対方向には一手で行けない
                            // ただし、直前で何もしていない場合は、2回行動できる
                            continue;
                        }
                        let next_dir: Direction = to_direction((dir as usize + i) % 4);
                        let delta = DIJ4[next_dir as usize] * Coord::new(len, len);
                        let finger_pos = finger_parent_pos + delta;

                        // 掴んでいるモノを離す
                        if finger_has == FingerHas::Has
                            && finger_pos.in_map(input.N)
                            && !self.field.contains(&Coord::new(finger_pos.i, finger_pos.j))
                            && input.T[finger_pos.i][finger_pos.j] == '1'
                            && best_score < input.release_score
                        {
                            best_score = input.release_score;
                            best_rotate_action = to_rotate_direction(i);
                            best_finger_direction = next_dir;
                            best_finger_action = FingerAction::Release;
                            best_finger_has = FingerHas::NotHas;
                            best_finger_coord = finger_pos;

                        // 目的地に到達していないモノを掴む
                        } else if finger_has == FingerHas::NotHas
                            && finger_pos.in_map(input.N)
                            && self.field.contains(&Coord::new(finger_pos.i, finger_pos.j))
                            && input.T[finger_pos.i][finger_pos.j] == '0'
                            && best_score < input.grab_score
                        {
                            best_score = input.grab_score;
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

                // スコア0の場合は、根の移動以外は同一視
                if score == 0 {
                    if !score_is_zero {
                        let mut actions_and_directions = vec![(move_action, Direction::None)];
                        for dir in self.arm_direction.iter() {
                            actions_and_directions.push((MoveAction::None, dir.clone()));
                        }
                        let mut finger_actions = vec![];
                        for (_, has) in self.finger_status.iter() {
                            finger_actions.push((
                                FingerAction::None,
                                has.clone(),
                                Coord::new(!0, !0),
                            ));
                        }
                        let hash = input
                            .calc_hash
                            .calc_root_position(self.hash, self.root, root_next);
                        let op = Op {
                            move_actions: actions_and_directions,
                            finger_actions,
                        };
                        root_move_cands.push((0, hash, op, self.is_done(input, self.score)));
                        score_is_zero = true;
                    }
                    continue;
                }

                let mut rotate_actions = vec![(move_action, Direction::None)];
                rotate_actions.extend(finger_parent_action.clone());
                rotate_actions.extend(finger_rotate_actions_and_directions);

                let field_change_coords: Vec<Coord> = finger_actions
                    .iter()
                    .filter(|x| x.0 == FingerAction::Grab || x.0 == FingerAction::Release)
                    .map(|x| x.2)
                    .collect();
                let arm_direction_changes: Vec<(Direction, Direction)> = rotate_actions
                    .iter()
                    .skip(1)
                    .map(|x| x.1)
                    .zip(self.arm_direction.iter().cloned())
                    .collect();
                let hash = input.calc_hash.calc(
                    self.hash,
                    &field_change_coords,
                    self.root,
                    root_next,
                    &arm_direction_changes,
                );
                let op = Op {
                    move_actions: rotate_actions,
                    finger_actions,
                };
                root_move_cands.push((score, hash, op, self.is_done(input, self.score + score)));
            }
            cands.extend(root_move_cands);
        }
        cands
    }
    pub fn apply(&mut self, score: usize, hash: usize, op: &Op) {
        self.root = self.root + DIJ5[move_action_to_direction(op.move_actions[0].0) as usize];
        self.arm_direction = op
            .move_actions
            .iter()
            .cloned()
            .skip(1)
            .map(|x| x.1)
            .collect::<Vec<Direction>>();
        self.finger_status = op
            .finger_actions
            .iter()
            .cloned()
            .map(|x| (x.0, x.1))
            .collect::<Vec<(FingerAction, FingerHas)>>();
        for (finger_action, _, coord) in op.finger_actions.iter() {
            if *finger_action == FingerAction::Grab {
                self.field.remove(&Coord::new(coord.i, coord.j));
            } else if *finger_action == FingerAction::Release {
                self.field.insert(Coord::new(coord.i, coord.j));
            }
        }
        self.score = score;
        self.hash = hash;
    }
}
