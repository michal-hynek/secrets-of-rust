use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use anyhow::Result;
use walkdir::WalkDir;

#[derive(Default)]
pub struct Slimmer {
    pub dry_run: bool,
}

impl Slimmer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn slim(&self, path: impl AsRef<Path>) -> Result<String> {
        let mut output = String::new();

        for target in manifests(path)? {
            let mut cmd = self.cargo_clean_cmd(&target)?;
            let cmd_output = cmd.output()?;
            output.push_str(&summary(target, &cmd_output));
        }

        Ok(output)
    }

    fn cargo_clean_cmd(&self, path: impl AsRef<Path>) -> Result<Command> {
        let mut cmd = Command::new("cargo");
        cmd.args([
            "clean",
            "--manifest-path",
            &path.as_ref().to_string_lossy(),
        ]);

        if self.dry_run {
            cmd.arg("--dry-run");
        }
        
        Ok(cmd)
    }
}

fn manifests(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut targets = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !e.path().ends_with("target/package"))
    {
        let entry = entry?;

        if entry.file_name() == "Cargo.toml" {
            targets.push(entry.path().to_path_buf());
        }
    }

    Ok(targets)
}

fn summary(target: impl AsRef<Path>, cmd_output: &Output) -> String {
    format!(
        "{}: {}",
        target.as_ref().parent().unwrap().display(),
        String::from_utf8_lossy(&cmd_output.stderr).trim_start(),
    )
}

#[cfg(test)]
mod tests {
    use std::process::ExitStatus;

    use super::*;

    #[test]
    fn manifests_returns_cargo_toml_paths() {
        let mut tomls = manifests("tests").unwrap();
        tomls.sort();

        assert_eq!(
            tomls,
            vec![
                PathBuf::from("tests/data/proj_1/Cargo.toml"),
                PathBuf::from("tests/data/proj_2/Cargo.toml"),
                PathBuf::from("tests/data/proj_3/Cargo.toml"),
            ],
        );
    }

    #[test]
    fn cargo_clean_cmd_returns_correct_command() {
        let slimmer = Slimmer::new();
        let cmd = slimmer.cargo_clean_cmd("tests/data/proj_1/Cargo.toml").unwrap();

        assert_eq!(cmd.get_program(), "cargo");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            ["clean", "--manifest-path", "tests/data/proj_1/Cargo.toml"],
        );
    }

    #[test]
    fn cargo_clean_cmd_honours_dry_run_mode() {
        let mut slimmer = Slimmer::new();
        slimmer.dry_run = true;
        let cmd = slimmer.cargo_clean_cmd("tests/data/proj_2/Cargo.toml").unwrap();

        assert_eq!(cmd.get_program(), "cargo");
        assert_eq!(
            cmd.get_args().collect::<Vec<_>>(),
            ["clean", "--manifest-path", "tests/data/proj_2/Cargo.toml", "--dry-run"],
        )
    }

    #[test]
    fn summary_returns_output_for_target() {
        let cmd_output = Output {
            status: ExitStatus::default(),
            stdout: Vec::new(),
            stderr: String::from("    Removed 3 files, 1.2MiB total...\n").into_bytes(),
        };
        let summary_msg= summary(
            PathBuf::from("tests/data/proj_1/Cargo.toml"),
            &cmd_output,
        );

        assert_eq!(
            summary_msg,
            "tests/data/proj_1: Removed 3 files, 1.2MiB total...\n"
        );
    }


}