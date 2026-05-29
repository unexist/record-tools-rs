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
use anyhow::{Context, Result, anyhow};

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
    pub(crate) fn get_current_path(self: &Self) -> Result<&String> {
        for format in self.record_types.iter() {
            if Some(&self.record_type) == format.get("name") {
                return format.get("directory").context("No directory found");
            }
        }

        Err(anyhow!("No format found"))
    }
}
