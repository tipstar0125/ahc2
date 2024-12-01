#[cfg(test)]
mod tests {
    use colored::*;
    use itertools::Itertools;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::process::{Command, Stdio};
    use std::thread;

    struct Result {
        test_number: String,
        score: usize,
        N: usize,
        T: usize,
        sigma: usize,
        limit: usize,
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
                "{},{},{},{},{},{},{},{}",
                self.test_number,
                self.N,
                self.T,
                self.sigma,
                self.limit,
                self.score,
                self.elapsed_time,
                result
            )?;
            Ok(())
        }
    }

    fn cocurrent<F, R>(job_num: usize, worker: F, args: Vec<usize>) -> Vec<R>
    where
        F: FnOnce(usize) -> R + std::marker::Send + Copy + 'static,
        R: std::marker::Send + 'static,
    {
        let mut handles = vec![];
        let mut results = vec![];
        for &arg in args.iter() {
            let handle = thread::spawn(move || {
                let reuslt = worker(arg);
                reuslt
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

    fn parse_elapased_time(line: &str) -> f64 {
        line.strip_prefix("Elapsed time = ")
            .unwrap()
            .parse::<f64>()
            .unwrap()
    }

    fn run(test_number: usize) -> Result {
        let test_number = format!("{:04}", test_number);

        // TLE設定
        const TLE: f64 = 3.0;

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
            .args(["interactive", exe_filename, test_number.as_str()])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();

        // 標準エラー出力よりスコアと実行時間を取得
        let mut elapsed_time = 0.0;
        let mut N = 0;
        let mut T = 0;
        let mut sigma = 0;
        let mut limit = 0;

        let binding = String::from_utf8_lossy(&run_output.stderr);
        for line in binding.split("\n") {
            if line.starts_with("Elapsed time = ") {
                elapsed_time = parse_elapased_time(line);
            }
            if line.starts_with("N = ") {
                N = parse_int(line, "N = ");
            }
            if line.starts_with("T = ") {
                T = parse_int(line, "T = ");
            }
            if line.starts_with("sigma = ") {
                sigma = parse_int(line, "sigma = ");
            }
            if line.starts_with("Limit = ") {
                limit = parse_int(line, "Limit = ");
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

        println!(
            "{}: N={}, T={}, sigma={}, limit={}, score={}, elapsed={}",
            if vis_score == 0 {
                test_number.to_string().red()
            } else {
                test_number.to_string().green()
            },
            N,
            T,
            sigma,
            limit,
            vis_score,
            if elapsed_time > TLE {
                elapsed_time.to_string().yellow()
            } else {
                elapsed_time.to_string().white()
            }
        );

        Result {
            test_number,
            N,
            T,
            sigma,
            limit,
            score: vis_score,
            elapsed_time,
            is_ac: vis_score > 0,
            is_tle: elapsed_time > TLE,
        }
    }

    #[test]
    fn multi() {
        let job_num = 4;
        let test_case_num = 100;
        let results = cocurrent(job_num, run, (0..test_case_num).collect_vec());
        let mut file = File::create("results.csv").unwrap();
        writeln!(file, "{}", "test_num,N,T,sigma,limit,score,elapsed,result").unwrap();
        let mut score_sum = 0;
        let mut wa_cnt = 0;
        let mut tle_cnt = 0;

        for result in results {
            writeln!(file, "{}", result).unwrap();
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
            score_sum / 2,
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
