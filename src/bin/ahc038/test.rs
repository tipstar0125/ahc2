#[cfg(test)]
mod tests {
    use colored::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::process::{Command, Stdio};

    fn parse_input(line: &str, para: &str) -> usize {
        let mut ret = !0;
        let inputs: Vec<_> = line.strip_prefix("input:").unwrap().split(",").collect();
        for input in inputs.iter() {
            if input.starts_with(format!(" {} = ", para).as_str()) {
                ret = input
                    .split("=")
                    .last()
                    .unwrap()
                    .strip_prefix(" ")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
        }
        ret
    }

    fn parse_score(line: &str) -> usize {
        line.strip_prefix("Score = ")
            .unwrap()
            .parse::<usize>()
            .unwrap()
    }

    fn parse_elapased_time(line: &str) -> f64 {
        line.strip_prefix("Elapsed time = ")
            .unwrap()
            .parse::<f64>()
            .unwrap()
    }

    #[test]
    fn multi() {
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
        let mut file = File::create("results.csv").unwrap();
        writeln!(file, "{}", "num,N,M,V,score,elapsed").unwrap();

        for i in 0..2 {
            let test_number = format!("{:04}", i);

            // run + visualize
            // exp: makers run ahc038 0000
            let run_output = Command::new("makers")
                .args(["run", exe_filename, test_number.as_str()])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .unwrap();

            // 標準エラー出力よりスコアと実行時間を取得
            let mut score = 0;
            let mut elapsed_time = 0.0;
            let mut N = 0;
            let mut M = 0;
            let mut V = 0;

            let binding = String::from_utf8_lossy(&run_output.stderr);
            for line in binding.split("\n") {
                if line.starts_with("input:") {
                    N = parse_input(line, "N");
                    M = parse_input(line, "M");
                    V = parse_input(line, "V");
                }
                if line.starts_with("Score = ") {
                    score = parse_score(line);
                }
                if line.starts_with("Elapsed time = ") {
                    elapsed_time = parse_elapased_time(line);
                }
            }

            // 標準出力よりスコアを取得し、実行結果と同じ値であるか確認
            let mut vis_score = 0;
            let binding = String::from_utf8_lossy(&run_output.stdout);
            for line in binding.split("\n") {
                if line.starts_with("Score = ") {
                    vis_score = parse_score(line);
                }
            }

            writeln!(
                file,
                "{},{},{},{},{},{}",
                test_number, N, M, V, score, elapsed_time
            )
            .unwrap();
            println!(
                "{}: N={}, M={}, V={}, score={}, elapsed={}",
                if score == vis_score {
                    test_number.to_string().green()
                } else {
                    test_number.to_string().red()
                },
                N,
                M,
                V,
                score,
                if elapsed_time > TLE {
                    elapsed_time.to_string().yellow()
                } else {
                    elapsed_time.to_string().white()
                }
            );
        }
    }
}
