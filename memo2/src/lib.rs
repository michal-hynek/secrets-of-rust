use std::{fs::{self, File}, io::{BufRead, BufReader, Result}, path::{Path, PathBuf}};

pub struct Memos {
    path: PathBuf,
    pub inner: Vec<String>,
}

impl Memos {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut memos = Self {
            path: PathBuf::from(path.as_ref()),
            inner: Vec::new(),
        };

        if fs::exists(&path)? {
            let file = BufReader::new(File::open(&path)?);
            for memo in file.lines() {
                memos.inner.push(memo?);
            }
        }

        Ok(memos)
    }

    pub fn sync(&self) -> Result<()> {
        fs::write(&self.path, self.inner.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    use tempfile::tempdir;

    #[test]
    fn open_returns_empty_vector_if_file_does_not_exist() {
        let memos = Memos::open("bogus.txt").unwrap();
        assert!(memos.inner.is_empty());
    }

    #[test]
    fn open_returns_data_from_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("memos.txt");
        fs::write(&path, "foo\nbar").unwrap();
        let memos = Memos::open(&path).unwrap();

        assert_eq!(memos.inner, vec!["foo", "bar"]);
    }

    #[test]
    fn sync_creates_file_if_it_does_not_exist() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("new.txt");
        let memos = Memos {
            path: path.clone(),
            inner: vec!["hello".to_string(), "world".to_string()],
        };
        memos.sync().unwrap();
        let memos = Memos::open(&path).unwrap();

        assert_eq!(memos.inner, vec!["hello", "world"]);
    }

    #[test]
    fn sync_overwrites_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("existing.txt");
        fs::write(&path, "foo\nbar").unwrap();
        let new_memos = Memos {
            path: path.clone(),
            inner: vec!["hola".to_string(), "mundo".to_string()],
        };
        new_memos.sync().unwrap();
        let memos = Memos::open(&path).unwrap();

        assert_eq!(memos.inner, vec!["hola", "mundo"]);
    }
}