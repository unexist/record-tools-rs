//!
//! @package record-tools-rs
//!
//! @file Main file
//! @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv3.
//! See the file LICENSE for details.
//!

use log::{debug, LevelFilter};
use anyhow::Result;
use stdext::function_name;
use crate::Config;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    None,
    Info,
    Warnings,
    Error,
    Debug
}

impl From<&String> for LogLevel {
    fn from(level: &String) -> Self {
        match level.to_lowercase().as_str() {
            "none" => LogLevel::None,
            "info" => LogLevel::Info,
            "warnings" => LogLevel::Warnings,
            "errors" => LogLevel::Error,
            "debug" => LogLevel::Debug,
            _ => LogLevel::Info,
        }
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::None => LevelFilter::Off,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warnings => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Debug => LevelFilter::Debug,
        }
    }
}

/// Init logging system
///
/// # Arguments
///
/// * `config` - Config values read either from args or config file
///
/// # Returns
///
/// A [`Result`] with either [`unit`] on success or otherwise [`anyhow::Error`]
pub(crate) fn init(config: &Config) -> Result<()> {
    let mut level = LogLevel::from(&config.loglevel);

    // Handle special logging levels
    if config.debug {
        level = LogLevel::Debug;
    }

    if config.quiet {
        level = LogLevel::None;
    }

    let filter = LevelFilter::from(level);

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .filter_level(filter)
        .try_init()?;

    debug!("{}", function_name!());

    Ok(())
}
