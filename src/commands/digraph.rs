use std::{fs, io};

///
/// @package record-tools-rs
///
/// @file List records as digraph
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use crate::Config;
use anyhow::{Context, Result};
use crate::records::record_builder::RecordBuilder;

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

    println!("digraph {{");
    println!("    node [shape=plaintext]");
    println!("    subgraph {{");

    for entry in entries {
        let record_builder = RecordBuilder::try_from(config)?
            .extract_from(&entry)?;

        println!("        _$n [label=\"{}\"; URL=\"{}\"];",
            entry.file_name().context("Filename cannot be empty")?.display(),
            record_builder.get_title().context("Title cannot be empty")?);
    }

    println!("    }}");

    Ok(())
}
