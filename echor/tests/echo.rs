use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().failure().stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success().stdout("hello\n");
    Ok(())
}

fn echo_test_multiargs(input: Vec<&str>, expected_path: &str) -> TestResult {
    let content = fs::read_to_string(expected_path)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(input).assert().success().stdout(content);
    Ok(())
}

#[test]
fn hello1() {
    let expected_path = "tests/expected/hello1.txt";
    let input = vec!["Hello there"];
    let _ = echo_test_multiargs(input, expected_path);
}

#[test]
fn hello2() {
    let expected_path = "tests/expected/hello2.txt";
    let input = vec!["Hello", "There"];
    let _ = echo_test_multiargs(input, expected_path);
}

#[test]
fn hello1n() {
    let expected_path = "tests/expected/hello1.n.txt";
    let input = vec!["-n", "Hello there"];
    let _ = echo_test_multiargs(input, expected_path);
}

#[test]
fn hello2n() {
    let expected_path = "tests/expected/hello2.n.txt";
    let input = vec!["-n", "Hello", "there"];
    let _ = echo_test_multiargs(input, expected_path);
}
