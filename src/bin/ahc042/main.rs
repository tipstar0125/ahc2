#![allow(non_snake_case)]
#![allow(dead_code)]

use input::Input;

use crate::{common::get_time, input::read_input};

mod common;
mod input;
mod test;

fn solve(input: &Input) {
    let mut C = input.C.clone();
    let mut cnt = 2 * input.N;
    let mut score = 8 * input.N * input.N;
    let mut ans = vec![];

    while cnt > 0 {
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
                    candidates.push((c as f64 / num as f64, c, num, 'L', i));
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
                    candidates.push((c as f64 / num as f64, c, num, 'R', i));
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
                    candidates.push((c as f64 / num as f64, c, num, 'U', j));
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
                    candidates.push((c as f64 / num as f64, c, num, 'D', j));
                }
            }
        }
        if candidates.is_empty() {
            break;
        }
        candidates.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let (_, c, num, dir, idx) = candidates[0];
        cnt -= c;

        if dir == 'L' {
            for _ in 0..num {
                ans.push(format!("L {}", idx));
                score -= 1;
                for j in 1..input.N {
                    C[idx][j - 1] = C[idx][j];
                }
                C[idx][input.N - 1] = '.';
            }

            while !validate(&C) {
                ans.push(format!("R {}", idx));
                score -= 1;
                for j in (1..input.N).rev() {
                    C[idx][j] = C[idx][j - 1];
                }
                C[idx][0] = '.';
            }
        } else if dir == 'R' {
            for _ in 0..num {
                ans.push(format!("R {}", idx));
                score -= 1;
                for j in (1..input.N).rev() {
                    C[idx][j] = C[idx][j - 1];
                }
                C[idx][0] = '.';
            }

            while !validate(&C) {
                ans.push(format!("L {}", idx));
                score -= 1;
                for j in 1..input.N {
                    C[idx][j - 1] = C[idx][j];
                }
                C[idx][input.N - 1] = '.';
            }
        } else if dir == 'U' {
            for _ in 0..num {
                ans.push(format!("U {}", idx));
                score -= 1;
                for i in 1..input.N {
                    C[i - 1][idx] = C[i][idx];
                }
                C[input.N - 1][idx] = '.';
            }

            while !validate(&C) {
                ans.push(format!("D {}", idx));
                score -= 1;
                for i in (1..input.N).rev() {
                    C[i][idx] = C[i - 1][idx];
                }
                C[0][idx] = '.';
            }
        } else if dir == 'D' {
            for _ in 0..num {
                ans.push(format!("D {}", idx));
                score -= 1;
                for i in (1..input.N).rev() {
                    C[i][idx] = C[i - 1][idx];
                }
                C[0][idx] = '.';
            }

            while !validate(&C) {
                ans.push(format!("U {}", idx));
                score -= 1;
                for i in 1..input.N {
                    C[i - 1][idx] = C[i][idx];
                }
                C[input.N - 1][idx] = '.';
            }
        }
    }

    if cnt > 0 {
        for i in 0..input.N {
            if !C[i].contains(&'o') && C[i].contains(&'x') {
                let x_pos_left = C[i].iter().position(|&c| c == 'x').unwrap();
                let x_pos_right = C[i].iter().rposition(|&c| c == 'x').unwrap();
                let num_left = input.N - x_pos_left;
                let num_right = x_pos_right + 1;
                if num_left < num_right {
                    for _ in 0..num_left {
                        ans.push(format!("R {}", i));
                        score -= 1;
                    }
                } else {
                    for _ in 0..num_right {
                        ans.push(format!("L {}", i));
                        score -= 1;
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
                        ans.push(format!("D {}", j));
                        score -= 1;
                    }
                } else {
                    for _ in 0..num_down {
                        ans.push(format!("U {}", j));
                        score -= 1;
                    }
                }
                for i in 0..input.N {
                    C[i][j] = '.';
                }
            }
        }
    }
    for a in ans {
        println!("{}", a);
    }

    eprintln!("score = {}", score);
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
