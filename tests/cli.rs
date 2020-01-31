use std::process::Command;

use assert_cmd::prelude::*;
//use predicates::prelude::*;

#[test]
fn gaffy_check() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("fastix")?;
    cmd.args(&["--prefix", "K-3138."]);
    cmd.arg("tests/data/K-3138.fa");

    cmd.assert().success();

    Ok(())
}
