use std::{fs::{self, File}, path::Path};
use std::io::Write;

use anyhow::Result;

pub fn read(path: impl AsRef<Path>) -> Result<Option<String>> {
    if fs::exists(&path)? {
        let content = fs::read_to_string(&path)?;

        if content.is_empty() {
            return Ok(None);
        }

        Ok(Some(content))
    } else {
        Ok(None)
    }
}

pub fn append(path: impl AsRef<Path>, message: &str) -> Result<()> {
    let mut logbook = File::options()
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(logbook, "{}", message)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn read_returns_none_if_file_does_not_exist() {
        let text = read("tests/data/bogus.txt").unwrap();
        assert_eq!(text, None, "expected None");
    }

    #[test]
    fn read_returns_none_if_file_exists_and_is_empty() {
        let text = read("tests/data/empty.txt").unwrap();
        assert_eq!(text, None, "expected None");
    }

    #[test]
    fn read_returns_contents_of_file_as_string() {
        let text = read("tests/data/logbook.txt").unwrap().unwrap();
        assert_eq!(text, "hello world", "wrong content");
    }

    #[test]
    fn append_creates_file_if_necessary() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("newlog.txt");
        append(&path, "hello logbook").unwrap();
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "hello logbook\n", "wrong content");
    }

    #[test]
    fn append_appends_message_to_existring_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("logbook.trx");
        fs::write(&path, "hello world\n").unwrap();
        append(&path, "hello world 2").unwrap();
        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "hello world\nhello world 2\n", "wrong content");
    }
}