use std::collections::HashSet;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

use crate::coord::Coord;

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
        let mut rng = Pcg64Mcg::new(0);
        let mut used: HashSet<usize> = HashSet::new();
        let mut hash_map = vec![vec![!0; N]; N];
        for i in 0..N {
            for j in 0..N {
                hash_map[i][j] = gen_not_used(&mut rng, &mut used);
            }
        }

        Self { hash_map }
    }
    pub fn calc(&self, hash: usize, pos: Coord) -> usize {
        hash ^ self.hash_map[pos.i][pos.j]
    }
}
