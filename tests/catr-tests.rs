use std::{error::Error, fs};

use assert_cmd::Command;
use predicates::prelude::predicate;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "catr";

const HELLO_WORLD_WITHOUT_NEWLINE: &str = "tests/inputs/hello_world_without_newline.txt";
const HELLO_WORLD_WITH_NEWLINE: &str = "tests/inputs/hello_world_with_newline.txt";
const COUNTING: &str = "tests/inputs/counting.txt";
const INVICTUS: &str = "tests/inputs/invictus.txt";

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

#[test]
fn hello_world_without_newline_n() -> TestResult {
    run(
        &["-n", HELLO_WORLD_WITHOUT_NEWLINE],
        "tests/expected/hello_world_without_newline.txt.n.out",
    )
}

#[test]
fn hello_world_with_newline() -> TestResult {
    run(
        &[HELLO_WORLD_WITH_NEWLINE],
        "tests/expected/hello_world_with_newline.txt.out",
    )
}

#[test]
fn hello_world_with_newline_n() -> TestResult {
    run(
        &["-n", HELLO_WORLD_WITH_NEWLINE],
        "tests/expected/hello_world_with_newline.txt.n.out",
    )
}

#[test]
fn counting() -> TestResult {
    run(&[COUNTING], "tests/expected/counting.txt.out")
}

#[test]
fn counting_n() -> TestResult {
    run(&["--number", COUNTING], "tests/expected/counting.txt.n.out")
}

#[test]
fn invictus() -> TestResult {
    run(&[INVICTUS], "tests/expected/invictus.txt.out")
}

#[test]
fn invictus_n() -> TestResult {
    run(&["-n", INVICTUS], "tests/expected/invictus.txt.n.out")
}

#[test]
fn invictus_b() -> TestResult {
    run(&["-b", INVICTUS], "tests/expected/invictus.txt.b.out")
}
