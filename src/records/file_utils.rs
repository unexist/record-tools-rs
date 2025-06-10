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

use anyhow::{anyhow, Result};

pub fn find_next_val(path: &str) -> Result<i16> {
    let last_file = std::fs::read_dir(path)?
        .flatten()
        .filter(|f| f.metadata().unwrap().is_file()) 
        .max_by_key(|x| x.file_name());
    
    if let Some(file) = last_file {
        return Ok(String::from(&file.file_name().to_str().unwrap()[0..4]).parse::<i16>()?);
    }
    
    Err(anyhow!("Failed to parse file name"))
}