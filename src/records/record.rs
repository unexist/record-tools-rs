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
use log::{info, debug};
use std::collections::HashMap;

pub(crate) const DEFAULT_TITLE: &str = "No title given";

pub(crate) type RecordAttributes = HashMap<String, String>;

#[derive(Debug)]
pub(crate) struct Record {
    pub(crate) content: String,
    pub(crate) target_path: String,
}

impl Record {
    pub(crate) fn write(self: Self) -> Result<()> {
        debug!("Creating record `{}`", self.target_path);

        let mut file = File::create_new(&self.target_path)
            .with_context(|| format!("Failed to create new file: {}", self.target_path))?;

        file.write_all(self.content.to_string().as_bytes())
            .with_context(|| format!("Failed to write to file: {}", self.target_path))?;

        info!("Wrote record `{}`", self.target_path);

        Ok(())
    }
}

#[derive(Default)]
pub(crate) struct RecordBuilder<'a> {
    pub(crate) config: Option<&'a Config>,
    pub(crate) attrs: RecordAttributes,
    pub(crate) number: i16,
    pub(crate) title: String,
    pub(crate) date: String,
}

impl<'a> TryFrom<&'a Config> for RecordBuilder<'a> {
    type Error = anyhow::Error;

    fn try_from(config: &'a Config) -> Result<Self> {
        Ok(RecordBuilder {
            config: Some(config),
            number: 0,
            ..Default::default()
        })
    }
}

impl<'a> RecordBuilder<'a> {

    /// Set specific attribute
    ///
    /// # Arguments
    ///
    /// * `key` - Name of the attribute
    /// * `value` - Valueof the attribute
    ///
    /// # Returns
    ///
    /// An instance of [`RecordBuilder`]
    #[allow(unused)]
    pub(crate) fn set(&'a mut self, key: String, value: String) -> &'a mut RecordBuilder<'a> {
        self.attrs.insert(key, value);

        self
    }

    /// Merge attributes with the internal set
    ///
    /// # Arguments
    ///
    /// * `attrs` - Attributes to merge
    ///
    /// # Returns
    ///
    /// An instance of [`RecordBuilder`]
    pub(crate) fn merge(&'a mut self, attrs: &RecordAttributes) -> &'a mut RecordBuilder<'a> {
        self.attrs.extend(attrs.into_iter().map(|(key, value)| (key.clone(), value.clone())));

        self
    }

    /// Set title of the current record
    ///
    /// # Arguments
    ///
    /// * `number` - Number to set for this record
    ///
    /// # Returns
    ///
    /// An instance of [`RecordBuilder`]
    pub(crate) fn set_number(&'a mut self, number: i16) -> &'a mut RecordBuilder<'a> {
        self.number = number;

        self
    }

    /// Set title of the current record
    ///
    /// # Arguments
    ///
    /// * `title` - Title to set for this record
    ///
    /// # Returns
    ///
    /// An instance of [`RecordBuilder`]
    pub(crate) fn set_title(&'a mut self, title: &str) -> &'a mut RecordBuilder<'a> {
        self.title = title.to_string();

        self
    }

    /// Set current date to now
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// A [`Result`] with either [`Record`] on success or otherwise [`anyhow::Error`]
    pub(crate) fn set_date_now(&'a mut self) -> &'a mut RecordBuilder<'a> {
        let odt: OffsetDateTime = SystemTime::now().into();
        let format = format_description!("[year]-[month]-[day]");

        self.date = odt.format(&format)
            .expect("This date format should never fail");

        self
    }

    /// Build a new Record from the provided values
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// A [`Result`] with either [`Record`] on success or otherwise [`anyhow::Error`]
    pub(crate) fn build(&mut self) -> Result<Record> {
        let content = std::fs::read_to_string(self.config.unwrap()
            .get_default_template_path()?)?;
        let template = Template::from(content.as_str());

        if 0 <= self.number {
            self.number = find_next_num(&PathBuf::from(
                &self.config.unwrap().get_record_path()?))?;
        }

        self.attrs.insert(String::from("NUMBER"), self.number.to_string());
        self.attrs.insert(String::from("TITLE"), self.title.to_string());
        self.attrs.insert(String::from("DATE"), self.date.to_string());

        let mapping = self.attrs.iter()
            .map(|(ref k, ref v)| (k.as_str(), v.as_str()))
            .collect();

        debug!("Using attributes {:?}", mapping);

        Ok(Record {
            content: template.fill_in(&mapping).to_string(),
            target_path:  format!("{}/{:04}-{}.{}",
                self.config.unwrap().get_record_path()?.display(),
                self.number,
                slugify!(&*self.title),
                self.config.unwrap().file_type),
        })
    }
}

/// Find next free number
///
/// # Arguments
///
/// * `path` - Path to check for existing numbers files
///
/// # Returns
///
/// A [`Result`] with either [`i16`] on success or otherwise [`anyhow::Error`]
fn find_next_num(path: &Path) -> Result<i16> {
    let last_entry = std::fs::read_dir(path)?
        .flatten()
        .filter(|f| f.metadata().unwrap().is_file())
        .max_by_key(|x| x.file_name());

    if let Some(entry) = last_entry {
        let number = entry.file_name().to_str()
            .with_context(|| format!("Couldn't convert {:?} to string", entry.file_name()))?
            .chars().take(4).collect::<String>();

        return number.parse::<i16>().map_err(anyhow::Error::from).map(|i| i + 1);
    }

    Ok(1)
}
