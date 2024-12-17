use std::collections::HashSet;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

#[derive(Debug)]
pub struct CalcHash {
    pub MAX: usize,
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
    pub fn new(width_limit: i32) -> Self {
        let mut rng = Pcg64Mcg::new(20);
        let mut used = HashSet::new();
        let MAX = 15;
        let L = width_limit as usize / 1e4 as usize + 5;
        let mut hash_map = vec![vec![!0; L]; MAX];
        for i in 0..MAX {
            for l in 0..L {
                hash_map[i][l] = gen_not_used(&mut rng, &mut used);
            }
        }

        Self { MAX, hash_map }
    }
    pub fn calc(&self, mut hash: usize, row: usize, before_width: i32, after_width: i32) -> usize {
        let bw = before_width as usize / 1e4 as usize;
        let aw = after_width as usize / 1e4 as usize;
        hash ^= self.hash_map[row][bw];
        hash ^= self.hash_map[row][aw];
        hash
    }
}
