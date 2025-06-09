///
/// @package record-tools-rs
///
/// @file Main file
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use clap_config_file::ClapConfigFile;

#[derive(ClapConfigFile)]
#[config_file_name = "config"]
#[config_file_formats = "yaml,toml,json"]
struct Config {
    #[config_arg(default_value = "adoc")]
    file_type: String,

    #[config_arg(default_value = "./architecture-decision-record")]
    adr_dir: String,

    #[config_arg(default_value = "./technical-debt-records")]
    tdr_dir: String,

}

fn main() {
    let (config, _used_file, _format) = Config::parse_info();

    println!("Config: {:?}", config);
}
