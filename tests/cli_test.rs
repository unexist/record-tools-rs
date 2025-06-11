///
/// @package record-tools-rs
///
/// @file CLI test
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn should_show_help() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("rtrs")?
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: rtrs [OPTIONS] [commands]..."));

    Ok(())
}

#[test]
fn should_load_config_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("rtrs")?
        .arg("--config-file=test_config.toml")

        // https://github.com/bodo-run/clap-config-file/issues/8
        .arg("--template-dir=./templates")
        .arg("--adr-dir=./example/src/site/asciidoc/architecture-decision-records")
        .arg("--tdr-dir=./example/src/site/asciidoc/technical-debt-records")
        
        .assert()
        .success()
        .stdout(predicate::str::contains("Loaded config from:"))
        .stdout(predicate::str::contains("./example/src/site/asciidoc/"));

    Ok(())
}

#[test]
fn should_create_new_record_dry() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("rtrs")?
        .arg("--config-file=test_config.toml")
        
        // https://github.com/bodo-run/clap-config-file/issues/8
        .arg("--template-dir=./templates")
        .arg("--adr-dir=./example/src/site/asciidoc/architecture-decision-records")
        .arg("--tdr-dir=./example/src/site/asciidoc/technical-debt-records")
        
        .arg("create")
        .args(&["-t", "adr"])
        .arg("--dry-run")
        .arg("Test ADR")
        .assert()
        .success()
        .stdout(predicate::str::contains("Dry-run"))
        .stdout(predicate::str::contains( "Created new decision record"));

    Ok(())
}

#[test]
fn should_create_new_record() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("rtrs")?
        .arg("--config-file=test_config.toml")
        
        // https://github.com/bodo-run/clap-config-file/issues/8
        .arg("--template-dir=./templates")
        .arg("--adr-dir=./example/src/site/asciidoc/architecture-decision-records")
        .arg("--tdr-dir=./example/src/site/asciidoc/technical-debt-records")
        
        .arg("create")
        .args(&["-t", "adr"])
        .arg("Test ADR")
        .assert()
        .success()
        .stdout(predicate::str::contains( "Created new decision record"));

    Ok(())
}