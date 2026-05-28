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


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    None,
    Info,
    Warnings,
    Error,
    Debug
}

pub(crate) fn init(config: &Config) -> Result<()> {
    let mut level = LogLevel::from(&config.loglevel);

    let filter = LevelFilter::from(level);

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .filter_level(filter)
        .try_init()?;

    Ok(())
}
