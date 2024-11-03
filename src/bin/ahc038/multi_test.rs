#[cfg(test)]
mod tests {
    use std::env;
    use std::process::Command;
    use std::process::Stdio;

    #[test]
    fn check() {
        let exe_file_path = env::args().collect::<Vec<String>>()[0].clone();
        let exe_filename = exe_file_path
            .split("/")
            .last()
            .unwrap()
            .split("-")
            .next()
            .unwrap();

        let output = Command::new("bash")
            .args(["shell/run.sh", exe_filename, "0000"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        println!("{}", String::from_utf8_lossy(&output.stderr));
        let output = Command::new("bash")
            .args(["shell/vis.sh", "in", "out"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
