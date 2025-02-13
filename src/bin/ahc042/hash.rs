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
    pub fn calc(&self, field: &Vec<Vec<char>>) -> usize {
        let mut ret = 0;
        for i in 0..field.len() {
            for j in 0..field[i].len() {
                if field[i][j] == 'x' {
                    ret ^= self.hash_map[i][j];
                }
            }
        }
        ret
    }
}
