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
use slugify::slugify;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use text_template::Template;
use time::OffsetDateTime;
use time::macros::format_description;
use log::info;

#[derive(Debug)]
pub(crate) struct Record {
    pub(crate) content: String,
    pub(crate) target_path: String,
}

impl Record {
    pub(crate) fn write(self: Self) -> Result<()> {
        let mut file = File::create_new(&self.target_path)
            .with_context(|| format!("Failed to create new file: {}", self.target_path))?;

        file.write_all(self.content.to_string().as_bytes())
            .with_context(|| format!("Failed to write to file: {}", self.target_path))?;

        info!("Wrote record {}", self.target_path);

        Ok(())
    }
}

#[derive(Default)]
pub(crate) struct RecordBuilder {
    pub(crate) config: &<'_> Config,
    pub(crate) values: HashMap<String, String>,
    pub(crate) number: u16,
    pub(crate) title: String,
    pub(crate) date: String,
}

impl TryFrom<&Config> for RecordBuilder {
    type Error = anyhow::Error;

    fn try_from(config: &Config) -> Result<Self> {
        Ok(RecordBuilder {
            config: config,
            ..Default::default()
        })
    }
}

impl RecordBuilder {
    pub(crate) fn set(&mut self, key: String, value: String) -> &mut RecordBuilder {
        self.values.insert(key, value);

        self
    }

    pub(crate) fn set_number(&mut self, number: u16) -> &mut RecordBuilder {
        self.number = number;

        self
    }

    pub(crate) fn set_title(&mut self, title: String) -> &mut RecordBuilder {
        self.title = title;

        self
    }

    pub(crate) fn set_date_now(&mut self) -> &mut RecordBuilder {
        let odt: OffsetDateTime = SystemTime::now().into();
        let format = format_description!("[year]-[month]-[day]");

        self.date = odt.format(&format).expect("This date format should never fail");

        self
    }

    pub(crate) fn build(&mut self) -> Result<Record> {
        let content = std::fs::read_to_string(self.config.get_default_template_path()?)?;
        let template = Template::from(content.as_str());

        if 0 <= self.number {
            self.number = find_next_num(&PathBuf::from(&self.config.get_current_path()?))?;
        }

        self.values.insert(String::from("NUMBER"), self.number.to_string());
        self.values.insert(String::from("TITLE"), self.title);
        self.values.insert(String::from("DATE"), self.date);

        let mapping = self.values.iter()
            .map(|(ref key, ref value)| (key.as_str(), value.as_str()))
            .collect();

        Ok(Record {
            content: template.fill_in(&mapping).to_string(),
            target_path:  format!("{}/{:04}-{}.{}",
                self.config.get_current_path()?,
                self.number,
                slugify!(&*self.title),
                self.config.file_type),
        })
    }
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
