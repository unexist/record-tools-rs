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

use anyhow::{Context, Result};
use log::info;
use prettytable::{row, table};
use std::{fs, io};
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
    // Load and sort entries
    let mut entries = fs::read_dir(config.get_record_path()?)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    let mut table = table!(["Number", "Title", "Date"]);

    for entry in entries {
        let record_builder = RecordBuilder::try_from(config)?
            .extract_from(&entry)?;

        info!("Loaded record `{}`", entry.display());

        table.add_row(row![
            record_builder.get_number().context("Number cannot be empty")?,
            record_builder.get_title().context("Title cannot be empty")?,
            record_builder.get_date().context("Date cannot be empty")?,
        ]);
    }

    table.printstd();

    Ok(())
}
