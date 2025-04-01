use std::{error::Error, fs};

use assert_cmd::Command;
use predicates::prelude::predicate;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "catr";

const HELLO_WORLD_WITHOUT_NEWLINE: &str = "tests/inputs/hello_world_without_newline.txt";

#[test]
fn usage() -> TestResult {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello_world_without_newline() -> TestResult {
    run(
        &[HELLO_WORLD_WITHOUT_NEWLINE],
        "tests/expected/hello_world_without_newline.txt.out",
    )
}
