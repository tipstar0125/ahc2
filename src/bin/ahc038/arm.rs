use crate::coord::Coord;

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
