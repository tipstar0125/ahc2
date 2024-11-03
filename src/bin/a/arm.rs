use crate::{coord::Coord, input::Input};

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
    pub fn new(input: &Input) -> Self {
        // 長さ2から始めて、2冪の腕を、腕の総和がN/2以上になるまで追加
        let mut arm_length = vec![];
        let mut parents = vec![];
        let mut v_cnt = 1;
        let mut arm_length_sum = 0;
        while arm_length_sum < input.N / 2 && v_cnt < input.V {
            let length = 1 << v_cnt;
            arm_length.push(length);
            parents.push(v_cnt - 1);
            arm_length_sum += length;
            v_cnt += 1;
        }

        // 腕の先端に先端が指になる腕を追加
        let mut length = 0;
        let mut fingers = vec![];
        let finger_parent = arm_length.len();
        while v_cnt < input.V {
            arm_length.push(length % input.N + 1); // 長さが1～nの範囲になるように制限
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
        assert!(arm_length.len() == input.V - 1);
        assert!(arm_length.len() == parents.len());

        Self {
            start: Coord::new(input.N / 2, input.N / 2),
            finger_num,
            not_finger_arm_num: input.V - finger_num - 1, // rootを除く
            lengths: arm_length,
            fingers,
            parents,
        }
    }
    pub fn output(&self) {
        // V
        eprintln!("{}", self.lengths.len() + 1);
        println!("{}", self.lengths.len() + 1);
        // parent Length
        for (p, len) in self.parents.iter().zip(self.lengths.iter()) {
            eprintln!("{} {}", p, len);
            println!("{} {}", p, len);
        }
        // x y
        eprintln!("{} {}", self.start.i, self.start.j);
        println!("{} {}", self.start.i, self.start.j);
    }
}

#[cfg(test)]
mod tests {
    use crate::input::read_input;

    use super::Arm;

    // #[test]
    fn check() {
        let input = read_input();
        let arm = Arm::new(&input);
        eprintln!("{:?}", arm);
        arm.output();
    }
}
