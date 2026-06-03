///
/// @package record-tools-rs
///
/// @file Config functions
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use clap_config_file::ClapConfigFile;
use std::collections::HashMap;
use anyhow::{Context, Result, bail};
use std::path::PathBuf;

#[derive(ClapConfigFile)]
#[config_file_name = "config"]
#[config_file_formats = "yaml,toml,json"]
pub(crate) struct Config {
    /// Set logging level LEVEL
    #[config_arg(short = 'l', name = "level", default_value = "info", accept_from = "cli_only")]
    pub(crate) loglevel: String,

    /// Print debugging messages
    #[config_arg(short = 'd', default_value = false, accept_from = "cli_only")]
    pub(crate) debug: bool,

    /// Record file type
    #[config_arg(default_value = "adoc")]
    pub(crate) file_type: String,

    /// Path to templates
    #[config_arg(default_value = "./templates")]
    pub(crate) template_dir: String,

    /// List of known record types
    #[config_arg(name = "record_type", accept_from = "config_only")]
    pub(crate) record_types: Vec<HashMap<String, String>>,

    /// Record type to create
    #[config_arg(short = 't', default_value = "adr", accept_from = "cli_only")]
    pub(crate) record_type: String,

    /// Username to ue for new records
    #[config_arg(short = 'U', env = "USER")]
    pub(crate) username: String,

    /// Record type to create
    #[config_arg(short = 'T', default_value = "No title given", accept_from = "cli_only")]
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

    #[config_arg(positional)]
    pub(crate) commands: Vec<String>,
}

impl Config {
    pub(crate) fn get_record_path(self: &Self) -> Result<PathBuf> {
        for record_type in self.record_types.iter() {
            if Some(&self.record_type) == record_type.get("name") {
                return Ok(PathBuf::from(record_type.get("directory").context("No directory found")?));
            }
        }

        bail!("No record type found");
    }

    pub(crate) fn get_template_path(self: &Self) -> PathBuf {
        PathBuf::from(&self.template_dir)
    }

    pub(crate) fn get_default_template_path(self: &Self) -> Result<PathBuf> {
        for record_type in self.record_types.iter() {
            if Some(&self.record_type) == record_type.get("name") {
                let template_name = record_type.get("default_template_name").context("No default template found")?;

                return Ok(PathBuf::from(format!("{}/{}.{}", self.template_dir, template_name, self.file_type)))
            }
        }

        bail!("No default template found");
    }
}
