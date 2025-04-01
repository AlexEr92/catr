use std::{error::Error, fs};

use assert_cmd::Command;
use predicates::prelude::predicate;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "catr";

const HELLO_WORLD_WITHOUT_NEWLINE: &str = "tests/inputs/hello_world_without_newline.txt";
const HELLO_WORLD_WITH_NEWLINE: &str = "tests/inputs/hello_world_with_newline.txt";
const COUNTING: &str = "tests/inputs/counting.txt";
const INVICTUS: &str = "tests/inputs/invictus.txt";
const EMPTY: &str = "tests/inputs/empty.txt";

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
fn hello_world_without_newline_b() -> TestResult {
    run(
        &["-b", HELLO_WORLD_WITHOUT_NEWLINE],
        "tests/expected/hello_world_without_newline.txt.b.out",
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
fn hello_world_with_newline_b() -> TestResult {
    run(
        &["-b", HELLO_WORLD_WITH_NEWLINE],
        "tests/expected/hello_world_with_newline.txt.b.out",
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
fn counting_b() -> TestResult {
    run(
        &["--number-nonblank", COUNTING],
        "tests/expected/counting.txt.b.out",
    )
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

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_n() -> TestResult {
    run(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
}

#[test]
fn empty_b() -> TestResult {
    run(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
}
