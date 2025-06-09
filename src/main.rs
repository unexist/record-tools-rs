
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

mod create;

use clap_config_file::ClapConfigFile;

#[derive(ClapConfigFile)]
#[config_file_name = "config"]
#[config_file_formats = "yaml,toml,json"]
struct Config {
    /// Record file type
    #[config_arg(default_value = "adoc")]
    file_type: String,

    /// Path to Architecture Decision Records
    #[config_arg(default_value = "./architecture-decision-record")]
    adr_dir: String,

    /// Path to Technical Debts Records
    #[config_arg(default_value = "./technical-debt-records")]
    tdr_dir: String,

    /// Record type to create
    #[config_arg(short = "t", accept_from = "cli_only")]
    record_type: String,

    /// Option s
    #[config_arg(name = "s", short = "s", accept_from = "cli_only")]
    option_s: bool,

    #[config_arg(positional)]
    commands: Vec<String>,
}

fn main() {
    let (config, path, _format) = Config::parse_info();
    
    println!("Loaded config from: {:?}", path.unwrap_or_default());
    println!("Config: {:?}", config);
    println!("Command: {:?}", config.commands);

    if !config.commands.is_empty() {
        match config.commands[0].as_str() {
            "new" => {
                create::create(config.commands[1..].join(" ").into(), &config);
            },
            _ => {}
        }

    }
}
