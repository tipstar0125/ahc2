use std::collections::HashSet;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

#[derive(Debug)]
pub struct CalcHash {
    pub MAX: usize,
    pub hash_map: Vec<Vec<Vec<(usize, usize)>>>,
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
        let MAX = 15;
        let mut hash_map = vec![vec![vec![(!0, !0); N]; MAX]; MAX];
        for i in 0..MAX {
            for j in 0..MAX {
                for n in 0..N {
                    hash_map[i][j][n].0 = gen_not_used(&mut rng, &mut used);
                    hash_map[i][j][n].1 = gen_not_used(&mut rng, &mut used);
                }
            }
        }

        Self { MAX, hash_map }
    }
    pub fn calc(&self, mut hash: usize, i: usize, j: usize, n: usize, rot: bool) -> usize {
        if rot {
            hash ^= self.hash_map[i][j][n].0;
        } else {
            hash ^= self.hash_map[i][j][n].1;
        }
        hash
    }
}
