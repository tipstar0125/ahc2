#![allow(non_snake_case)]
#![allow(dead_code)]

mod common;
mod input;

use common::get_time;
use input::read_input;

fn main() {
    get_time();
    let input = read_input();

    eprintln!("Elapsed: {}", get_time());
}
