use std::path::{Path, PathBuf};

use anyhow::Result;
use walkdir::WalkDir;

pub fn slim(path: impl AsRef<Path>) -> Result<String> {
    let mut output = String::new();

    for target in manifests(path)? {
        //let mut cmd = cargo_clean_cmd(&target);
        //let cmd_output = cmd.output()?;
        //output.push_str(&summary(target, &cmd_output));
    }

    Ok(output)
}

fn manifests(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut targets = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        if entry.file_name() == "Cargo.toml" {
            targets.push(entry.path().to_path_buf());
        }
    }

    Ok(targets)
}

#[cfg(test)]
mod tests {
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

}