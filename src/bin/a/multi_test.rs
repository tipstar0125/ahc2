#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::process::Stdio;

    #[test]
    fn check() {
        let output = Command::new("bash")
            .args(["run.sh", "a", "0000"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}