#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input_interactive;
use rand::prelude::*;
use rustc_hash::FxHashSet;
use std::cell::Cell;

fn main() {
    get_time();
    let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(849032);
    let input = read_input();
    let g = estimate(&input, &mut rng);
    let mut cands = vec![];
    for i in 0..input.N {
        for j in i + 1..input.N {
            if g[i * input.N + j] <= 700 {
                cands.push((i, j));
            }
        }
    }
    let mut dist = vec![-1; input.N];
    let mut vss = random_prim(
        &g,
        input.N,
        &(0..input.N).collect_vec(),
        &mut dist,
        &input.G,
        &mut rng,
    );
    let mut gs = vec![0; input.N];
    for i in 0..input.M {
        for &v in &vss[i].1 {
            gs[v] = i;
        }
    }
    let mut crt = vss.iter().map(|v| v.0).sum::<i32>();
    let mut max = crt;
    let mut best = vss.clone();
    eprintln!("{:.3}: {}", get_time(), crt);
    let stime = get_time();
    const T0: f64 = 1e2;
    const T1: f64 = 1e1;
    for iter in 0.. {
        let t = (get_time() - stime) / (1.95 - stime);
        if t >= 1.0 {
            eprintln!("!log iter {iter}");
            break;
        }
        let T = T0.powf(1.0 - t) * T1.powf(t);
        let (u, v) = cands[rng.gen_range(0..cands.len())];
        let i1 = gs[u];
        let i2 = gs[v];
        if i1 == i2 {
            continue;
        }
        let mut gain = vss[i1].0 + vss[i2].0;
        let coin = rng.gen_range(0..2);
        if coin == 0 {
            let vss2 = random_prim(
                &g,
                input.N,
                &vss[i1].1.iter().chain(&vss[i2].1).copied().collect(),
                &mut dist,
                &[vss[i1].1.len(), vss[i2].1.len()],
                &mut rng,
            );
            let w1 = vss2[0].0;
            let w2 = vss2[1].0;
            gain -= w1 + w2;
            if gain >= 0 || rng.gen_bool((gain as f64 / T).exp()) {
                crt -= gain;
                for &v in &vss2[0].1 {
                    gs[v] = i1;
                }
                for &v in &vss2[1].1 {
                    gs[v] = i2;
                }
                vss[i1] = vss2[0].clone();
                vss[i2] = vss2[1].clone();
                if max.setmin(crt) {
                    best = vss.clone();
                    eprintln!("{:.3}: {}", get_time(), crt);
                }
            }
        } else {
            if let Some(i3) = (0..input.M)
                .filter(|&i3| vss[i3].1.len() == vss[i1].1.len() + vss[i2].1.len())
                .choose(&mut rng)
            {
                gain += vss[i3].0;
                let vss2 = random_prim(
                    &g,
                    input.N,
                    &vss[i3].1,
                    &mut dist,
                    &[vss[i1].1.len(), vss[i2].1.len()],
                    &mut rng,
                );
                let vs1 = vss[i1].1.iter().chain(&vss[i2].1).copied().collect();
                let w1 =
                    random_prim(&g, input.N, &vs1, &mut dist, &[vss[i3].1.len()], &mut rng)[0].0;
                let w2 = vss2[0].0;
                let w3 = vss2[1].0;
                gain -= w1 + w2 + w3;
                if gain >= 0 || rng.gen_bool((gain as f64 / T).exp()) {
                    crt -= gain;
                    for &v in &vs1 {
                        gs[v] = i1;
                    }
                    for &v in &vss2[0].1 {
                        gs[v] = i2;
                    }
                    for &v in &vss2[1].1 {
                        gs[v] = i3;
                    }
                    vss[i1] = (w1, vs1);
                    vss[i2] = vss2[0].clone();
                    vss[i3] = vss2[1].clone();
                    if max.setmin(crt) {
                        best = vss.clone();
                        eprintln!("{:.3}: {}", get_time(), crt);
                    }
                }
            }
        }
    }
    vss = best;
    let mut out = vec![];
    let mut dist = vec![(0, !0); input.N];
    for vs in &vss {
        out.extend(prim_es(&g, input.N, &vs.1, &mut dist));
    }
    write_output(&input, &out);
    eprintln!("Time = {:.3}", get_time());
}

fn estimate(input: &Input, rng: &mut rand_pcg::Pcg64Mcg) -> Vec<i32> {
    let mut ps = (0..input.N)
        .map(|i| {
            (
                (input.xs[i].0 + input.xs[i].1) / 2,
                (input.ys[i].0 + input.ys[i].1) / 2,
            )
        })
        .collect_vec();
    // (a,b,c,d): dist(a, b) <= dist(c, d)
    let mut ineqs = vec![];
    let mut order = (0..input.N).collect_vec();
    order.sort_by_key(|&i| input.xs[i].1 - input.xs[i].0 + input.ys[i].1 - input.ys[i].0);
    order.reverse();
    let mut done = FxHashSet::default();
    let mut q = 0;
    let mut cnt = vec![0; input.N];
    let mut used = mat![false; input.N; input.N];
    for c in order {
        let mut cand = (0..input.N).map(|i| (dist(ps[c], ps[i]), i)).collect_vec();
        cand.sort();
        let mut cs: Vec<usize> = vec![];
        for (_, i) in cand {
            if cs.iter().all(|&j| !used[i][j]) {
                cs.push(i);
                if cs.len() == input.L {
                    break;
                }
            }
        }
        assert!(cs.len() == input.L);
        if !done.insert(cs.clone()) {
            continue;
        }
        q += 1;
        let es = query(&cs);
        let mut adj = vec![vec![]; input.N];
        for &(i, j) in &es {
            adj[i].push(j);
            adj[j].push(i);
            used[i][j] = true;
            used[j][i] = true;
        }
        for i in 0..input.L {
            let u = cs[i];
            cnt[u] += 1;
            for j in i + 1..input.L {
                let v = cs[j];
                if adj[u].contains(&v) {
                    continue;
                }
                // MSTに含まれない辺 uv に対し、uvパスを求める。
                let path = find_path(&adj, u, v, !0).unwrap();
                for k in 0..path.len() - 1 {
                    ineqs.push((
                        path[k].min(path[k + 1]),
                        path[k].max(path[k + 1]),
                        u.min(v),
                        u.max(v),
                    ));
                }
            }
        }
        if q == input.Q {
            break;
        }
    }
    eprintln!("!log q {}", q);
    eprintln!("!log ineqs0 {}", ineqs.len());
    ineqs.sort();
    ineqs.dedup();
    let pss = (0..input.N)
        .map(|i| {
            (0..20)
                .map(|_| {
                    (
                        rng.gen_range(input.xs[i].0..=input.xs[i].1),
                        rng.gen_range(input.ys[i].0..=input.ys[i].1),
                    )
                })
                .collect_vec()
        })
        .collect_vec();
    // 違反確率の低い不等式は消す
    ineqs.retain(|&(a, b, c, d)| {
        (0..20).any(|i| dist(pss[a][i], pss[b][i]) > dist(pss[c][i], pss[d][i]))
    });
    eprintln!("!log ineqs {}", ineqs.len());
    let mut ids = vec![vec![]; input.N];
    for (k, &(a, b, c, d)) in ineqs.iter().enumerate() {
        ids[a].push(k);
        ids[b].push(k);
        ids[c].push(k);
        ids[d].push(k);
    }
    for i in 0..input.N {
        ids[i].sort();
        ids[i].dedup();
    }
    // 不等式の違反量が小さくなるように焼き鈍し
    let mut crt = 0;
    for &(a, b, c, d) in &ineqs {
        crt += (dist(ps[a], ps[b]) - dist(ps[c], ps[d])).max(0);
    }
    const T0: f64 = 1e4;
    const T1: f64 = 1e-1;
    let mut T = T0;
    for iter in 0.. {
        if iter & 0xff == 0 {
            let t = get_time() / 0.5;
            if t >= 1.0 {
                eprintln!("!log iter1 {}", iter);
                break;
            }
            T = T0.powf(1.0 - t) * T1.powf(t);
        }
        let u = rng.gen_range(0..input.N);
        let bk = ps[u];
        let (x, y) = if rng.gen_bool(0.5) {
            (
                rng.gen_range(input.xs[u].0.max(ps[u].0 - 100)..=input.xs[u].1.min(ps[u].0 + 100)),
                rng.gen_range(input.ys[u].0.max(ps[u].1 - 100)..=input.ys[u].1.min(ps[u].1 + 100)),
            )
        } else {
            (
                rng.gen_range(input.xs[u].0..=input.xs[u].1),
                rng.gen_range(input.ys[u].0..=input.ys[u].1),
            )
        };
        let mut diff = 0;
        for &k in &ids[u] {
            let (a, b, c, d) = ineqs[k];
            diff -= (dist(ps[a], ps[b]) - dist(ps[c], ps[d])).max(0);
        }
        ps[u] = (x, y);
        for &k in &ids[u] {
            let (a, b, c, d) = ineqs[k];
            diff += (dist(ps[a], ps[b]) - dist(ps[c], ps[d])).max(0);
        }
        if diff <= 0 || rng.gen_bool((-diff as f64 / T).exp()) {
            crt += diff;
            if crt == 0 {
                break;
            }
        } else {
            ps[u] = bk;
        }
    }
    eprintln!("!log diff1 {}", crt);
    let mut sum = vec![0; input.N * input.N];
    let mut den = 0;
    while get_time() < 1.0 {
        for i in 0..input.N {
            let mut diff = 0;
            for &k in &ids[i] {
                let (a, b, c, d) = ineqs[k];
                diff -= (dist(ps[a], ps[b]) - dist(ps[c], ps[d])).max(0);
            }
            let (mut x0, mut x1) = input.xs[i];
            let (mut y0, mut y1) = input.ys[i];
            for &k in &ids[i] {
                let (a, b, c, d) = ineqs[k];
                if (a == i || b == i) && c != i && d != i {
                    let r = dist(ps[a], ps[b]).max(dist(ps[c], ps[d]));
                    let (bx, by) = if a == i { ps[b] } else { ps[a] };
                    x0 = x0.max(bx - r);
                    x1 = x1.min(bx + r);
                    y0 = y0.max(by - r);
                    y1 = y1.min(by + r);
                }
            }
            let bk = ps[i];
            // 100回試して新しい点が見つからなかった場合は諦めて今のを採用
            for _ in 0..100 {
                let x = rng.gen_range(x0..=x1);
                let y = rng.gen_range(y0..=y1);
                ps[i] = (x, y);
                let mut diff = diff;
                for &k in &ids[i] {
                    let (a, b, c, d) = ineqs[k];
                    diff += (dist(ps[a], ps[b]) - dist(ps[c], ps[d])).max(0);
                }
                if diff <= 0 {
                    crt += diff;
                    break;
                }
                ps[i] = bk;
            }
        }
        for i in 0..input.N {
            for j in 0..i {
                let d = dist(ps[i], ps[j]) as i64;
                sum[i * input.N + j] += d;
                sum[j * input.N + i] += d;
            }
        }
        den += 1;
    }
    eprintln!("!log den {}", den);
    eprintln!(
        "!log diff {}",
        ineqs
            .iter()
            .map(|&(a, b, c, d)| (dist(ps[a], ps[b]) - dist(ps[c], ps[d])).max(0))
            .sum::<i32>()
    );
    sum.iter().map(|&d| (d / den) as i32).collect()
}

fn find_path(g: &Vec<Vec<usize>>, s: usize, t: usize, p: usize) -> Option<Vec<usize>> {
    if s == t {
        return Some(vec![s]);
    }
    for &u in &g[s] {
        if u == p {
            continue;
        }
        if let Some(mut path) = find_path(g, u, t, s) {
            path.push(s);
            return Some(path);
        }
    }
    None
}

fn random_prim(
    g: &Vec<i32>,
    n: usize,
    vs: &Vec<usize>,
    dist: &mut Vec<i32>,
    gs: &[usize],
    rng: &mut rand_pcg::Pcg64Mcg,
) -> Vec<(i32, Vec<usize>)> {
    for &v in vs {
        dist[v] = i32::MAX;
    }
    let mut gs = gs.to_vec();
    gs.shuffle(rng);
    let mut ret = vec![(0, vec![]); gs.len()];
    for k in 0..gs.len() {
        let s = *vs.iter().filter(|&&v| dist[v] >= 0).choose(rng).unwrap();
        ret[k].1.push(s);
        dist[s] = -1;
        for &v in vs {
            if dist[v] >= 0 {
                dist[v] = g[s * n + v];
            }
        }
        for _ in 1..gs[k] {
            let mut next = !0;
            let mut min = i32::MAX;
            for &v in vs {
                if dist[v] >= 0 && min.setmin(dist[v]) {
                    next = v;
                }
            }
            ret[k].0 += min;
            ret[k].1.push(next);
            dist[next] = -1;
            for &v in vs {
                dist[v].setmin(g[next * n + v]);
            }
        }
    }
    ret
}

fn prim_es(
    g: &Vec<i32>,
    n: usize,
    vs: &Vec<usize>,
    dist: &mut Vec<(i32, usize)>,
) -> Vec<(usize, usize)> {
    let mut out = vec![];
    let s = vs[0];
    for &v in vs {
        dist[v] = (g[s * n + v], s);
    }
    dist[s] = (-1, !0);
    for _ in 1..vs.len() {
        let mut next = !0;
        let mut min = (i32::MAX, !0);
        for &v in vs {
            if dist[v].0 >= 0 && min.setmin(dist[v]) {
                next = v;
            }
        }
        out.push((min.1, next));
        dist[next].0 = -1;
        for &v in vs {
            dist[v].setmin((g[next * n + v], next));
        }
    }
    out
}

// 入出力と得点計算

fn query(cs: &Vec<usize>) -> Vec<(usize, usize)> {
    println!("? {} {}", cs.len(), cs.iter().join(" "));
    input_interactive! {
        es: [(usize, usize); cs.len() - 1]
    }
    es
}

fn write_output(input: &Input, out: &Vec<(usize, usize)>) {
    println!("!");
    let mut uf = UnionFind::new(input.N);
    for &(i, j) in out {
        uf.unite(i, j);
    }
    let mut gid = vec![vec![]; input.N + 1];
    for i in 0..input.M {
        gid[input.G[i]].push(i);
    }
    let mut ids = vec![!0; input.N];
    for i in 0..input.N {
        if uf.find(i) == i {
            ids[i] = gid[uf.size(i)].pop().unwrap();
        }
    }
    let mut vs = vec![vec![]; input.M];
    let mut es = vec![vec![]; input.M];
    for i in 0..input.N {
        vs[ids[uf.find(i)]].push(i);
    }
    for &(i, j) in out {
        let g = ids[uf.find(i)];
        es[g].push((i, j));
    }
    for g in 0..input.M {
        println!("{}", vs[g].iter().join(" "));
        for &(i, j) in &es[g] {
            println!("{} {}", i, j);
        }
    }
}

fn dist(p: (i32, i32), q: (i32, i32)) -> i32 {
    (((p.0 - q.0) * (p.0 - q.0) + (p.1 - q.1) * (p.1 - q.1)) as f64).sqrt() as i32
}

#[allow(unused)]
struct Input {
    N: usize,
    M: usize,
    Q: usize,
    L: usize,
    W: i32,
    G: Vec<usize>,
    xs: Vec<(i32, i32)>,
    ys: Vec<(i32, i32)>,
}

fn read_input() -> Input {
    input_interactive! {
        N: usize,
        M: usize,
        Q: usize,
        L: usize,
        W: i32,
        G: [usize; M],
        ps: [(i32, i32, i32, i32); N],
    }
    let xs = ps.iter().map(|&(x1, x2, _, _)| (x1, x2)).collect();
    let ys = ps.iter().map(|&(_, _, y1, y2)| (y1, y2)).collect();
    Input {
        N,
        M,
        Q,
        L,
        W,
        G,
        xs,
        ys,
    }
}

// ここからライブラリ

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { vec![$($e),*] };
	($($e:expr,)*) => { vec![$($e),*] };
	($e:expr; $d:expr) => { vec![$e; $d] };
	($e:expr; $d:expr $(; $ds:expr)+) => { vec![mat![$e $(; $ds)*]; $d] };
}

pub fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        // ローカル環境とジャッジ環境の実行速度差はget_timeで吸収しておくと便利
        #[cfg(feature = "local")]
        {
            (ms - STIME) * 1.0
        }
        #[cfg(not(feature = "local"))]
        {
            ms - STIME
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnionFind {
    /// size / parent
    ps: Vec<Cell<usize>>,
    pub is_root: Vec<bool>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            ps: vec![Cell::new(1); n],
            is_root: vec![true; n],
        }
    }
    pub fn find(&self, x: usize) -> usize {
        if self.is_root[x] {
            x
        } else {
            let p = self.find(self.ps[x].get());
            self.ps[x].set(p);
            p
        }
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return;
        }
        if self.ps[x].get() < self.ps[y].get() {
            ::std::mem::swap(&mut x, &mut y);
        }
        *self.ps[x].get_mut() += self.ps[y].get();
        self.ps[y].set(x);
        self.is_root[y] = false;
    }
    pub fn same(&self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    pub fn size(&self, x: usize) -> usize {
        self.ps[self.find(x)].get()
    }
}
