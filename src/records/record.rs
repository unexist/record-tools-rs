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

use anyhow::{Context, Result};
use std::fs::File;
use std::io::Write;
use log::{info, debug};

#[derive(Debug)]
pub(crate) struct Record {
    pub(crate) content: String,
    pub(crate) target_path: String,
}

impl Record {

    /// Write record to disk
     ///
     /// # Returns
     ///
     /// A [`Result`] with either [`Record`] on success or otherwise [`anyhow::Error`]
    pub(crate) fn write(self: Self) -> Result<()> {
        debug!("Creating record `{}`", self.target_path);

        let mut file = File::create_new(&self.target_path)
            .with_context(|| format!("Failed to create new file: {}", self.target_path))?;

        file.write_all(self.content.to_string().as_bytes())
            .with_context(|| format!("Failed to write to file: {}", self.target_path))?;

        info!("Wrote record `{}`", self.target_path);

        Ok(())
    }
}
