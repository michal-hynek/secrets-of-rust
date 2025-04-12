use assert_cmd::Command;

#[test]
fn binary_with_no_args_prints_usage_message() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
}

#[test]
fn binary_counts_lines_in_named_files() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(["tests/data/test.txt", "tests/data/test2.txt"])
        .assert()
        .success()
        .stdout(predicates::str::contains("tests/data/test.txt: 3 lines\ntests/data/test2.txt: 2 lines"));
}

#[test]
fn binary_counts_words_if_words_flag_is_set() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .args(["-w", "tests/data/words.txt"])
        .assert()
        .success()
        .stdout("tests/data/words.txt: 4 words\n");
}