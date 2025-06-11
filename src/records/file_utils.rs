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

pub fn find_next_val(path: &str) -> Result<i16> {
    let last_entry = std::fs::read_dir(path)?
        .flatten()
        .filter(|f| f.metadata().unwrap().is_file()) 
        .max_by_key(|x| x.file_name());
    
    if let Some(entry) = last_entry {
        let number = entry.file_name().to_str()
            .with_context(|| format!("Could not convert {:?} to string", entry.file_name()))?
            .chars().take(4).collect::<String>();
        
        return number.parse::<i16>().map_err(anyhow::Error::from);
    }
    
    Err(anyhow!("Failed to parse file name"))
}