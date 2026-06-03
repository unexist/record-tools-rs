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

/// Records module
mod records;
/// Config module
mod config;
/// Log facility
mod logger;

use std::process::exit;
use anyhow::{bail, Result};
use log::{error, info, debug};
use crate::config::Config;
use std::collections::HashMap;

/// Print version info
fn print_version() {
    info!("{} {} - Copyright (c) 2025-present {}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    info!("Released under the GNU GPLv3");
}

/// Sanity-check configuration
///
/// # Arguments
///
/// * `config` - Config values read either from args or config file
///
/// # Returns
///
/// A [`Result`] with either [`unit`] on success or otherwise [`anyhow::Error`]
fn sanity_checks(config: &Config) -> Result<()> {
    if !config.get_template_path().exists() {
        bail!("Template directory {} does not exist", config.template_dir);
    }

    // Check record path
    let record_path = config.get_record_path()?;

    if !record_path.exists() {
        debug!("Creating records directory")
    }

    Ok(())
}

/// Handle commands
///
/// # Arguments
///
/// * `config` - Config values read either from args or config file
///
/// # Returns
///
/// A [`Result`] with either [`unit`] on success or otherwise [`anyhow::Error`]
fn handle_command(config: &Config) -> Result<()> {
    if !config.commands.is_empty() {
        let subcmd = config.commands[0].as_str();
        let remainder = config.commands[1..].join(" ").to_string();

        match subcmd {
            "init" => {
                let attrs = HashMap::from([(String::from("title"), remainder)]);

                records::init::execute(&config, &attrs)?;
            },
            "create" => {
                let attrs = HashMap::from([(String::from("title"), remainder)]);

                records::create::execute(&config, &attrs)?;
            },
            _ => bail!("Command not implemented yet"),
        }
    }

    Ok(())
}

/// Main function
///
/// # Returns
///
/// A [`Result`] with either [`unit`] on success or otherwise [`anyhow::Error`]
fn main() -> Result<()> {
    let (config, path, _format) = Config::parse_info();

    logger::init(&config)?;

    print_version();

    info!("Loaded config from: {:?}", path.unwrap_or_default());
    info!("Config: {:?}", config);
    info!("Command: {:?}", config.commands);

    if let Err(e) = sanity_checks(&config) {
        error!("Error: {}", e);

        exit(1);
    }

    // Run actual command
    if let Err(e) = handle_command(&config) {
        error!("Failed to handle command: {}", e);
    }

    Ok(())
}
