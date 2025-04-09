use std::{fs::File, io::{self, BufRead, BufReader}};
use anyhow::Context;

pub fn count_lines(input: impl BufRead) -> io::Result<usize> {
    let mut count = 0;

    for line_res in input.lines() {
        line_res?;
        count += 1;
    }

    Ok(count)
}

pub fn count_lines_in_path(path: &String) -> anyhow::Result<usize> {
    let file = File::open(path).with_context(|| path.clone())?;
    let file = BufReader::new(file);

    count_lines(file).with_context(|| path.clone())
}

pub fn count_words(input: impl BufRead) -> io::Result<usize> {
    let mut count = 0;

    for line_res in input.lines() {
        let line = line_res?;
        count += line.split_whitespace().count();
    }

    Ok(count)
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
    fn count_lines_counts_lines_in_input() {
        let input = Cursor::new("line 1\nline 2\n");
        let lines = count_lines(input);

        assert_eq!(lines.unwrap(), 2);
    }

    #[test]
    fn count_lines_returns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count_lines(reader);

        assert!(result.is_err(), "no error returned");
    }

    #[test]
    fn count_lines_in_path_counts_lines_for_path() {
        let path = String::from("tests/data/test.txt");
        let result = count_lines_in_path(&path);

        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn count_lines_in_path_returns_err_for_invalid_paths() {
        let path = String::from(".");
        let result = count_lines_in_path(&path);

        assert!(result.is_err());
    }

    #[test]
    fn count_words_returns_words_in_input() {
        let input = Cursor::new("hello world\nfoo\nbar");
        let words = count_words(input);

        assert_eq!(words.unwrap(), 4);
    }
}