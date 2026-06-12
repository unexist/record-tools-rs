///
/// @package record-tools-rs
///
/// @file Create new record builder
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use crate::Config;
use crate::records::record::Record;
use aho_corasick::AhoCorasick;
use anyhow::{Context, Result};
use regex::Regex;
use slugify::slugify;
use std::path::Path;
use std::time::SystemTime;
use text_template::Template;
use time::OffsetDateTime;
use time::macros::format_description;
use log::debug;
use std::collections::HashMap;

pub(crate) const DEFAULT_TITLE: &str = "No title given";

const ATTR_NUMBER: &'static str = "NUMBER";
const ATTR_TITLE: &'static str = "TITLE";
const ATTR_DATE: &'static str = "DATE";

pub(crate) type RecordAttributes = HashMap<String, String>;

#[derive(Default)]
pub(crate) struct RecordBuilder<'a> {
    pub(crate) config: Option<&'a Config>,
    pub(crate) attrs: RecordAttributes,
}

impl<'a> TryFrom<&'a Config> for RecordBuilder<'a> {
    type Error = anyhow::Error;

    fn try_from(config: &'a Config) -> Result<Self> {
        Ok(RecordBuilder {
            config: Some(config),
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
    pub(crate) fn set(mut self, key: &str, value: &str) -> RecordBuilder<'a> {
        self.attrs.insert(String::from(key), String::from(value));

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
    pub(crate) fn merge(mut self, attrs: &RecordAttributes) -> RecordBuilder<'a> {
        self.attrs.extend(attrs.into_iter().map(|(key, value)| (key.clone(), value.clone())));

        self
    }

    /// Get the number of the record builder
    ///
    /// # Returns
    ///
    /// An [`Option`] with either [`Some(String)`] on success or otherwise [`None`]
    pub(crate) fn get_number(&'a self) -> Option<&'a String> {
        self.attrs.get(ATTR_NUMBER)
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
    #[allow(dead_code)]
    pub(crate) fn set_number(self, number: i16) -> RecordBuilder<'a> {
        let formatted = format!("{}", number);

        self.set(ATTR_NUMBER, &formatted)
    }

    /// Get the title of the record builder
    ///
    /// # Returns
    ///
    /// An [`Option`] with either [`Some(String)`] on success or otherwise [`None`]
    pub(crate) fn get_title(&'a self) -> Option<&'a String> {
        self.attrs.get(ATTR_TITLE)
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
    pub(crate) fn set_title(self, title: &str) -> RecordBuilder<'a> {
        self.set(ATTR_TITLE, title)
    }

    /// Get the date of the record builder
    ///
    /// # Returns
    ///
    /// An [`Option`] with either [`Some(String)`] on success or otherwise [`None`]
    pub(crate) fn get_date(&'a self) -> Option<&'a String> {
        self.attrs.get(ATTR_DATE)
    }

    /// Set current date to now
    ///
    /// # Arguments
    ///
    /// # Returns
    ///
    /// An instance of [`RecordBuilder`]
    pub(crate) fn set_date_now(self) -> RecordBuilder<'a> {
        let odt: OffsetDateTime = SystemTime::now().into();
        let format = format_description!("[year]-[month]-[day]");

        self.set(ATTR_DATE, &odt.format(&format)
            .expect("This date format should never fail"))
    }

    /// Extract record attributes based on the original template
    ///
    /// # Arguments
    ///
    /// * `path` - Path of the record to use
    ///
    /// # Returns
    ///
    /// A [`Result`] with either [`RecordBuilder`] on success or otherwise [`anyhow::Error`]
    pub(crate) fn extract_from(mut self, path: &Path) -> Result<RecordBuilder<'a>> {
        let content = std::fs::read_to_string(path)?;
        let template = std::fs::read_to_string(self.config.context("Config cannot be none")?
            .get_default_template_path()?)?;

        debug!("Loaded record `{}`", path.display());

        let mut pattern_lines = vec!();
        let re = Regex::new(r"\$\{(?<name>[A-Z_-]+)\}").unwrap();

        // Scan each line for attributes
        for line in template.lines() {
            let mut patterns = vec!();
            let mut replace_with = vec!();

            // Collect each attribute as a regex capture
            for cap in re.captures_iter(line) {
                if let Some(name) = cap.name("name") {
                    // We need to escape all the strings to avoid special regex relevant characters
                    let name_templ = regex::escape(format!("${{{}}}", name.as_str()).as_str()).to_string();
                    let pat_templ = format!("(?<{}>.+)", name.as_str());

                    patterns.push(name_templ);
                    replace_with.push(pat_templ);
                }
            }

            // Replace all attributes with the capture groups
            if !patterns.is_empty() {
                let ac = AhoCorasick::new(patterns)?;

                pattern_lines.push(ac.replace_all(&regex::escape(line), &replace_with));
            }
        }

        // Finally run regex and collect captured attrbiutes
        for pat in pattern_lines.iter() {
            debug!("pattern={}", pat);

            let re = Regex::new(&pat)?;

            for cap in re.captures_iter(&content) {
                debug!("capture={:?}", cap);

                for cap_name in re.capture_names() {
                    if let Some(name) = cap_name {
                        if let Some(act_match) = cap.name(name) {
                            debug!("{:?} => {:?}", name, act_match.as_str());
                            self.attrs.insert(String::from(name), String::from(act_match.as_str()));
                        }
                    }
                }
            }
        }

        Ok(self)
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

        // Sanitize record number
        let mut num = 0;

        if let Some(maybe_num) = self.get_number() {
            num = maybe_num.parse::<i16>().unwrap_or(0);
        }

        if 0 <= num {
            num = find_next_num(&self.config.unwrap().get_record_path()?)?;
        }

        self.attrs.insert(String::from(ATTR_NUMBER), num.to_string());

        // Convert HashMap<String, String> to HashMap<&str, &str> to satiesfy text_template::fill_in
        let mapping = self.attrs.iter()
            .map(|(ref k, ref v)| (k.as_str(), v.as_str()))
            .collect();

        debug!("Using attributes {:?}", mapping);

        Ok(Record {
            content: template.fill_in(&mapping).to_string(),
            target_path: format!("{}/{:04}-{}.{}",
                self.config.unwrap().get_record_path()?.display(),
                num,
                slugify!(&*self.get_title().context("Title cannot be empty")?),
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
