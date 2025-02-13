use std::collections::HashSet;

use rand::Rng;
use rand_pcg::Pcg64Mcg;

#[derive(Debug)]
pub struct CalcHash {
    pub hash_map: Vec<usize>,
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
    pub fn new() -> Self {
        let mut rng = Pcg64Mcg::new(0);
        let mut used: HashSet<usize> = HashSet::new();
        let mut hash_map = vec![];

        Self { hash_map }
    }
    pub fn calc(&self, field: &Vec<Vec<char>>) -> usize {
        let mut ret = 0;
        ret
    }
}
