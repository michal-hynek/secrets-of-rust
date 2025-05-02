use std::process::Command;
use std::time::{Duration, Instant};

pub struct Report {
    pub stdout: String,
    pub stderr: String,
    pub elapsed: Duration,
}
pub fn time(program: &str, args: &[String]) -> anyhow::Result<Report> {
    let mut command = Command::new(program);
    command.args(args);

    let start = Instant::now();
    let output = command.output()?;
    let elapsed = start.elapsed();

    Ok(Report {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        elapsed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_times_echo_program() {
        let report = time(
            "echo",
            &["hello".into(), "world".into()],
        ).unwrap();

        assert_eq!(report.stdout.trim_end(), "hello world");
        assert_eq!(report.stderr.trim_end(), "");
        assert!(!report.elapsed.is_zero());
    }

}