#![allow(non_snake_case)]
#![allow(dead_code)]

use std::cmp::Reverse;

use input::Input;

use crate::{common::get_time, input::read_input};

mod common;
mod input;
mod test;

fn solve(input: &Input) {
    let mut C = input.C.clone();

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
            for i in 0..input.N {
                C[i][j] = '.';
            }
        }
    }

    let mut x_cnt = 0;

    for i in 0..input.N {
        for j in 0..input.N {
            if C[i][j] == 'x' {
                x_cnt += 1;
            }
        }
    }
    eprintln!("x_cnt = {}", x_cnt);

    let mut cnt = 0;
    // eprintln!("validate = {}", validate(&C));

    while cnt < x_cnt {
        let mut candidates = vec![];
        for i in 0..input.N {
            if let Some(o_pos) = C[i].iter().position(|&c| c == 'o') {
                if let Some(x_pos) = C[i][..o_pos].iter().rposition(|&c| c == 'x') {
                    let mut c = 0;
                    for j in 0..o_pos {
                        if C[i][j] == 'x' {
                            c += 1;
                        }
                    }
                    let num = x_pos + 1;
                    candidates.push((c, Reverse(num), 'L', i));
                }
            }
        }

        for i in 0..input.N {
            if let Some(o_pos) = C[i].iter().rposition(|&c| c == 'o') {
                if let Some(x_pos) = C[i][o_pos + 1..].iter().position(|&c| c == 'x') {
                    let mut c = 0;
                    for j in o_pos + 1..input.N {
                        if C[i][j] == 'x' {
                            c += 1;
                        }
                    }
                    let num = input.N - (x_pos + o_pos + 1);
                    candidates.push((c, Reverse(num), 'R', i));
                }
            }
        }

        for j in 0..input.N {
            if let Some(o_pos) = C.iter().position(|row| row[j] == 'o') {
                if let Some(x_pos) = C[..o_pos].iter().rposition(|row| row[j] == 'x') {
                    let mut c = 0;
                    for i in 0..o_pos {
                        if C[i][j] == 'x' {
                            c += 1;
                        }
                    }
                    let num = x_pos + 1;
                    candidates.push((c, Reverse(num), 'U', j));
                }
            }
        }

        for j in 0..input.N {
            if let Some(o_pos) = C.iter().rposition(|row| row[j] == 'o') {
                if let Some(x_pos) = C[o_pos + 1..].iter().position(|row| row[j] == 'x') {
                    let mut c = 0;
                    for i in o_pos + 1..input.N {
                        if C[i][j] == 'x' {
                            c += 1;
                        }
                    }
                    let num = input.N - (x_pos + o_pos + 1);
                    candidates.push((c, Reverse(num), 'D', j));
                }
            }
        }
        if candidates.is_empty() {
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
                    for i in 0..input.N {
                        C[i][j] = '.';
                    }
                }
            }
            break;
        }
        candidates.sort();
        candidates.reverse();

        let (c, Reverse(num), dir, idx) = candidates[0];
        // eprintln!("c = {}, num = {}, dir = {}, idx = {}", c, num, dir, idx);
        cnt += c;

        if dir == 'L' {
            for _ in 0..num {
                println!("L {}", idx);
                for j in 1..input.N {
                    C[idx][j - 1] = C[idx][j];
                }
                C[idx][input.N - 1] = '.';
            }

            if !validate(&C) {
                for _ in 0..num {
                    println!("R {}", idx);
                    for j in (1..input.N).rev() {
                        C[idx][j] = C[idx][j - 1];
                    }
                    C[idx][0] = '.';
                }
            }
        } else if dir == 'R' {
            for _ in 0..num {
                println!("R {}", idx);
                for j in (1..input.N).rev() {
                    C[idx][j] = C[idx][j - 1];
                }
                C[idx][0] = '.';
            }

            if !validate(&C) {
                for _ in 0..num {
                    println!("L {}", idx);
                    for j in 1..input.N {
                        C[idx][j - 1] = C[idx][j];
                    }
                    C[idx][input.N - 1] = '.';
                }
            }
        } else if dir == 'U' {
            for _ in 0..num {
                println!("U {}", idx);
                for i in 1..input.N {
                    C[i - 1][idx] = C[i][idx];
                }
                C[input.N - 1][idx] = '.';
            }

            if !validate(&C) {
                for _ in 0..num {
                    println!("D {}", idx);
                    for i in (1..input.N).rev() {
                        C[i][idx] = C[i - 1][idx];
                    }
                    C[0][idx] = '.';
                }
            }
        } else if dir == 'D' {
            for _ in 0..num {
                println!("D {}", idx);
                for i in (1..input.N).rev() {
                    C[i][idx] = C[i - 1][idx];
                }
                C[0][idx] = '.';
            }

            if !validate(&C) {
                for _ in 0..num {
                    println!("U {}", idx);
                    for i in 1..input.N {
                        C[i - 1][idx] = C[i][idx];
                    }
                    C[input.N - 1][idx] = '.';
                }
            }
        }

        // eprintln!(
        //     "{}",
        //     C.iter()
        //         .map(|row| row.iter().collect::<String>())
        //         .collect::<Vec<String>>()
        //         .join("\n")
        // );
    }
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}

fn validate(C: &Vec<Vec<char>>) -> bool {
    let N = C.len();
    for i in 0..N {
        for j in 0..N {
            if C[i][j] == 'x' {
                let mut ok = false;

                let mut ok2 = true;
                for k in 0..j {
                    if C[i][k] == 'o' {
                        ok2 = false;
                        break;
                    }
                }
                ok |= ok2;

                let mut ok2 = true;
                for k in j + 1..N {
                    if C[i][k] == 'o' {
                        ok2 = false;
                        break;
                    }
                }
                ok |= ok2;

                let mut ok2 = true;
                for k in 0..i {
                    if C[k][j] == 'o' {
                        ok2 = false;
                        break;
                    }
                }
                ok |= ok2;

                let mut ok2 = true;
                for k in i + 1..N {
                    if C[k][j] == 'o' {
                        ok2 = false;
                        break;
                    }
                }
                ok |= ok2;

                if !ok {
                    return false;
                }
            }
        }
    }
    true
}
