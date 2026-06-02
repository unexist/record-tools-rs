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
///

use anyhow::Result;
use crate::Config;
use crate::records::record::{RecordBuilder, RecordAttributes, DEFAULT_TITLE};

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
    let title = attrs.get("title").unwrap_or(DEFAULT_TITLE);

    let record = RecordBuilder::try_from(config)?
        .set_number(1)
        .set_title(title)
        .set_date_now()
        .merge(attrs)
        .build()?;

    record.write()?;

    Ok(())
}
