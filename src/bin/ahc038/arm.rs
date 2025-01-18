use itertools::iproduct;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{
    common::get_time,
    coord::{calc_manhattan_dist, Coord, DIJ4, DIJ5},
    state::{to_direction, to_rotate_direction, Direction, MoveAction},
};

#[derive(Debug)]
pub struct Arm {
    N: usize,
    pub start: Coord,
    pub not_finger_arm_num: usize,
    pub finger_num: usize,
    pub lengths: Vec<usize>,
    pub fingers: Vec<usize>, // length, parentsのindex
    pub parents: Vec<usize>, // parentのnode番号(0-indexed)
}

impl Arm {
    pub fn new(N: usize, V: usize, not_finger_arm_num: usize) -> Self {
        let finger_num = V - not_finger_arm_num - 1;
        let mut lengths = vec![];
        let mut parents = vec![];
        for idx in 0..not_finger_arm_num {
            lengths.push(1 << (idx + 1));
            parents.push(idx);
        }
        let mut fingers = vec![];

        for idx in not_finger_arm_num..V - 1 {
            parents.push(not_finger_arm_num);
            fingers.push(idx);
            lengths.push(N / 2);
        }

        Self {
            N,
            start: Coord::new(N / 2, N / 2),
            not_finger_arm_num,
            finger_num,
            lengths,
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
    pub fn can_reach(&self, opposite: bool) -> Vec<Vec<usize>> {
        let arm_direction = vec![Direction::Right; self.lengths.len()];
        let finger_parent_relative_positions =
            self.finger_parent_relative_position(&arm_direction, opposite);
        let mut can_reach = vec![vec![0; self.N]; self.N];
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
                        if (self.start + delta).in_map(self.N) && pos.in_map(self.N) {
                            can_reach[pos.i][pos.j] += 1;
                        }
                    }
                }
            }
        }
        can_reach
    }
    fn calc_score(&self, can_reach: &Vec<Vec<usize>>, base: usize) -> usize {
        let mut range_score = 0;
        let mut comprehensiveness = 0;
        let center = Coord::new(self.N / 2, self.N / 2);
        for (i, j) in iproduct!(0..self.N, 0..self.N) {
            let pos = Coord::new(i, j);
            let dist = calc_manhattan_dist(pos, center);
            let num = can_reach[i][j];
            if num > 0 {
                comprehensiveness += 1;
            }

            let mut cnt = 1;
            while cnt <= num {
                let mut s = dist;
                for _ in 0..cnt {
                    s /= 2;
                }
                range_score += s;
                cnt += 1;
            }
        }
        range_score + comprehensiveness * base / self.N / self.N
    }
    pub fn climbing(&mut self, time_limit: f64, rng: &mut Pcg64Mcg) -> usize {
        let center = Coord::new(self.N / 2, self.N / 2);
        let mut base = 0;
        for (i, j) in iproduct!(0..self.N, 0..self.N) {
            let pos = Coord::new(i, j);
            base += calc_manhattan_dist(pos, center);
        }
        let opposite = false;
        let mut best_score = self.calc_score(&self.can_reach(opposite), base);
        eprintln!("Initial Arm Score = {}", best_score);

        while get_time() < time_limit {
            let arm_idx = rng.gen_range(0..self.lengths.len());
            let before_length = self.lengths[arm_idx];
            if rng.gen_bool(0.5) {
                self.lengths[arm_idx] = (before_length + rng.gen_range(1..self.N)).min(self.N - 1);
            } else {
                let mut length = before_length - rng.gen_range(1..self.N);
                if length >= self.N || length == 0 {
                    length = 1;
                }
                self.lengths[arm_idx] = length;
            }
            let score = self.calc_score(&self.can_reach(opposite), base);
            if score > best_score {
                best_score = score;
            } else {
                self.lengths[arm_idx] = before_length;
            }
        }

        eprintln!("Final Arm Score = {}", best_score);
        best_score
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
    use crate::common::get_time;

    use super::Arm;
    use colored::*;
    use rand_pcg::Pcg64Mcg;

    fn output(
        N: usize,
        can_reach_one_step: &Vec<Vec<usize>>,
        can_reach_two_step: &Vec<Vec<usize>>,
    ) {
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

    #[test]
    fn arm() {
        let N = 15;
        let V = 6;
        let time_limit = 0.25;
        let mut rng = Pcg64Mcg::new(100);
        println!("N: {}, V: {}", N, V);

        println!("Arm 2");
        let start = get_time();
        let mut arm2 = Arm::new(N, V, 2);
        output(N, &arm2.can_reach(false), &arm2.can_reach(true));
        let score2 = arm2.climbing(start + time_limit, &mut rng);
        output(N, &arm2.can_reach(false), &arm2.can_reach(true));
        eprintln!("Arm2: {:?}", arm2);

        println!("Arm 3");
        let start = get_time();
        let mut arm3 = Arm::new(N, V, 3);
        output(N, &arm3.can_reach(false), &arm3.can_reach(true));
        let score3 = arm3.climbing(start + time_limit, &mut rng);
        output(N, &arm3.can_reach(false), &arm3.can_reach(true));
        eprintln!("Arm3: {:?}", arm3);
        println!("Score2: {}, Score3: {}", score2, score3);
    }
}
