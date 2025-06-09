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
    // path to basic example
    let dir = Path::new(".");

    Command::cargo_bin("rtrs")?
        .current_dir(dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Loaded config from:"));

    Ok(())
}