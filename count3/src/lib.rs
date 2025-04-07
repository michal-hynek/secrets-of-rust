use std::io::{self, BufRead};

pub fn count_lines(input: impl BufRead) -> io::Result<usize> {
    let mut count = 0;

    for line_res in input.lines() {
        line_res?;
        count += 1;
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
}