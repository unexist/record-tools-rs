//!
//! @package record-tools-rs
//!
//! @file List records as digraph
//! @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv3.
//! See the file LICENSE for details.
//!

use crate::records::record_builder::RecordBuilder;
use crate::{Config, filters};
use anyhow::Result;
use log::{debug, info};
use std::{fs, io};
use tap::Pipe;

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
    debug!("Reading record files from {:?}", config.get_record_path()?);

    // Load and sort entries
    let mut entries = fs::read_dir(config.get_record_path()?)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    for entry in entries {
        info!("Reading record file {:?}", entry);

        RecordBuilder::try_from(config)?
            .extract_from(&entry)?
            .pipe_borrow_mut(|builder| {
                if cfg!(feature = "asciidoc") && "adoc" == config.output_type {
                    builder.add_content_filter(&filters::asciidoc::filter)
                } else if cfg!(feature = "markdown") && "md" == config.output_type {
                    builder.add_content_filter(&filters::markdown::filter)
                } else {
                    Ok(builder)
                }
            })?
            .add_record_filter(&|record| {
                record.target_path = format!("{}.html", record.target_path);
            })?
            .build()?
            .write()?;
    }

    Ok(())
}
