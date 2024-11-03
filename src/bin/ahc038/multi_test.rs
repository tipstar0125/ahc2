#[cfg(test)]
mod tests {
    use colored::*;
    use std::env;
    use std::process::Command;
    use std::process::Stdio;

    #[test]
    fn check() {
        let tle = 3.0;

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
            let run_output = Command::new("bash")
                .args(["shell/run.sh", exe_filename, test_number.as_str()])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .unwrap();
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
                    if elapsed_time > tle {
                        output += format!("{}sec", line).yellow().to_string().as_str();
                    } else {
                        output += format!("{}sec", line).as_str();
                    }
                }
            }

            let vis_output = Command::new("bash")
                .args([
                    "shell/vis.sh",
                    format!("tools/in/{}.txt", test_number).as_str(),
                    format!("tools/out/{}.txt", test_number).as_str(),
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .unwrap();
            let ok = String::from_utf8_lossy(&vis_output.stdout)
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
