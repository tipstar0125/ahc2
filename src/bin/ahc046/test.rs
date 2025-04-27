#[cfg(test)]
mod tests {
    use colored::*;
    use serde::{Deserialize, Serialize};
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Write;
    use std::process::{Command, Stdio};
    use std::thread;

    #[derive(Debug, Serialize, Deserialize)]
    struct Result {
        test_number: String,
        score: usize,
        elapsed_time: f64,
        is_ac: bool,
        is_tle: bool,
    }

    impl std::fmt::Display for Result {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut result = "".to_string();
            result += if self.is_ac { "AC" } else { "WA" };
            if self.is_tle {
                result += "/TLE";
            }

            write!(
                f,
                "{},{},{},{}",
                self.test_number, self.score, self.elapsed_time, result
            )?;
            Ok(())
        }
    }

    fn cocurrent<F, R>(job_num: usize, worker: F, args: Vec<usize>) -> Vec<R>
    where
        F: FnOnce(usize, usize) -> R + std::marker::Send + Copy + 'static,
        R: std::marker::Send + 'static,
    {
        let mut handles = vec![];
        let mut results = vec![];
        for (i, &arg) in args.iter().enumerate() {
            let handle = thread::spawn(move || {
                let result = worker(i, arg);
                result
            });
            handles.push(handle);
            if handles.len() == job_num {
                for handle in handles {
                    results.push(handle.join().unwrap());
                }
                handles = vec![];
            }
        }
        for handle in handles {
            results.push(handle.join().unwrap());
        }
        results
    }

    fn parse_int(line: &str, start: &str) -> usize {
        line.strip_prefix(start).unwrap().parse::<usize>().unwrap()
    }

    fn parse_float(line: &str, start: &str) -> f64 {
        line.strip_prefix(start).unwrap().parse::<f64>().unwrap()
    }

    fn run(test_number: usize, before_score: usize) -> Result {
        let test_number = format!("{:04}", test_number);

        // TODO: TLE設定
        const TLE: f64 = 2.0;

        // --binで指定するディレクトリを取得
        let exe_file_path = env::args().collect::<Vec<String>>()[0].clone();
        let exe_filename = exe_file_path
            .split("/")
            .last()
            .unwrap()
            .split("-")
            .next()
            .unwrap();

        // run + visualize
        // exp: makers run ahc038 0000
        let run_output = Command::new("makers")
            .args(["run", exe_filename, test_number.as_str()])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();

        // 標準エラー出力よりスコアと実行時間を取得
        let mut elapsed_time = 0.0;

        let binding = String::from_utf8_lossy(&run_output.stderr);
        for line in binding.split("\n") {
            if line.starts_with("Elapsed time = ") {
                elapsed_time = parse_float(line, "Elapsed time = ");
            }
        }

        // 標準出力よりスコアを取得し、実行結果と同じ値であるか確認
        let mut vis_score = 0;
        let binding = String::from_utf8_lossy(&run_output.stdout);
        for line in binding.split("\n") {
            if line.starts_with("Score = ") {
                vis_score = parse_int(line, "Score = ");
            }
        }

        let delta_score = vis_score as i64 - before_score as i64;

        // TODO
        println!(
            "{}: score={}, elapsed={}, delta={}",
            if vis_score > 1 {
                test_number.to_string().green()
            } else {
                test_number.to_string().red()
            },
            vis_score,
            if elapsed_time > TLE {
                elapsed_time.to_string().yellow()
            } else {
                elapsed_time.to_string().white()
            },
            if delta_score == 0 {
                delta_score.to_string().white()
            // TODO
            } else if delta_score > 0 {
                delta_score.to_string().green()
            } else {
                delta_score.to_string().red()
            }
        );

        Result {
            test_number,
            score: vis_score,
            elapsed_time,
            is_ac: vis_score > 0,
            is_tle: elapsed_time > TLE,
        }
    }

    #[test]
    fn multi() {
        let job_num = 4;
        let test_case_num = 50;

        let initial_score = 0;
        let before_scores = match File::open("before.json") {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let best_results: Vec<Result> = serde_json::from_str(&contents).unwrap();
                let mut best_scores = vec![initial_score; test_case_num];
                for r in best_results.iter() {
                    let num = r.test_number.parse::<usize>().unwrap();
                    if num < test_case_num {
                        best_scores[num] = r.score;
                    }
                }
                best_scores
            }
            Err(_) => {
                vec![initial_score; test_case_num]
            }
        };

        let results = cocurrent(job_num, run, before_scores.clone());

        let mut json_file = File::create("results.json").unwrap();
        writeln!(json_file, "{}", serde_json::to_string(&results).unwrap()).unwrap();

        let mut file = File::create("results.csv").unwrap();
        writeln!(file, "{}", "test_num,score,elapsed,result,delta").unwrap();
        let mut score_sum = 0;
        let mut wa_cnt = 0;
        let mut tle_cnt = 0;

        for (i, result) in results.iter().enumerate() {
            let delta_score = result.score as i64 - before_scores[i] as i64;
            writeln!(file, "{},{}", result, delta_score).unwrap();
            score_sum += result.score;
            if !result.is_ac {
                wa_cnt += 1;
            }
            if result.is_tle {
                tle_cnt += 1;
            }
        }
        let total = format!(
            "score sum: {}/{:.3}(log), WA: {}/{}, TLE: {}/{}",
            score_sum * 3,
            (score_sum as f64).log2(),
            wa_cnt,
            test_case_num,
            tle_cnt,
            test_case_num,
        );
        println!("{}", total);
        writeln!(file, "{}", total).unwrap();
    }
}
