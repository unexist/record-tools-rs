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
use anyhow::{Context, Result};
use std::{fs, io};
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

    let timestamp = chrono::offset::Local::now();

    println!(r#"<?xml version="1.0" encoding="UTF-8" ?>
<feed version="2.0">

    <!--
    //////////////////////////////////////////////////////////////
    /                                                            /
    / Do not change this file - all changes will be overwritten! /
    /                                                            /
    //////////////////////////////////////////////////////////////
    -->

    <title>List of all {} records</title>
    <description>List of all created {} records</description>
    <updated>{}</updated>
    <generator>{}</generator>
    <author>
        <name>{}</name>
    </author>"#, config.record_type, config.record_type,
    timestamp.format("%Y-%m-%d %H:%M"),
    env!("CARGO_PKG_NAME"), env!("CARGO_PKG_AUTHORS"));

    for entry in entries {
        let record_builder = RecordBuilder::try_from(config)?
            .extract_from(&entry)?;

        let num = record_builder.get_number().context("Numer cannot be empty")?;

        println!(r#"    <entry>
        <title>{}</title>
        <link></link>
        <id>{}</id>
        <updated>{}</updated>
        <summary></summary>
        <content></content>
    </entry>"#,
            record_builder.get_title().context("Title cannot be empty")?,
            num,
            record_builder.get_date().context("Date cannot be empty")?);
    }

    println!("</feed>");

    Ok(())
}
