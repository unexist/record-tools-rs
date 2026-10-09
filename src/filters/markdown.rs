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

use anyhow::Result;
use log::info;

/// Execute command
///
/// # Arguments
///
/// * `content` - Content to filter here
///
/// # Returns
///
/// A [`Result`] with either [`String`] on success or otherwise [`anyhow::Error`]
pub(crate) fn filter(content: &String, _css: &str) -> Result<String> {
    info!("Converting markdown to html");

    Ok(markdown::to_html(content))
}
