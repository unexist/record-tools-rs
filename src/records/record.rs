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
use std::io::Write;
use std::{fs::File, path::PathBuf};

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
    pub(crate) fn write(self) -> Result<()> {
        debug!("Creating record `{}`", self.target_path);

        File::create_new(&self.target_path)?.write_all(self.content.as_bytes())?;

        info!("Wrote record `{}`", self.target_path);

        Ok(())
    }

    /// Write record as html to disk
    ///
    /// # Returns
    ///
    /// A [`Result`] with either [`()`] on success or otherwise [`anyhow::Error`]
    pub(crate) fn write_html(self) -> Result<()> {
        let html_file = format!("{}.html", &self.target_path);

        File::create_new(html_file)?.write_all(self.content.as_bytes())?;

        Ok(())
    }
}
