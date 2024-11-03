#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::process::Stdio;

    #[test]
    fn check() {
        let output = Command::new("bash")
            .args(["shell/run.sh", "a", "0000"])
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