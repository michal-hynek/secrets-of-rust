use std::path::Path;

use anyhow::Result;

pub fn slim(path: impl AsRef<Path>) -> Result<String> {
    let mut output = String::new();

    for target in manifests(path)? {
        let mut cmd = cargo_clean_cmd(&target);
        let cmd_output = cmd.output()?;
        output.push_str(&summary(target, &cmd_output));
    }

    Ok(output)
}