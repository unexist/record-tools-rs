use std::collections::HashMap;
///
/// @package record-tools-rs
///
/// @file File utils
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use anyhow::{anyhow, Context, Result};
use regex::Regex;

pub fn find_next_num(path: &str) -> Result<u16> {
    let last_entry = std::fs::read_dir(path)?
        .flatten()
        .filter(|f| f.metadata().unwrap().is_file()) 
        .max_by_key(|x| x.file_name());
    
    if let Some(entry) = last_entry {
        let number = entry.file_name().to_str()
            .with_context(|| format!("Couldn't convert {:?} to string", entry.file_name()))?
            .chars().take(4).collect::<String>();
        
        return number.parse::<u16>().map_err(anyhow::Error::from).map(|i| i + 1);
    }
    
    Ok(1)
}

fn extract_field(path: &str, name: &str) -> Result<String> {
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to open template file: {}", path))?;

    let re = Regex::new(&*format!("{}:[ |](.*)", name))?;

    re.captures(&content).map(|caps| caps.extract())
}

pub fn get_title(&self) -> Option<&String> {
    self.meta.get("title")
}
