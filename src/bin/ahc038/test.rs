#[cfg(test)]
mod tests {
    use colored::*;
    use std::env;
    use std::process::Command;
    use std::process::Stdio;

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

        for i in 0..100 {
            let test_number = format!("{:04}", i);

            // run + visualize
            let run_output = Command::new("makers")
                .args(["run", exe_filename, test_number.as_str()])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .unwrap();

            // 標準エラー出力よりスコアと実行時間を取得
            let mut output = "".to_string();
            let mut score = "";
            let binding = String::from_utf8_lossy(&run_output.stderr);
            for line in binding.split("\n") {
                if line.starts_with("Score = ") {
                    score = line;
                    output += format!("{}, ", line).as_str();
                }
                if line.starts_with("Elapsed time = ") {
                    let elapsed_time = line
                        .split("=")
                        .last()
                        .unwrap()
                        .strip_prefix(" ")
                        .unwrap()
                        .parse::<f64>()
                        .unwrap();
                    if elapsed_time > TLE {
                        output += format!("{}sec", line).yellow().to_string().as_str();
                    } else {
                        output += format!("{}sec", line).as_str();
                    }
                }
            }

            // 標準出力よりスコアを取得し、実行結果と同じ値であるか確認
            let ok = String::from_utf8_lossy(&run_output.stdout)
                .split("\n")
                .into_iter()
                .any(|line| line == score);

            let output = format!(
                "{}: {} {}",
                test_number,
                if ok { "OK".green() } else { "NG".red() },
                output
            );
            println!("{}", output);
        }
    }
}
