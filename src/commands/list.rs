///
/// @package record-tools-rs
///
/// @file List records
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use anyhow::Result;
use log::info;
use prettytable::{row, table};
use std::fs;
use crate::{Config, records::record_builder::RecordBuilder};

/// Execute command
///
/// # Arguments
///
/// * `config` - Config values read either from args or config file
/// * `attrs` - Record attributes
///
/// # Returns
///
/// A [`Result`] with either [`unit`] on success or otherwise [`anyhow::Error`]
pub(crate) fn execute(config: &Config) -> Result<()> {
    if let Ok(dir) = fs::read_dir(config.get_record_path()?) {
        let mut table = table!(["Number", "Title", "Date"]);

        for entry in dir {
            let entry = entry?;
            let record_builder = RecordBuilder::try_from(config)?
                .extract_from(&entry.path())?;

            info!("Loaded record `{}`", entry.path().display());

            table.add_row(row![
                record_builder.get_number(),
                record_builder.get_title(),
                record_builder.get_date(),
            ]);
        }

        table.printstd();
    }

    Ok(())
}
