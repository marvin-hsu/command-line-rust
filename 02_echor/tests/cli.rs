use std::fs;

use assert_cmd::Command;

#[test]
fn dies_no_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::is_match(r"(?i)usAge").unwrap());

    Ok(())
}

#[test]
fn runs() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello1() -> Result<(), Box<dyn std::error::Error>> {
    run(&vec!["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> Result<(), Box<dyn std::error::Error>> {
    run(&vec!["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<(), Box<dyn std::error::Error>> {
    run(&vec!["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> Result<(), Box<dyn std::error::Error>> {
    run(&vec!["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}

fn run(args: &[&str], expected_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}
