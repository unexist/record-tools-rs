
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

mod records;

use clap_config_file::ClapConfigFile;

#[derive(ClapConfigFile)]
#[config_file_name = "config"]
#[config_file_formats = "yaml,toml,json"]
struct Config {
    /// Record file type
    #[config_arg(default_value = "adoc")]
    file_type: String,
    
    #[config_arg(default_value = "./templates")]
    template_dir: String,

    /// Path to Architecture Decision Records
    #[config_arg(default_value = "./architecture-decision-record")]
    adr_dir: String,

    /// Path to Technical Debts Records
    #[config_arg(default_value = "./technical-debt-records")]
    tdr_dir: String,

    /// Record type to create
    #[config_arg(short = 't', accept_from = "cli_only")]
    record_type: String,

    /// Supersed old decision record
    #[config_arg(short = 's', accept_from = "cli_only")]
    superseded: String,

    /// Just run and don't create files
    #[config_arg(accept_from = "cli_only")]
    dry_run: bool,

    #[config_arg(positional)]
    commands: Vec<String>,
}

fn handle_command(config: &Config) -> anyhow::Result<()> {
    if !config.commands.is_empty() {
        let subcmd = config.commands[0].as_str();
        let remainder = config.commands[1..].join(" ").to_string();

        match subcmd {
            "create" => {
                records::create::execute(remainder, &config)?;
            },
            _ => anyhow::bail!("Command not implemented yet"),
        }
    }

    Ok(())
}

fn main() {
    let (config, path, _format) = Config::parse_info();
    
    println!("Loaded config from: {:?}", path.unwrap_or_default());
    println!("Config: {:?}", config);
    println!("Command: {:?}", config.commands);

    if let Err(e) = handle_command(&config) {
        eprintln!("Error: {}", e);
    }
}
