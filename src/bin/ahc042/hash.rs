use std::collections::HashSet;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

#[derive(Debug)]
pub struct CalcHash {
    pub hash_map: Vec<Vec<usize>>,
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

impl CalcHash {
    pub fn new(N: usize) -> Self {
        let mut rng = Pcg64Mcg::new(20);
        let mut used = HashSet::new();
        let mut hash_map = vec![vec![!0; N]; N];
        for i in 0..N {
            for j in 0..N {
                hash_map[i][j] = gen_not_used(&mut rng, &mut used);
            }
        }

        Self { hash_map }
    }
    pub fn calc(&self, mut hash: usize, row: usize, col: usize, dir: char) -> usize {
        let N = self.hash_map.len();
        hash ^= self.hash_map[row][col];
        if dir == 'L' {
            if col > 0 {
                hash ^= self.hash_map[row][col - 1];
            }
        } else if dir == 'R' {
            if col + 1 < N {
                hash ^= self.hash_map[row][col + 1];
            }
        } else if dir == 'U' {
            if row > 0 {
                hash ^= self.hash_map[row - 1][col];
            }
        } else if dir == 'D' {
            if row + 1 < N {
                hash ^= self.hash_map[row + 1][col];
            }
        }
        hash
    }
}
