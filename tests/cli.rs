use assert_cmd::Command;

#[test]
fn run_no_args() {
    let mut cmd = Command::cargo_bin("corroded_sock").unwrap();
    
    // should fail as no argument for port
    cmd.assert().failure().code(1);
}

#[test]
fn run_non_numeric_arg() {
    let mut cmd = Command::cargo_bin("corroded_sock").unwrap();
    let assert = cmd.arg("testtesttest").assert();
    
    // should fail as invalid argument for port
    assert.failure().code(2);
}