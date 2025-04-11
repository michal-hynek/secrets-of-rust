use std::{fs::File, io::{self, BufRead, BufReader}};
use anyhow::Context;

#[derive(Default)]
pub struct Count {
    pub lines: usize,
    pub words: usize,
}

pub fn count(mut input: impl BufRead) -> io::Result<Count> {
    let mut line = String::new();
    let mut count = Count::default();

    loop {
        let bytes_read= input.read_line(&mut line)?;

        if bytes_read == 0 {
            break;
        }

        count.lines += 1;
        count.words += line.split_whitespace().count();

        line.clear();
    }

    Ok(count)
}

pub fn count_in_path(path: &String) -> anyhow::Result<Count> {
    let file = File::open(path).with_context(|| path.clone())?;
    let file = BufReader::new(file);

    count( file).with_context(|| path.clone())
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, ErrorKind, Read, Cursor};

    use super::*;

    struct ErrorReader;

    impl Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(ErrorKind::Other, "unknown error"))
        }
    }

    #[test]
    fn count_in_path_counts_lines_and_words_for_path() {
        let path = String::from("tests/data/test.txt");
        let count = count_in_path(&path).unwrap();

        assert_eq!(count.lines, 3);
        assert_eq!(count.words, 3);
    }

    #[test]
    fn count_in_path_returns_err_for_invalid_paths() {
        let path = String::from(".");
        let result = count_in_path(&path);

        assert!(result.is_err());
    }

    #[test]
    fn count_returns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count(reader);
        
        assert!(result.is_err());
    }

    #[test]
    fn count_returns_count_struct() {
        let reader = Cursor::new("hello world\nfoo\nbar");
        let count = count(reader).unwrap();

        assert_eq!(count.lines, 3, "incorrect number of lines");
        assert_eq!(count.words, 4, "incorrect number of words");
    }
}