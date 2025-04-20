use std::{fs::{self, File}, io::{BufReader, BufWriter, Result}, path::{Path, PathBuf}};
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Memos {
    path: PathBuf,
    pub inner: Vec<Memo>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Memo {
    pub text: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Status {
    Done,
    Pending,
}

impl Display for Memo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.text)
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Status::Pending => "-",
            Status::Done => "x",
        })
    }
}

impl Memos {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut memos = Self {
            path: PathBuf::from(path.as_ref()),
            inner: Vec::new(),
        };

        if fs::exists(&path)? {
            let file = BufReader::new(File::open(&path)?);
            memos.inner = serde_json::from_reader(file)?;
        }

        Ok(memos)
    }

    pub fn sync(&self) -> Result<()> {
        let file = File::create(&self.path)?;
        serde_json::to_writer(BufWriter::new(file), &self.inner)?;
        Ok(())
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
    fn round_trip_via_sync_and_open_preserves_data() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("memos.json");
        let memos = Memos {
            path: path.clone(),
            inner: vec![
                Memo {
                    text: "foo".to_string(),
                    status: Status::Pending,
                },
                Memo {
                    text: "bar".to_string(),
                    status: Status::Pending,
                },
            ]
        };
        memos.sync().unwrap();

        let memos2 = Memos::open(&path).unwrap();

        assert_eq!(memos.inner, memos2.inner);
    }

    #[test]
    fn sync_creates_file_if_it_does_not_exist() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("new.txt");
        let memos = Memos {
            path: path.clone(),
            inner: vec![
                Memo{ text: "hello".to_string(), status: Status::Pending },
                Memo{ text: "world".to_string(), status: Status::Pending },
            ],
        };
        memos.sync().unwrap();
        let memos = Memos::open(&path).unwrap();

        assert_eq!(memos.inner, vec![
            Memo {
                text: "hello".to_string(),
                status: Status::Pending,
            },
            Memo {
                text: "world".to_string(),
                status: Status::Pending,
            },
        ]);
    }

    #[test]
    fn sync_overwrites_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("existing.txt");
        fs::write(&path, "foo\nbar").unwrap();
        let new_memos = Memos {
            path: path.clone(),
            inner: vec![
                Memo { text: "hola".to_string(), status: Status::Pending },
                Memo { text: "mundo".to_string(), status: Status::Pending },
            ],
        };
        new_memos.sync().unwrap();
        let memos = Memos::open(&path).unwrap();

        assert_eq!(memos.inner, vec![
            Memo {
                text: "hola".to_string(),
                status: Status::Pending,
             },
            Memo {
                text: "mundo".to_string(),
                status: Status::Pending,
            }
        ]);
    }
}