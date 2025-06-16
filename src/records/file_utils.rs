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
use std::path::Path;
use regex::Regex;

pub fn find_next_num(path: &Path) -> Result<u16> {
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

pub fn extract_field(path: &Path, name: &str) -> Result<String> {
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to open template file: {}", 
                                 path.to_str().unwrap_or_default()))?;
    
    let re = Regex::new(&*format!("{}:[ |](.*)", name))?;

    if let Some((_, [result])) = re.captures(&content).map(|caps| caps.extract()) {
        return Ok(String::from(result));
    }

    Err(anyhow!("Couldn't find field"))
}