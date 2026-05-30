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
use crate::records::file_utils;
use anyhow::{Context, Result, bail};
use slugify::slugify;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;
use derive_builder::Builder;
use text_template::Template;
use time::OffsetDateTime;
use time::macros::format_description;
use log::info;
use stdext::default::default;

#[derive(Default, Debug)]
pub(crate) struct Record {
    pub(crate) path: String,
}

#[derive(Builder)]
#[builder(name = "RecordBuilder", build_fn(skip))]
pub(crate) struct RecordBuilderSeed {

}

impl RecordBuilder {
    pub(crate) fn build(&mut self) -> Result<Record> {
        Ok(Record {
            ..default()
        })
    }
}

pub(crate) fn execute(title: String, config: &Config) -> Result<()> {
    if title.is_empty() {
        bail!("Title cannot be empty");
    }

    // Load template
    let source_path = format!("{}/{}-template.{}",
        config.template_dir, config.record_type, config.file_type
    );

    let content = std::fs::read_to_string(&source_path)
        .with_context(|| format!("Failed to open template file: {}", source_path))?;

    let template = Template::from(content.as_str());

    // Get number
    let next_num = file_utils::find_next_num(&PathBuf::from(&config.get_current_path()?))?;
    let next_num_str = format!("{}", next_num);

    // Get time
    let odt: OffsetDateTime = SystemTime::now().into();
    let format = format_description!("[year]-[month]-[day]");
    let date = odt.format(&format)?;

    // Populate template
    let mut values = HashMap::<&str, &str>::new();

    values.insert("NUMBER", next_num_str.as_str());
    values.insert("TITLE", title.as_str());
    values.insert("DATE", date.as_str());
    values.insert("STATUS", "drafted");

    let result = template.fill_in(&values);

    // Write template
    let target_path = format!("{}/{:04}-{}.{}",
        config.get_current_path()?,
        next_num,
        slugify!(&*title),
        config.file_type
    );

    if config.dry_run {
        println!("Dry-run: {}:\n{}", target_path, result);
    } else {
        let mut file = File::create_new(&target_path)
            .with_context(|| format!("Failed to create new file: {}", target_path))?;

        file.write_all(result.to_string().as_bytes())
            .with_context(|| format!("Failed to write to file: {}", target_path))?;
    }

    info!("Created new decision record {} (dry-run: {}, superseded: {})",
        target_path,
        config.dry_run,
        !config.superseded.is_empty()
    );

    Ok(())
}
