use rand::Rng;
use rand_pcg::Pcg64Mcg;
use rustc_hash::FxHashSet;

use crate::{
    coord::{calc_dist, Coord},
    input::Input,
};

pub struct KMeans {
    pub num: usize,
    pub centers: Vec<Coord>,
    pub clusters: Vec<FxHashSet<usize>>,
}

impl KMeans {
    pub fn new(num: usize, input: &Input, rng: &mut Pcg64Mcg) -> Self {
        let mut centers = vec![];
        for _ in 0..num {
            let x = rng.gen_range(0..=input.size);
            let y = rng.gen_range(0..=input.size);
            centers.push(Coord::new(x, y));
        }
        let mut clusters = vec![FxHashSet::default(); num];
        for (saba_idx, coord) in input.saba.iter().enumerate() {
            let mut cands = vec![];
            for (idx, center) in centers.iter().enumerate() {
                let dist = calc_dist(&coord, &center);
                cands.push((dist, idx));
            }
            cands.sort();
            let best_idx = cands[0].1;
            clusters[best_idx].insert(saba_idx);
        }

        loop {
            let mut next_centers = vec![];
            for cluster in clusters.iter() {
                let length = cluster.len();
                if length == 0 {
                    continue;
                }
                let cx = cluster
                    .iter()
                    .fold(0, |sum, &saba_idx| sum + input.saba[saba_idx].x)
                    / length;
                let cy = cluster
                    .iter()
                    .fold(0, |sum, &saba_idx| sum + input.saba[saba_idx].y)
                    / length;
                next_centers.push(Coord::new(cx, cy));
            }

            let mut next_clusters = vec![FxHashSet::default(); num];
            for (saba_idx, coord) in input.saba.iter().enumerate() {
                let mut cands = vec![];
                for (idx, center) in next_centers.iter().enumerate() {
                    let dist = calc_dist(&coord, &center);
                    cands.push((dist, idx));
                }
                cands.sort();
                let best_idx = cands[0].1;
                next_clusters[best_idx].insert(saba_idx);
            }
            if clusters == next_clusters {
                break;
            }
            centers = next_centers;
            clusters = next_clusters;
        }

        Self {
            num,
            centers,
            clusters,
        }
    }
    pub fn calc_square(&self, cluster_num: usize, input: &Input) -> Option<[Coord; 4]> {
        if self.clusters[cluster_num].is_empty() {
            return None;
        }
        let xmin = self.clusters[cluster_num]
            .iter()
            .map(|&idx| input.saba[idx].x)
            .min()
            .unwrap();
        let ymin = self.clusters[cluster_num]
            .iter()
            .map(|&idx| input.saba[idx].y)
            .min()
            .unwrap();
        let xmax = self.clusters[cluster_num]
            .iter()
            .map(|&idx| input.saba[idx].x)
            .max()
            .unwrap();
        let ymax = self.clusters[cluster_num]
            .iter()
            .map(|&idx| input.saba[idx].y)
            .max()
            .unwrap();
        Some([
            Coord::new(xmin, ymin),
            Coord::new(xmax, ymin),
            Coord::new(xmax, ymax),
            Coord::new(xmin, ymax),
        ])
    }
    pub fn calc_good_square(&self, cluster_num: usize, input: &Input) -> Option<[Coord; 4]> {
        if self.clusters[cluster_num].is_empty() {
            return None;
        }

        let squre_limit = self.calc_square(cluster_num, input).unwrap();

        let delta = 500;
        let mut best_score = 0;
        let mut best_square = [
            Coord::new(!0, !0),
            Coord::new(!0, !0),
            Coord::new(!0, !0),
            Coord::new(!0, !0),
        ];
        let Coord {
            x: mut cx,
            y: mut cy,
        } = self.centers[cluster_num];
        let mut cnt = 0;

        cx /= delta * 2;
        cx *= delta * 2;
        cy /= delta * 2;
        cy *= delta * 2;

        loop {
            cnt += 1;
            let mut next_square = [
                Coord::new(!0, !0),
                Coord::new(!0, !0),
                Coord::new(!0, !0),
                Coord::new(!0, !0),
            ];
            for i in 0..4 {
                let mut nx = !0;
                let mut ny = !0;
                if i == 0 {
                    nx = if cx - delta * cnt > input.size {
                        0
                    } else {
                        cx - delta * cnt
                    };
                    ny = if cy - delta * cnt > input.size {
                        0
                    } else {
                        cy - delta * cnt
                    };
                } else if i == 1 {
                    nx = if cx + delta * cnt > input.size {
                        input.size
                    } else {
                        cx + delta * cnt
                    };
                    ny = if cy - delta * cnt > input.size {
                        0
                    } else {
                        cy - delta * cnt
                    };
                } else if i == 2 {
                    nx = if cx + delta * cnt > input.size {
                        input.size
                    } else {
                        cx + delta * cnt
                    };
                    ny = if cy + delta * cnt > input.size {
                        input.N
                    } else {
                        cy + delta * cnt
                    };
                } else {
                    nx = if cx - delta * cnt > input.size {
                        0
                    } else {
                        cx - delta * cnt
                    };
                    ny = if cy + delta * cnt > input.size {
                        input.N
                    } else {
                        cy + delta * cnt
                    };
                }
                next_square[i] = Coord::new(nx, ny);
            }

            let mut saba_cnt = 0;
            for coord in input.saba.iter() {
                if next_square[0].x <= coord.x
                    && coord.x <= next_square[2].x
                    && next_square[0].y <= coord.y
                    && coord.y <= next_square[2].y
                {
                    saba_cnt += 1;
                }
            }
            let mut iwashi_cnt = 0;
            for coord in input.iwashi.iter() {
                if next_square[0].x <= coord.x
                    && coord.x <= next_square[2].x
                    && next_square[0].y <= coord.y
                    && coord.y <= next_square[2].y
                {
                    iwashi_cnt += 1;
                }
            }
            let score = if saba_cnt > iwashi_cnt {
                saba_cnt - iwashi_cnt
            } else {
                0
            };

            if best_score == 0 {
                best_square = next_square.clone();
            }
            if score > best_score {
                best_score = score;
                best_square = next_square;
            }

            if next_square[0].x <= squre_limit[0].x
                || next_square[0].y <= squre_limit[0].y
                || next_square[2].x >= squre_limit[2].x
                || next_square[2].y >= squre_limit[2].y
            {
                break;
            }
        }
        eprintln!("score: {}", best_score);
        Some(best_square)
    }
}
