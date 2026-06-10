///
/// @package record-tools-rs
///
/// @file Create new record
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use crate::Config;
use anyhow::{Result, bail};
use crate::records::record_builder::{RecordBuilder, RecordAttributes, DEFAULT_TITLE};

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
pub(crate) fn execute(config: &Config, attrs: &RecordAttributes) -> Result<()> {
    let title = attrs.get("title").map_or(DEFAULT_TITLE, |v| if v.is_empty() { DEFAULT_TITLE } else { v });

    if title.is_empty() {
        bail!("Title cannot be empty");
    }

    let record = RecordBuilder::try_from(config)?
        .set_title(title)
        .set_date_now()
        .merge(attrs)
        .build()?;

    if config.dry_run {
        println!("Dry-run: {}:\n{}", record.target_path, record.content);
    } else {
        record.write()?;
    }

    Ok(())
}
