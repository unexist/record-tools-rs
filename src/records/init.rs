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
use crate::records::record::RecordBuilder;

pub(crate) fn execute(title: String, config: &Config) -> Result<()> {
    let record = RecordBuilder::try_from(config)?
        .set_number(1)
        .set_title(title)
        .set_date_now()
        .build()?;

    record.write()?;

    Ok(())
}
