use std::{fs::{self, File}, io::{BufRead, BufReader, Result}, path::Path};

pub fn open(path: impl AsRef<Path>) -> Result<Vec<String>> {
    if fs::exists(&path)? {
        let file = BufReader::new(File::open(&path)?);
        file.lines().collect()
    } else {
        Ok(Vec::new())
    }
}

pub fn sync(memos: &[String], path: impl AsRef<Path>) -> anyhow::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    use tempfile::tempdir;

    #[test]
    fn open_returns_empty_vector_if_file_does_not_exist() {
        let memos = open("bogus.txt").unwrap();
        assert!(memos.is_empty());
    }

    #[test]
    fn open_returns_data_from_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("memos.txt");
        fs::write(&path, "foo\nbar").unwrap();
        let memos = open(&path).unwrap();

        assert_eq!(memos, vec!["foo", "bar"]);
    }
}