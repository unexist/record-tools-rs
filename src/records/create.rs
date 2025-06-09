///
/// @package record-tools-rs
///
/// @file Create new record
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use crate::Config;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::time::SystemTime;
use text_template::Template;
use time::OffsetDateTime;
use time::macros::format_description;

pub(crate) fn execute(title: String, config: &Config) -> Result<()> {
    if title.is_empty() {
        anyhow::bail!("Title cannot be empty");
    }

    // Load template
    let file_path = format!("{}/{}-template.{}",
                            config.template_dir, config.record_type, config.file_type);

    let content = std::fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to open file: {}", file_path))?;

    let template = Template::from(content.as_str());

    let odt: OffsetDateTime = SystemTime::now().into();
    let format = format_description!("[year]-[month]-[day]");
    let date = odt.format(&format)?;

    // Populate template
    let mut values = HashMap::<&str, &str>::new();

    values.insert("NUMBER", "1");
    values.insert("TITLE", title.as_str());
    values.insert("DATE", date.as_str());
    values.insert("STATUS", "drafted");

    let result = template.fill_in(&values);

    println!("{}", result);

    println!("Created new decision record (title: {}, type: {}, superseded: {})",
             title, config.record_type, config.superseded);
    
    Ok(())
}