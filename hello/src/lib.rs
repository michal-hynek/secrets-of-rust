use std::{io::Write, io::Result};

pub fn print(mut w: impl Write) -> Result<()> {
    writeln!(w, "Hello, world!")
}

pub fn world() -> String {
    String::from("Hello, world!")
}

#[test]
fn print_writes_to_writer() {
    let mut buffer = Vec::new();
    print(&mut buffer).unwrap();

    assert_eq!(String::from_utf8_lossy(&buffer), "Hello, world!\n")
}

#[test]
fn world_returns_hello_world() {
    let result = world();

    assert_eq!(result, "Hello, world!", "world() expected 'Hello, world!', got '{result}'")
}
