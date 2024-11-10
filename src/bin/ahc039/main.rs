#![allow(non_snake_case)]
#![allow(dead_code)]

mod common;
mod coord;
mod input;
mod kmeans;
mod test;

use common::get_time;
use input::{read_input, Input};
use rand_pcg::Pcg64Mcg;

use crate::kmeans::KMeans;

fn solve(input: &Input) {
    let mut rng = Pcg64Mcg::new(100);
    let num = 5;
    let kmeans = KMeans::new(num, &input, &mut rng);
    for i in 0..num {
        eprintln!("num: {}", i);
        let square = kmeans.calc_good_square(i, &input);
        if square.is_none() {
            continue;
        }
        let square = square.unwrap();
        eprintln!("center: {}", kmeans.centers[i]);
        println!("{}", square.len());
        for coord in square.iter() {
            println!("{}", coord);
        }
        eprintln!();
    }
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
