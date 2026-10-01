//!
//! @package record-tools-rs
//!
//! @file Create new record
//! @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv3.
//! See the file LICENSE for details.
//!

use anyhow::Result;
use log::{debug, info};
use std::fs::File;
use std::io::Write;

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
    pub(crate) fn write(&self) -> Result<()> {
        debug!("Writing record `{}`", self.target_path);

        File::create(&self.target_path)?.write_all(self.content.as_bytes())?;

        info!("Wrote record `{}`", self.target_path);

        Ok(())
    }
}
