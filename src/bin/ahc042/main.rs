#![allow(non_snake_case)]
#![allow(dead_code)]

use input::Input;

use crate::{common::get_time, input::read_input};

mod common;
mod input;
mod test;

fn solve(input: &Input) {
    let mut C = input.C.clone();
    for i in 0..input.N {
        if let Some(o_pos) = C[i].iter().position(|&c| c == 'o') {
            if let Some(x_pos) = C[i][..o_pos].iter().rposition(|&c| c == 'x') {
                let num = x_pos + 1;
                for _ in 0..num {
                    println!("L {}", i)
                }
                for _ in 0..num {
                    println!("R {}", i)
                }
                for j in 0..=x_pos {
                    C[i][j] = '.';
                }
            }
        }
    }

    for i in 0..input.N {
        if let Some(o_pos) = C[i].iter().rposition(|&c| c == 'o') {
            if let Some(x_pos) = C[i][o_pos + 1..].iter().position(|&c| c == 'x') {
                let num = input.N - (x_pos + o_pos + 1);
                for _ in 0..num {
                    println!("R {}", i);
                }
                for _ in 0..num {
                    println!("L {}", i);
                }
                for j in o_pos + 1..input.N {
                    C[i][j] = '.';
                }
            }
        }
    }

    for i in 0..input.N {
        if !C[i].contains(&'o') && C[i].contains(&'x') {
            let x_pos_left = C[i].iter().position(|&c| c == 'x').unwrap();
            let x_pos_right = C[i].iter().rposition(|&c| c == 'x').unwrap();
            let num_left = input.N - x_pos_left;
            let num_right = x_pos_right + 1;
            if num_left < num_right {
                for _ in 0..num_left {
                    println!("R {}", i);
                }
            } else {
                for _ in 0..num_right {
                    println!("L {}", i);
                }
            }
            for j in 0..input.N {
                C[i][j] = '.';
            }
        }
    }

    for j in 0..input.N {
        if let Some(o_pos) = C.iter().position(|row| row[j] == 'o') {
            if let Some(x_pos) = C[..o_pos].iter().rposition(|row| row[j] == 'x') {
                let num = x_pos + 1;
                for _ in 0..num {
                    println!("U {}", j)
                }
                for _ in 0..num {
                    println!("D {}", j)
                }
            }
        }
    }

    for j in 0..input.N {
        if let Some(o_pos) = C.iter().rposition(|row| row[j] == 'o') {
            if let Some(x_pos) = C[o_pos + 1..].iter().position(|row| row[j] == 'x') {
                let num = input.N - (x_pos + o_pos + 1);
                for _ in 0..num {
                    println!("D {}", j);
                }
            }
        }
    }

    for j in 0..input.N {
        if !C.iter().any(|row| row[j] == 'o') && C.iter().any(|row| row[j] == 'x') {
            let x_pos_up = C.iter().position(|row| row[j] == 'x').unwrap();
            let x_pos_down = C.iter().rposition(|row| row[j] == 'x').unwrap();
            let num_up = input.N - x_pos_up;
            let num_down = x_pos_down + 1;
            if num_up < num_down {
                for _ in 0..num_up {
                    println!("D {}", j);
                }
            } else {
                for _ in 0..num_down {
                    println!("U {}", j);
                }
            }
        }
    }
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
