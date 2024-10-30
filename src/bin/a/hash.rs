use std::collections::HashSet;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::{coord::Coord, input::Input, state::Direction};

pub struct StateHash {
    field_status_hash_map: Vec<Vec<(usize, usize)>>,
    root_position_hash_map: Vec<Vec<usize>>,
    arm_direction_hash_map: Vec<Vec<usize>>,
}

fn gen_not_used(rng: &mut Pcg64Mcg, set: &mut HashSet<usize>) -> usize {
    loop {
        let r = rng.gen();
        if !set.contains(&r) {
            set.insert(r);
            return r;
        }
    }
}

impl StateHash {
    pub fn new(input: &Input, rng: &mut Pcg64Mcg) -> Self {
        let mut used = HashSet::new();
        let mut field_status_hash_map = vec![vec![(!0, !0); input.N]; input.N];
        for i in 0..input.N {
            for j in 0..input.N {
                field_status_hash_map[i][j] =
                    (gen_not_used(rng, &mut used), gen_not_used(rng, &mut used));
            }
        }

        let mut root_position_hash_map = vec![vec![!0; input.N]; input.N];
        for i in 0..input.N {
            for j in 0..input.N {
                root_position_hash_map[i][j] = gen_not_used(rng, &mut used);
            }
        }

        let mut arm_direction_hash_map = vec![vec![!0; 4]; input.V - 1];
        for i in 0..input.V - 1 {
            for j in 0..4 {
                arm_direction_hash_map[i][j] = gen_not_used(rng, &mut used);
            }
        }

        Self {
            field_status_hash_map,
            root_position_hash_map,
            arm_direction_hash_map,
        }
    }
    pub fn init(&self, input: &Input, root: Coord) -> usize {
        let mut ret = 0;
        for i in 0..input.N {
            for j in 0..input.N {
                if input.S[i][j] == '0' {
                    ret ^= self.field_status_hash_map[i][j].0;
                } else {
                    ret ^= self.field_status_hash_map[i][j].1;
                }
            }
        }
        ret ^= self.root_position_hash_map[root.i][root.j];
        for i in 0..input.V - 1 {
            // Right
            ret ^= self.arm_direction_hash_map[i][0];
        }
        ret
    }
    pub fn calc_field_status(&self, mut hash: usize, coords: &Vec<Coord>) -> usize {
        for coord in coords.iter() {
            hash ^= self.field_status_hash_map[coord.i][coord.j].0;
            hash ^= self.field_status_hash_map[coord.i][coord.j].1;
        }
        hash
    }
    pub fn calc_root_position(&self, mut hash: usize, pos1: Coord, pos2: Coord) -> usize {
        hash ^= self.root_position_hash_map[pos1.i][pos1.j];
        hash ^= self.root_position_hash_map[pos2.i][pos2.j];
        hash
    }
    pub fn calc_arm_direction(
        &self,
        mut hash: usize,
        directions: &Vec<(Direction, Direction)>,
    ) -> usize {
        for (dir, mp) in directions.iter().zip(self.arm_direction_hash_map.iter()) {
            hash ^= mp[dir.0 as usize];
            hash ^= mp[dir.1 as usize];
        }
        hash
    }
    pub fn calc(
        &self,
        mut hash: usize,
        field_change_coords: &Vec<Coord>,
        root_pos1: Coord,
        root_pos2: Coord,
        arm_directions: &Vec<(Direction, Direction)>,
    ) -> usize {
        hash ^= self.calc_field_status(hash, field_change_coords);
        hash ^= self.calc_root_position(hash, root_pos1, root_pos2);
        hash ^= self.calc_arm_direction(hash, arm_directions);
        hash
    }
}

#[cfg(test)]
mod tests {
    use rand_pcg::Pcg64Mcg;

    use crate::{coord::Coord, input::read_input};

    use super::StateHash;

    // #[test]
    fn check() {
        let input = read_input();
        let mut rng = Pcg64Mcg::new(0);
        let state_hash = StateHash::new(&input, &mut rng);
        println!(
            "hash: {}",
            state_hash.init(&input, Coord::new(input.N / 2, input.N / 2))
        )
    }
}
