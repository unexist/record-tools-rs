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

use crate::Config;
use crate::records::record_builder::RecordBuilder;
use anyhow::Result;
use log::debug;
use std::{fs, io};

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
    debug!("Reading files from {:?}", config.get_record_path()?);

    // Load and sort entries
    let mut entries = fs::read_dir(config.get_record_path()?)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    let css = config.get_css_skin()?;

    for entry in entries {
        debug!("Reading {:?}", entry);

        RecordBuilder::try_from(config)?
            .extract_from(&entry)?
            .build_html(&css)?
            .write_html()?;
    }

    Ok(())
}
