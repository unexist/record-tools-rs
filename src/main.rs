///
/// @package record-tools-rs
///
/// @file Main file
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

mod records;
mod config;

use std::path::Path;
use std::process::exit;
use anyhow::{bail, Result};
use crate::config::Config;

fn sanity_checks(config: &Config) -> Result<()> {
    if !Path::new(config.template_dir.as_str()).exists() {
        bail!("Template directory {} does not exist", config.template_dir);
    }
    if !Path::new(config.adr_dir.as_str()).exists() {
        bail!("ADR directory {} does not exist", config.adr_dir);
    }
    if !Path::new(config.tdr_dir.as_str()).exists() {
        bail!("TDR directory {} does not exist", config.tdr_dir);
    }

    Ok(())
}

fn handle_command(config: &Config) -> Result<()> {
    if !config.commands.is_empty() {
        let subcmd = config.commands[0].as_str();
        let remainder = config.commands[1..].join(" ").to_string();

        match subcmd {
            "init" => {
                records::init::execute(remainder, &config)?;
            },
            "create" => {
                records::create::execute(remainder, &config)?;
            },
            _ => bail!("Command not implemented yet"),
        }
    }

    Ok(())
}

fn main() {
    let (config, path, _format) = Config::parse_info();

    println!("Loaded config from: {:?}", path.unwrap_or_default());
    println!("Config: {:?}", config);
    println!("Command: {:?}", config.commands);

    if let Err(e) = sanity_checks(&config) {
        eprintln!("Error: {}", e);

        exit(1);
    }

    // Run actual command
    if let Err(e) = handle_command(&config) {
        eprintln!("Error: {}", e);
    }
}
