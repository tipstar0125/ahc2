use crate::{
    coord::{Coord, DIJ4, DIJ5},
    state::{to_direction, to_rotate_direction, Direction, MoveAction},
};

#[derive(Debug)]
pub struct Arm {
    pub start: Coord,
    pub not_finger_arm_num: usize,
    pub finger_num: usize,
    pub lengths: Vec<usize>,
    pub fingers: Vec<usize>, // length, parentsのindex
    pub parents: Vec<usize>, // parentのnode番号(0-indexed)
}

impl Arm {
    pub fn new(N: usize, V: usize) -> Self {
        // 長さ2から始めて、2冪の腕を、腕の総和がN/2以上になるまで追加
        let mut arm_length = vec![];
        let mut parents = vec![];
        let mut v_cnt = 1;
        let mut arm_length_sum = 0;
        let arm_delta = if N >= 25 { 7 } else { 3 };
        while arm_length_sum < N / 2 && v_cnt < V {
            let length = arm_delta * v_cnt;
            arm_length.push(length);
            parents.push(v_cnt - 1);
            arm_length_sum += length;
            v_cnt += 1;
        }

        // 腕の先端に先端が指になる腕を追加
        let mut length = 0;
        let mut fingers = vec![];
        let finger_parent = arm_length.len();
        while v_cnt < V {
            arm_length.push(length % N + 1); // 長さが1～nの範囲になるように制限
            fingers.push(v_cnt - 1);
            parents.push(finger_parent);
            length += 1;
            v_cnt += 1;
        }

        // 指がない場合は、最先端の腕を指とする
        if fingers.is_empty() {
            fingers.push(arm_length.len() - 1);
        }
        let finger_num = fingers.len();
        assert!(finger_num > 0);
        assert!(arm_length.len() == V - 1);
        assert!(arm_length.len() == parents.len());

        Self {
            start: Coord::new(N / 2, N / 2),
            finger_num,
            not_finger_arm_num: V - finger_num - 1, // rootを除く
            lengths: arm_length,
            fingers,
            parents,
        }
    }
    pub fn finger_parent_relative_position(
        &self,
        arm_direction: &Vec<Direction>,
        opposite: bool,
    ) -> Vec<(Coord, Vec<(MoveAction, Direction)>)> {
        let finger_parent_depth = self.not_finger_arm_num;
        // 現在の位置, 累積回転数, 深さ, 行動
        let mut Q = vec![(Coord::new(0, 0), 0, 0, vec![])];
        let mut ret = vec![];
        while let Some((pos, rotate, depth, actions)) = Q.pop() {
            if depth == finger_parent_depth {
                ret.push((pos, actions));
                continue;
            }
            let len = self.lengths[depth];
            let dir: Direction = arm_direction[depth];
            for i in 0..=3 {
                if i == 2 && !opposite {
                    // 反対方向には一手で行けない
                    continue;
                }
                let next_dir: Direction = to_direction((dir as usize + rotate + i) % 4); // 腕の累積回転数を加える必要がある
                let mut next_actions = actions.clone();
                next_actions.push((to_rotate_direction(i), next_dir)); // 回転行動を追加
                let next_rotate = (rotate + i) % 4; // 伝搬させる累積回転数
                let delta = DIJ4[next_dir as usize] * Coord::new(len, len);
                let next = pos + delta;
                Q.push((next, next_rotate, depth + 1, next_actions));
            }
        }
        ret
    }
    pub fn can_reach(&self, N: usize, opposite: bool) -> Vec<Vec<usize>> {
        let arm_direction = vec![Direction::Right; self.lengths.len()];
        let finger_parent_relative_positions =
            self.finger_parent_relative_position(&arm_direction, opposite);
        let mut can_reach = vec![vec![0; N]; N];
        for (pos, action) in finger_parent_relative_positions.iter() {
            let parent_pos = self.start + *pos;
            // 指がついた腕に伝搬される累積回転数を求める
            let rotate = action.iter().fold(0, |sum, &x| (sum + x.0 as usize) % 4);
            for idx in self.fingers.iter() {
                let len = self.lengths[*idx];
                let dir: Direction = to_direction((arm_direction[*idx] as usize + rotate) % 4);

                for i in 0..=3 {
                    if i == 2 && !opposite {
                        // 反対方向には一手で行けない
                        continue;
                    }
                    let next_dir: Direction = to_direction((dir as usize + i) % 4);
                    let delta = DIJ4[next_dir as usize] * Coord::new(len, len);
                    let finger_pos = parent_pos + delta;
                    for i in 0..=4 {
                        let delta = DIJ5[i];
                        let pos = finger_pos + delta;
                        if (self.start + delta).in_map(N) && pos.in_map(N) {
                            can_reach[pos.i][pos.j] += 1;
                        }
                    }
                }
            }
        }
        can_reach
    }
    pub fn output(&self) -> String {
        let mut output = "".to_string();
        // V
        output += format!("{}\n", self.lengths.len() + 1).as_str();
        // parent Length
        for (p, len) in self.parents.iter().zip(self.lengths.iter()) {
            output += format!("{} {}\n", p, len).as_str();
        }
        // x y
        output += format!("{} {}\n", self.start.i, self.start.j).as_str();
        output
    }
}

#[cfg(test)]
mod tests {
    use super::Arm;
    use colored::*;

    #[test]
    fn arm() {
        let N = 25;
        let V = 15;
        let arm = Arm::new(N, V);
        println!("N: {}, V: {}", N, V);
        let can_reach_one_step = arm.can_reach(N, false);
        let can_reach_two_step = arm.can_reach(N, true);
        for i in 0..N {
            for j in 0..N {
                if can_reach_one_step[i][j] >= 4 {
                    print!("{}", "■ ".to_string().magenta());
                } else if can_reach_one_step[i][j] == 3 {
                    print!("{}", "■ ".to_string().green());
                } else if can_reach_one_step[i][j] == 2 {
                    print!("{}", "■ ".to_string().blue());
                } else if can_reach_one_step[i][j] == 1 {
                    print!("{}", "■ ".to_string().white());
                } else {
                    print!("{}", "□ ".to_string().white());
                }
            }
            print!("     ");
            for j in 0..N {
                if can_reach_two_step[i][j] >= 4 {
                    print!("{}", "■ ".to_string().magenta());
                } else if can_reach_two_step[i][j] == 3 {
                    print!("{}", "■ ".to_string().green());
                } else if can_reach_two_step[i][j] == 2 {
                    print!("{}", "■ ".to_string().blue());
                } else if can_reach_two_step[i][j] == 1 {
                    print!("{}", "■ ".to_string().cyan());
                } else {
                    print!("{}", "□ ".to_string().white());
                }
            }
            println!();
        }
    }
}
