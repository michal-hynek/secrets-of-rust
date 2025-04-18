use std::{fs::{self, File}, io::{BufRead, BufReader, Result}, path::Path};

pub fn open(path: impl AsRef<Path>) -> Result<Vec<String>> {
    if fs::exists(&path)? {
        let file = BufReader::new(File::open(&path)?);
        file.lines().collect()
    } else {
        Ok(Vec::new())
    }
}

pub fn sync(memos: &[String], path: impl AsRef<Path>) -> Result<()> {
    fs::write(&path, memos.join("\n"))
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

    #[test]
    fn sync_creates_file_if_it_does_not_exist() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("new.txt");
        let memos = vec!["hello".to_string(), "world".to_string()];
        sync(&memos, &path).unwrap();

        assert_eq!(open(&path).unwrap(), vec!["hello", "world"]);
    }

    #[test]
    fn sync_overwrites_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("existing.txt");
        fs::write(&path, "foo\nbar").unwrap();
        let new_memos = vec!["hola".to_string(), "mundo".to_string()];
        sync(&new_memos, &path).unwrap();

        assert_eq!(open(&path).unwrap(), vec!["hola", "mundo"]);
    }
}