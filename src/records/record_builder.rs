//!
//! @package record-tools-rs
//!
//! @file Create new record builder
//! @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv3.
//! See the file LICENSE for details.
//!

use crate::Config;
use crate::records::record::Record;
use aho_corasick::AhoCorasick;
use anyhow::{Context, Result};
use log::debug;
use regex::Regex;
use slugify::slugify;
use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;
use time::OffsetDateTime;
use time::macros::format_description;

pub(crate) const DEFAULT_TITLE: &str = "No title given";

const ATTR_NUMBER: &str = "NUMBER";
const ATTR_TITLE: &str = "TITLE";
const ATTR_DATE: &str = "DATE";

pub(crate) type RecordAttributes = HashMap<String, String>;

#[derive(Default)]
pub(crate) struct RecordBuilder<'a> {
    pub(crate) config: Option<&'a Config>,
    pub(crate) attrs: RecordAttributes,
    pub(crate) content_filters: Vec<&'a dyn Fn(&String) -> Result<String>>,
    pub(crate) record_filters: Vec<&'a dyn Fn(&mut Record)>,
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
    pub(crate) fn set_attr(mut self, key: &str, value: &str) -> RecordBuilder<'a> {
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
        self.attrs.extend(
            attrs
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        );

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

        self.set_attr(ATTR_NUMBER, &formatted)
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
        self.set_attr(ATTR_TITLE, title)
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
    /// # Returns
    ///
    /// An instance of [`RecordBuilder`]
    pub(crate) fn set_date_now(self) -> RecordBuilder<'a> {
        let odt: OffsetDateTime = SystemTime::now().into();
        let format = format_description!("[year]-[month]-[day]");

        self.set_attr(
            ATTR_DATE,
            &odt.format(&format)
                .expect("This date format should never fail"),
        )
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
        let template = std::fs::read_to_string(
            self.config
                .context("Config cannot be none")?
                .get_default_template_path()?,
        )?;

        debug!("Loaded record `{}`", path.display());

        let mut pattern_lines = vec![];
        let re = Regex::new(r"%(?<name>[A-Z_-]+)%").unwrap();

        // Scan each line for attributes
        for line in template.lines() {
            let mut patterns = vec![];
            let mut replace_with = vec![];

            // Collect each attribute as a regex capture
            for cap in re.captures_iter(line) {
                if let Some(name) = cap.name("name") {
                    // We need to escape all the strings to avoid special regex relevant characters
                    let name_templ =
                        regex::escape(format!("%{}%", name.as_str()).as_str()).to_string();
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

            let re = Regex::new(pat)?;

            for cap in re.captures_iter(&content) {
                debug!("capture={:?}", cap);

                for name in re.capture_names().flatten() {
                    if let Some(act_match) = cap.name(name) {
                        debug!("{:?} => {:?}", name, act_match.as_str());
                        self.attrs
                            .insert(String::from(name), String::from(act_match.as_str()));
                    }
                }
            }
        }

        debug!("attrs={:?}", self.attrs);

        Ok(self)
    }

    /// Add a content filter to the filter chain
    ///
    /// # Arguments
    ///
    /// * `filter_func` - Filter func to call
    ///
    /// # Returns
    ///
    /// A [`Result`] with either [`String`] on success or otherwise [`anyhow::Error`]
    pub(crate) fn add_content_filter(
        &mut self,
        filter_func: &'a impl Fn(&String) -> Result<String>,
    ) -> Result<&mut Self> {
        self.content_filters.push(filter_func);

        Ok(self)
    }

    /// Add a filter to the filter chain
    ///
    /// # Arguments
    ///
    /// * `filter_func` - Filter func to call
    ///
    /// # Returns
    ///
    /// A [`Result`] with either [`String`] on success or otherwise [`anyhow::Error`]
    pub(crate) fn add_record_filter(
        &mut self,
        filter_func: &'a impl Fn(&mut Record),
    ) -> Result<&mut Self> {
        self.record_filters.push(filter_func);

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
        let mut content =
            std::fs::read_to_string(self.config.unwrap().get_default_template_path()?)?;

        // Sanitize record number
        let mut num = 0;

        if let Some(maybe_num) = self.get_number() {
            num = maybe_num.parse::<i16>().unwrap_or(0);
        }

        if 0 >= num {
            num = find_next_num(&self.config.unwrap().get_record_path()?)?;
        }

        self.attrs
            .insert(String::from(ATTR_NUMBER), num.to_string());

        debug!("Using attributes {:?}", self.attrs);

        // Apply content filters
        for filter in &self.content_filters {
            content = filter(&content)?;
        }

        let mut record = Record {
            content: replace_all(&content, &self.attrs)?,
            target_path: format!(
                "{}/{:04}-{}.{}",
                self.config.unwrap().get_output_path()?.display(),
                num,
                slugify!(self.get_title().context("Title cannot be empty")?),
                &self.config.unwrap().doc_type,
            ),
        };

        // Apply record filters
        for filter in &self.record_filters {
            filter(&mut record);
        }

        Ok(record)
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
        let number = entry
            .file_name()
            .to_str()
            .with_context(|| format!("Couldn't convert {:?} to string", entry.file_name()))?
            .chars()
            .take(4)
            .collect::<String>();

        return number
            .parse::<i16>()
            .map_err(anyhow::Error::from)
            .map(|i| i + 1);
    }

    Ok(1)
}

/// Replace all attributes found in document
///
/// # Arguments
///
/// * `source` - Source content
/// * `attrs` - Map of attributes to search and replace
///
/// # Returns
///
/// A [`Result`] with either [`String`] on success or otherwise [`anyhow::Error`]
fn replace_all(source: &str, attrs: &HashMap<String, String>) -> Result<String> {
    let mut result = vec![];

    // Split into two vecs
    let mut patterns: Vec<String> = vec![];
    let mut replace_with: Vec<&str> = vec![];

    for attr in attrs.iter() {
        patterns.push(format!("%{}%", attr.0));
        replace_with.push(attr.1);
    }

    debug!("patterns={:?}, replace_width={:?}", patterns, replace_with);

    let ac = AhoCorasick::new(patterns)?;

    ac.try_stream_replace_all(source.as_bytes(), &mut result, &replace_with)?;

    Ok(String::from_utf8(result)?)
}
