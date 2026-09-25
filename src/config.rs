//!
//! @package record-tools-rs
//!
//! @file Config functions
//! @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv3.
//! See the file LICENSE for details.
//!

use anyhow::{Context, Result, bail};
use clap_config_file::ClapConfigFile;
use log::debug;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

const GITHUB_SKIN_URL: &'static str =
    "https://raw.githubusercontent.com/darshandsoni/asciidoctor-skins/refs/heads/gh-pages/css";

#[derive(ClapConfigFile)]
#[config_file_name = "config"]
#[config_file_formats = "yaml,toml,json"]
pub(crate) struct Config {
    /// Set logging level LEVEL
    #[config_arg(
        short = 'l',
        name = "level",
        default_value = "info",
        accept_from = "cli_only"
    )]
    pub(crate) loglevel: String,

    /// Print debugging messages
    #[config_arg(short = 'D', default_value = false, accept_from = "cli_only")]
    pub(crate) debug: bool,

    /// Print no system messages at all
    #[config_arg(short = 'q', default_value = false, accept_from = "cli_only")]
    pub(crate) quiet: bool,

    /// Record file type
    #[config_arg(short = 'd', name = "doc_type", default_value = "adoc")]
    pub(crate) doc_type: String,

    /// Path to templates
    #[config_arg(default_value = "./templates")]
    pub(crate) template_dir: String,

    /// Set global output directry
    #[config_arg(short = 'o', default_value = ".")]
    pub(crate) output_dir: String,

    /// List of known document types
    #[config_arg(name = "doc_types", accept_from = "config_only")]
    pub(crate) doc_types: Vec<HashMap<String, String>>,

    /// List of known record types
    #[config_arg(name = "record_types", accept_from = "config_only")]
    pub(crate) record_types: Vec<HashMap<String, String>>,

    /// Record type to create
    #[config_arg(short = 't', default_value = "adr", accept_from = "cli_only")]
    pub(crate) record_type: String,

    /// Username to ue for new records
    #[config_arg(short = 'U', env = "USER")]
    pub(crate) username: String,

    /// Record type to create
    #[config_arg(short = 'T', default_value = record_builder::DEFAULT_TITLE, accept_from = "cli_only")]
    pub(crate) title: String,

    /// Edit record after creation
    #[config_arg(short = 'e', accept_from = "cli_only")]
    pub(crate) edit: bool,

    /// Supersed old decision record
    #[config_arg(short = 's', accept_from = "cli_only")]
    pub(crate) superseded: String,

    /// Just run and don't create files
    #[config_arg(accept_from = "cli_only")]
    pub(crate) dry_run: bool,

    /// Try to fetch skin from <https://github.com/darshandsoni/asciidoctor-skins>
    #[config_arg(default_value = "asciidoctor", accept_from = "cli_only")]
    pub(crate) skin_name: String,

    #[config_arg(positional)]
    pub(crate) commands: Vec<String>,
}

impl Config {
    /// Get path to records according to type
    ///
    /// # Returns
    ///
    /// A [`Result`] with either [`PathBuf`] on success or otherwise [`anyhow::Error`]
    pub(crate) fn get_record_path(&self) -> Result<PathBuf> {
        for record_type in self.record_types.iter() {
            if Some(&self.record_type) == record_type.get("name") {
                return Ok(PathBuf::from(
                    record_type.get("directory").context("No directory found")?,
                ));
            }
        }

        bail!("No record type found");
    }

    /// Get path to templates
    ///
    /// # Returns
    ///
    /// A [`Result`] with either [`PathBuf`] on success or otherwise [`anyhow::Error`]
    pub(crate) fn get_template_path(&self) -> PathBuf {
        PathBuf::from(&self.template_dir)
    }

    /// Get output path
    ///
    /// # Returns
    ///
    /// A [`Result`] with either [`PathBuf`] on success or otherwise [`anyhow::Error`]
    pub(crate) fn get_output_path(&self) -> Result<PathBuf> {
        let dir = if self.output_dir.eq(".") {
            env::current_dir()
        } else {
            Ok(PathBuf::from(&self.output_dir))
        };

        dir.map_err(anyhow::Error::from)
    }

    /// Get path to default templates according to type
    ///
    /// # Returns
    ///
    /// A [`Result`] with either [`PathBuf`] on success or otherwise [`anyhow::Error`]
    pub(crate) fn get_default_template_path(&self) -> Result<PathBuf> {
        for record_type in self.record_types.iter() {
            if Some(&self.record_type) == record_type.get("name") {
                let template_name = record_type
                    .get("template_name")
                    .context("No default template found")?;

                return Ok(PathBuf::from(format!(
                    "{}/{}.{}",
                    self.template_dir, template_name, self.doc_type
                )));
            }
        }

        bail!("No default template found");
    }

    /// Get link format according to type
    ///
    /// # Returns
    ///
    /// A [`Result`] with either [`String`] on success or otherwise [`anyhow::Error`]
    #[allow(dead_code)]
    pub(crate) fn get_link_format(&self) -> Result<String> {
        for record_type in self.record_types.iter() {
            if Some(&self.record_type) == record_type.get("name") {
                return Ok(String::from(
                    record_type
                        .get("link_format")
                        .context("No link format found")?,
                ));
            }
        }

        bail!("No record type found");
    }

    /// Get css skin of given name
    ///
    /// # Returns
    ///
    /// A [`Result`] with either [`String`] on success or otherwise [`anyhow::Error`]
    pub(crate) fn get_css_skin(&self) -> Result<String> {
        debug!("Fetching `{}/{}.css`", GITHUB_SKIN_URL, self.skin_name);

        reqwest::blocking::get(format!("{}/{}.css", GITHUB_SKIN_URL, self.skin_name))?
            .text()
            .map_err(anyhow::Error::from)
    }
}
