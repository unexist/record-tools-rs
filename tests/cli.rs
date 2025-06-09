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
use std::path::Path;
use std::process::Command;

#[test]
fn should_load_config_file() -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new(".");

    Command::cargo_bin("rtrs")?
        .current_dir(dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Loaded config from:"));

    Ok(())
}

#[test]
fn should_show_help() -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new(".");

    Command::cargo_bin("rtrs")?
        .current_dir(dir)
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: rtrs [OPTIONS] [commands]..."));

    Ok(())
}

#[test]
fn should_create_new_record_dry() -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new(".");

    Command::cargo_bin("rtrs")?
        .current_dir(dir)
        .arg("create")
        .arg("-t")
        .arg("adr")
        .arg("--dry-run")
        .arg("Test ADR")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created new decision record ./architecture-decision-record/1-test-adr.adoc (superseded: false)"));

    Ok(())
}