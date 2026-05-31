use std::cell::OnceCell;
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
use anyhow::{Context, Result, bail};
use slugify::slugify;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use text_template::Template;
use time::OffsetDateTime;
use time::macros::format_description;
use log::info;
use stdext::default::default;

#[derive(Debug)]
pub(crate) struct Record {
    pub(crate) content: String,
    pub(crate) target_path: String,
}

impl Record {
    pub(crate) fn write(self: Self, _path: &str) -> Result<()> {
        Ok(())
    }
}

pub(crate) struct RecordBuilder<'a> {
    pub(crate) template: Template<'a>,
    pub(crate) values: HashMap<String, String>,
    pub(crate) number: u16,
    pub(crate) title: String,
    pub(crate) date: String,
}

impl TryFrom<&Config> for RecordBuilder<'_> {
    type Error = anyhow::Error;

    fn try_from(config: &Config) -> Result<Self> {
        let content= std::fs::read_to_string(config.get_default_template_path()?)?;

        Ok(RecordBuilder {
            template: Template::from(content.clone().as_str()),
            values: Default::default(),
            number: find_next_num(&PathBuf::from(&config.get_current_path()?))?,
            title: Default::default(),
            date: Default::default(),
        })
    }
}

impl<'a> RecordBuilder<'a> {
    pub(crate) fn set(&mut self, key: String, value: String) -> &mut RecordBuilder<'a> {
        self.values.insert(key, value);

        self
    }

    pub(crate) fn set_number(&mut self, number: u32) -> &mut RecordBuilder<'a> {
        self.values.insert(String::from("NUMBER"), number.to_string());

        self
    }

    pub(crate) fn set_title(&mut self, title: String) -> &mut RecordBuilder<'a> {
        self.values.insert(String::from("TITLE"), title);

        self
    }

    pub(crate) fn set_date_now(&mut self) -> &mut RecordBuilder<'a> {
        let odt: OffsetDateTime = SystemTime::now().into();
        let format = format_description!("[year]-[month]-[day]");
        let date = odt.format(&format).expect("This date format should never fail");

        self.values.insert(String::from("DATE"), date);

        self
    }

    pub(crate) fn build(&mut self) -> Result<Record> {

        Ok(Record {
           content: self.template.fill_in(&self.values).to_string(),
            target_path:  format!("{}/{:04}-{}.{}",
                config.get_current_path()?,
                next_num,
                slugify!(&*title),
                config.file_type),
        })
    }
}

pub(crate) fn execute(title: String, config: &Config) -> Result<()> {
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

pub(crate) fn find_next_num(path: &Path) -> Result<u16> {
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
