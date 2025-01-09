use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;

// teste exit-code 0
#[test]
fn exit_ok() {
    let mut cmd = Command::cargo_bin("cli_test").unwrap();
    cmd.args(["-e", "0"]);
    // output() executes command
    let output = cmd.output().expect("fail");
    assert!(output.status.success());
}

// teste exit-code 1
#[test]
fn exit_fail() {
    let mut cmd = Command::cargo_bin("cli_test").unwrap();
    cmd.args(["-e", "1"]);
    // output() executes command
    let output = cmd.output().expect("fail");
    assert_eq!(output.status.success(), false);
}

// teste substring des zurueckgegebenen strings mit "predicate"
#[test]
fn msg_ok_1() {
    let mut cmd = Command::cargo_bin("cli_test").unwrap();
    cmd.args(["-m", "Donaudampfschiffgesellschaft"]);
    let output = cmd.output().expect("fail");
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    let p = predicate::str::contains("dampf");
    assert!(p.eval(&stdout));
}

// teste exit-code 0 und zurueckgegebenen string
#[test]
fn msg_ok_2() {
    let mut cmd = Command::cargo_bin("cli_test").unwrap();
    cmd.args(["-m", "Hello, world!"]);
    let output = cmd.output().expect("fail");
    assert!(output.status.success()); // default exit status is 0
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, "Hello, world!\n");
}
