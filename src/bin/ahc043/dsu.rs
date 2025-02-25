use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<isize>,
    roots: BTreeSet<usize>,
    size: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut roots = BTreeSet::new();
        for i in 0..n {
            roots.insert(i);
        }
        UnionFind {
            parent: vec![-1; n],
            roots,
            size: n,
        }
    }
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] < 0 {
            return x;
        }
        let root = self.find(self.parent[x] as usize);
        self.parent[x] = root as isize;
        root
    }
    pub fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return None;
        }
        let size_x = -self.parent[root_x];
        let size_y = -self.parent[root_y];
        self.size -= 1;
        if size_x >= size_y {
            self.parent[root_x] -= size_y;
            self.parent[root_y] = root_x as isize;
            self.roots.remove(&root_y);
            Some((root_x, root_y))
        } else {
            self.parent[root_y] -= size_x;
            self.parent[root_x] = root_y as isize;
            self.roots.remove(&root_x);
            Some((root_y, root_x))
        }
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn is_root(&mut self, x: usize) -> bool {
        self.find(x) == x
    }
    pub fn get_union_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        -self.parent[root] as usize
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn members(&mut self, x: usize) -> Vec<usize> {
        let root = self.find(x);
        (0..self.parent.len())
            .filter(|i| self.find(*i) == root)
            .collect::<Vec<usize>>()
    }
    pub fn all_group_members(&mut self) -> BTreeMap<usize, Vec<usize>> {
        let mut groups_map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for x in 0..self.parent.len() {
            let r = self.find(x);
            groups_map.entry(r).or_default().push(x);
        }
        groups_map
    }
}
