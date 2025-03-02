#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{collections::VecDeque, vec};

use input::Input;

use crate::{
    common::get_time,
    coord::{calc_manhattan_dist, Coord, DIJ4},
    input::read_input,
};

mod common;
mod coord;
mod input;
mod test;

fn solve(input: &Input) {
    let mut good_field = vec![vec![false; input.N]; input.N];
    let mut K = 0;
    for i in 0..input.N {
        for j in 0..input.N {
            if (i % 2) == (input.start.i % 2) || j == input.start.j {
                good_field[i][j] = true;
            }
            if good_field[i][j] {
                eprint!("o");
            } else {
                eprint!("x");
            }
            if input.C[i][j] == 'a' {
                K += 1;
            }
        }
        eprintln!();
    }

    let mut ng_coords = vec![];
    let mut empty_coords_for_stone = vec![];
    let mut empty_coords_for_wall = vec![];
    for i in 0..input.N {
        for j in 0..input.N {
            if input.C[i][j] == 'a' && !good_field[i][j] {
                ng_coords.push(Coord::new(i, j));
            }
            if input.C[i][j] == '@' && good_field[i][j] {
                ng_coords.push(Coord::new(i, j));
            }
            if input.C[i][j] == '.' && good_field[i][j] {
                empty_coords_for_stone.push(Coord::new(i, j));
            }
            if input.C[i][j] == '.' && !good_field[i][j] {
                empty_coords_for_wall.push(Coord::new(i, j));
            }
        }
    }

    eprintln!("start: {}", input.start);
    let mut C = input.C.clone();
    let mut ng_cnt = 0;
    loop {
        let mut used_ng_coords = vec![false; ng_coords.len()];
        let mut used_empty_coords_for_stone = vec![false; empty_coords_for_stone.len()];
        let mut used_empty_coords_for_wall = vec![false; empty_coords_for_wall.len()];
        let mut field = C.clone();
        let mut now = input.start;

        for i in 0..input.N {
            for j in 0..input.N {
                eprint!("{}", input.C[i][j]);
            }
            eprint!("     ");
            for j in 0..input.N {
                eprint!("{}", field[i][j]);
            }
            eprintln!();
        }
        eprintln!();

        while !used_ng_coords.iter().all(|&b| b) {
            let mut cand = vec![];

            for i in 0..ng_coords.len() {
                if used_ng_coords[i] {
                    continue;
                }
                let next = ng_coords[i];
                let dist = calc_manhattan_dist(now, next);
                cand.push((dist, next, i));
            }
            cand.sort();
            let (_, next, idx) = cand[0];
            let actions = get_move(now, next);
            for action in actions {
                println!("1 {}", action);
            }
            used_ng_coords[idx] = true;
            now = next;

            let obj = field[now.i][now.j];

            let mut cand = vec![];
            if obj == 'a' {
                for i in 0..empty_coords_for_stone.len() {
                    if used_empty_coords_for_stone[i] {
                        continue;
                    }
                    let next = empty_coords_for_stone[i];
                    let dist = calc_manhattan_dist(now, next);
                    cand.push((dist, next, i));
                }
            } else {
                for i in 0..empty_coords_for_wall.len() {
                    if used_empty_coords_for_wall[i] {
                        continue;
                    }
                    let next = empty_coords_for_wall[i];
                    let dist = calc_manhattan_dist(now, next);
                    cand.push((dist, next, i));
                }
            }
            cand.sort();
            let (_, next, idx) = cand[0];

            let mut queue = VecDeque::new();
            let mut dist = vec![vec![1000; input.N]; input.N];
            queue.push_back(now);
            dist[now.i][now.j] = 0;

            while let Some(pos) = queue.pop_front() {
                if pos == next {
                    break;
                };
                for dir in DIJ4.iter() {
                    let nxt = pos + *dir;
                    if !nxt.in_map(input.N) {
                        continue;
                    }
                    if field[nxt.i][nxt.j] != '.' {
                        continue;
                    }
                    if dist[pos.i][pos.j] + 1 < dist[nxt.i][nxt.j] {
                        dist[nxt.i][nxt.j] = dist[pos.i][pos.j] + 1;
                        queue.push_back(nxt);
                    }
                }
            }

            let mut actions = vec![];
            let mut cur = next;

            let mut okok = true;
            while cur != now {
                let mut ok = false;
                for dir in DIJ4.iter() {
                    let prev = cur + *dir;
                    if !prev.in_map(input.N) {
                        continue;
                    }
                    if dist[prev.i][prev.j] == dist[cur.i][cur.j] - 1 {
                        actions.push(get_move(prev, cur)[0]);
                        cur = prev;
                        ok = true;
                        break;
                    }
                }
                if !ok {
                    okok = false;
                    break;
                }
            }
            if !okok {
                used_empty_coords_for_stone[idx] = true;
                ng_cnt += 1;
                return;
            }
            actions.reverse();

            for action in actions {
                println!("2 {}", action);
            }
            field[now.i][now.j] = '.';
            field[next.i][next.j] = obj;
            if obj == 'a' {
                used_empty_coords_for_stone[idx] = true;
            } else if obj == '@' {
                used_empty_coords_for_wall[idx] = true;
            } else {
                panic!();
            }
            now = next;

            for i in 0..input.N {
                for j in 0..input.N {
                    eprint!("{}", input.C[i][j]);
                }
                eprint!("     ");
                for j in 0..input.N {
                    eprint!("{}", field[i][j]);
                }
                eprintln!();
            }
            eprintln!();
        }

        let mut K2 = 0;
        for i in 0..input.N {
            for j in 0..input.N {
                // if field[i][j] == 'a' {
                //     assert!(good_field[i][j]);
                // } else if field[i][j] == '@' {
                //     assert!(!good_field[i][j]);
                // }
                eprint!("{}", input.C[i][j]);
                if field[i][j] == 'a' {
                    K2 += 1;
                }
            }
            eprint!("     ");
            for j in 0..input.N {
                eprint!("{}", field[i][j]);
            }
            eprintln!();
        }
        eprintln!();

        eprintln!("K: {}/{}", K2, K);

        // 穴の十字方向の石を落とす

        // スタート位置に戻る
        let actions = get_move(now, input.start);
        for action in actions {
            println!("1 {}", action);
        }
        now = input.start;

        let mut A = 0;

        // 上方向に石があるか確認
        let mut coords = vec![];
        for i in 0..now.i {
            if field[i][now.j] == 'a' {
                coords.push(Coord::new(i, now.j));
            }
        }

        let mut cnt = 0;
        while cnt < coords.len() {
            println!("1 U");
            now.i -= 1;
            if field[now.i][now.j] == 'a' {
                println!("3 D");
                A += 1;
                field[now.i][now.j] = '.';
                cnt += 1;
            }
        }

        for i in 0..input.N {
            for j in 0..input.N {
                // if field[i][j] == 'a' {
                //     assert!(good_field[i][j]);
                // } else if field[i][j] == '@' {
                //     assert!(!good_field[i][j]);
                // }
                eprint!("{}", input.C[i][j]);
            }
            eprint!("     ");
            for j in 0..input.N {
                eprint!("{}", field[i][j]);
            }
            eprintln!();
        }
        eprintln!();

        // スタート位置に戻る
        let actions = get_move(now, input.start);
        for action in actions {
            println!("1 {}", action);
        }
        now = input.start;

        // 下方向に石があるか確認
        let mut coords = vec![];
        for i in now.i + 1..input.N {
            if field[i][now.j] == 'a' {
                coords.push(Coord::new(i, now.j));
            }
        }

        let mut cnt = 0;
        while cnt < coords.len() {
            println!("1 D");
            now.i += 1;
            if field[now.i][now.j] == 'a' {
                println!("3 U");
                A += 1;
                field[now.i][now.j] = '.';
                cnt += 1;
            }
        }

        for i in 0..input.N {
            for j in 0..input.N {
                // if field[i][j] == 'a' {
                //     assert!(good_field[i][j]);
                // } else if field[i][j] == '@' {
                //     assert!(!good_field[i][j]);
                // }
                eprint!("{}", input.C[i][j]);
            }
            eprint!("     ");
            for j in 0..input.N {
                eprint!("{}", field[i][j]);
            }
            eprintln!();
        }
        eprintln!();

        // スタート位置に戻る
        let actions = get_move(now, input.start);
        for action in actions {
            println!("1 {}", action);
        }
        now = input.start;

        // 左方向に石があるか確認
        let mut coords = vec![];
        for j in 0..now.j {
            if field[now.i][j] == 'a' {
                coords.push(Coord::new(now.i, j));
            }
        }

        let mut cnt = 0;
        while cnt < coords.len() {
            println!("1 L");
            now.j -= 1;
            if field[now.i][now.j] == 'a' {
                println!("3 R");
                A += 1;
                field[now.i][now.j] = '.';
                cnt += 1;
            }
        }

        for i in 0..input.N {
            for j in 0..input.N {
                // if field[i][j] == 'a' {
                //     assert!(good_field[i][j]);
                // } else if field[i][j] == '@' {
                //     assert!(!good_field[i][j]);
                // }
                eprint!("{}", input.C[i][j]);
            }
            eprint!("     ");
            for j in 0..input.N {
                eprint!("{}", field[i][j]);
            }
            eprintln!();
        }
        eprintln!();

        // スタート位置に戻る

        let actions = get_move(now, input.start);
        for action in actions {
            println!("1 {}", action);
        }
        now = input.start;

        // 右方向に石があるか確認
        let mut coords = vec![];
        for j in now.j + 1..input.N {
            if field[now.i][j] == 'a' {
                coords.push(Coord::new(now.i, j));
            }
        }

        let mut cnt = 0;
        while cnt < coords.len() {
            println!("1 R");
            now.j += 1;
            if field[now.i][now.j] == 'a' {
                println!("3 L");
                A += 1;
                field[now.i][now.j] = '.';
                cnt += 1;
            }
        }

        let mut K2 = 0;
        for i in 0..input.N {
            for j in 0..input.N {
                // if field[i][j] == 'a' {
                //     assert!(good_field[i][j]);
                // } else if field[i][j] == '@' {
                //     assert!(!good_field[i][j]);
                // }
                eprint!("{}", input.C[i][j]);
                if field[i][j] == 'a' {
                    K2 += 1;
                }
            }
            eprint!("     ");
            for j in 0..input.N {
                eprint!("{}", field[i][j]);
            }
            eprintln!();
        }
        eprintln!();

        eprintln!("K: {}/{}, A: {}", K2, K, A);

        // 石がある行に移動
        for i in 0..input.N {
            if i % 2 != input.start.i % 2 {
                continue;
            }
            let dir = if i < input.start.i { 'D' } else { 'U' };
            let a = field[i].iter().filter(|&&c| c == 'a').count();
            let mut a_cnt = 0;
            if field[i].iter().any(|&c| c == 'a') {
                let actions = get_move(now, Coord::new(i, now.j));
                for action in actions {
                    println!("1 {}", action);
                }
                now.i = i;
                eprintln!("now: {}", now);

                // start.jに最も近い石を探す
                let mut cand = vec![];
                for j in 0..input.N {
                    if field[i][j] == 'a' {
                        let dist = calc_manhattan_dist(input.start, Coord::new(i, j));
                        cand.push((dist, Coord::new(i, j)));
                    }
                }
                cand.sort();
                let (_, next) = cand[0];
                let actions = get_move(now, next);
                for action in actions {
                    println!("1 {}", action);
                }
                now = next;
                if i == 4 {
                    eprintln!("Debug {}", now);
                }

                let next = Coord::new(now.i, input.start.j);
                let actions = get_move(now, next);
                for action in actions {
                    println!("2 {}", action);
                }
                field[now.i][now.j] = '.';
                field[next.i][next.j] = 'a';
                now = next;

                // 左方向に石があるか確認
                let mut coords = vec![];
                for j in 0..now.j {
                    if field[now.i][j] == 'a' {
                        coords.push(Coord::new(now.i, j));
                    }
                }

                let mut cnt = 0;
                let mut move_cnt = 0;
                while cnt < coords.len() {
                    println!("1 L");
                    move_cnt += 1;
                    now.j -= 1;
                    if field[now.i][now.j] == 'a' {
                        println!("3 R");
                        field[now.i][now.j] = '.';
                        for j in now.j + 1..input.N - 1 {
                            if field[now.i][j + 1] == 'a' {
                                field[now.i][j] = 'a';
                                break;
                            }
                        }
                        cnt += 1;
                    }
                }
                for _ in 0..move_cnt {
                    println!("1 R");
                    now.j += 1;
                }

                // 右方向に石があるか確認
                let mut coords = vec![];
                for j in now.j + 1..input.N {
                    if field[now.i][j] == 'a' {
                        coords.push(Coord::new(now.i, j));
                    }
                }

                let mut cnt = 0;
                let mut move_cnt = 0;
                while cnt < coords.len() {
                    println!("1 R");
                    move_cnt += 1;
                    now.j += 1;
                    if field[now.i][now.j] == 'a' {
                        println!("3 L");
                        field[now.i][now.j] = '.';
                        for j in (1..now.j + 1).rev() {
                            if field[now.i][j - 1] == 'a' {
                                field[now.i][j] = 'a';
                                break;
                            }
                        }
                        cnt += 1;
                    }
                }
                for _ in 0..move_cnt {
                    println!("1 L");
                    now.j -= 1;
                }

                println!("3 {}", dir);
                A += 1;
                a_cnt += 1;
                field[now.i][now.j] = '.';

                if i == 4 {
                    eprintln!("Debug");
                    for i in 0..input.N {
                        for j in 0..input.N {
                            // if field[i][j] == 'a' {
                            //     assert!(good_field[i][j]);
                            // } else if field[i][j] == '@' {
                            //     assert!(!good_field[i][j]);
                            // }
                            eprint!("{}", input.C[i][j]);
                        }
                        eprint!("     ");
                        for j in 0..input.N {
                            eprint!("{}", field[i][j]);
                        }
                        eprintln!();
                    }
                    eprintln!("Debug");
                }

                // 左方向に石があるか確認
                let mut coords = vec![];
                for j in (0..now.j).rev() {
                    if field[now.i][j] == 'a' {
                        coords.push(Coord::new(now.i, j));
                    }
                }

                for coord in coords {
                    let actions = get_move(now, coord);
                    for action in actions {
                        println!("1 {}", action);
                    }
                    now = coord;
                    let next = Coord::new(now.i, input.start.j);
                    let actions = get_move(now, next);
                    for action in actions {
                        println!("2 {}", action);
                    }
                    now = next;
                    println!("3 {}", dir);
                    A += 1;
                    a_cnt += 1;
                    field[coord.i][coord.j] = '.';
                }

                // 右方向に石があるか確認

                let mut coords = vec![];
                for j in now.j + 1..input.N {
                    if field[now.i][j] == 'a' {
                        coords.push(Coord::new(now.i, j));
                    }
                }

                for coord in coords {
                    let actions = get_move(now, coord);
                    for action in actions {
                        println!("1 {}", action);
                    }
                    now = coord;
                    let next = Coord::new(now.i, input.start.j);
                    let actions = get_move(now, next);
                    for action in actions {
                        println!("2 {}", action);
                    }
                    now = next;
                    println!("3 {}", dir);
                    A += 1;
                    a_cnt += 1;
                    field[coord.i][coord.j] = '.';
                }
            }
            eprintln!("row: {}, a: {}/{}", i, a_cnt, a);
        }

        for i in 0..input.N {
            for j in 0..input.N {
                // if field[i][j] == 'a' {
                //     assert!(good_field[i][j]);
                // } else if field[i][j] == '@' {
                //     assert!(!good_field[i][j]);
                // }
                eprint!("{}", input.C[i][j]);
            }
            eprint!("     ");
            for j in 0..input.N {
                eprint!("{}", field[i][j]);
            }
            eprintln!();
        }
        eprintln!();

        eprintln!("{}/{}", A, K);
        eprintln!("ng_cnt: {}", ng_cnt);
        break;
    }
}

fn get_move(pos: Coord, next: Coord) -> Vec<char> {
    let mut ret = vec![];
    if pos.i < next.i {
        for _ in 0..next.i - pos.i {
            ret.push('D');
        }
    }
    if pos.i > next.i {
        for _ in 0..pos.i - next.i {
            ret.push('U');
        }
    }
    if pos.j < next.j {
        for _ in 0..next.j - pos.j {
            ret.push('R');
        }
    }
    if pos.j > next.j {
        for _ in 0..pos.j - next.j {
            ret.push('L');
        }
    }
    ret
}

fn main() {
    get_time();
    let input = read_input();
    solve(&input);
    eprintln!("Elapsed time = {:.3}", get_time());
}
