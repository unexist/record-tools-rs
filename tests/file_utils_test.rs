///
/// @package record-tools-rs
///
/// @file File utils test
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

#[path = "../src/records/file_utils.rs"]
mod file_utils;

use proptest::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempfile::TempDir;
use anyhow::Result;

macro_rules! file_pattern_str {
    () => {
        "{:04}-test-adr.adoc"
    };
}

fn create_n_records(n: u16, content: Option<&str>) -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    
    for i in 1..n {
        let mut file = File::create(
            Path::new(&temp_dir.path().join(format!(file_pattern_str!(), i))))?;
        
        if content.is_some() {
            file.write_all(content.unwrap_or_default().as_bytes())?;
            file.flush()?;
        }
    }
    
    Ok(temp_dir)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(5))]
    #[test]
    fn should_find_next_file_number(n in 1u16..20) {
        let temp_dir = create_n_records(n, None)
            .expect("Can't create temp dir");

        let number = file_utils::find_next_num(&temp_dir.path());

        // Todo: Refactor once assert_matches is stable
        assert!(number.is_ok());
        assert_eq!(number.unwrap(), n);
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(5))]
    #[test]
    fn should_extract_field(n in 1u16..5) {
        let temp_dir = create_n_records(n, Some("| Status: | drafted"))
            .expect("Can't create temp dir");
        
        let field = file_utils::extract_field(
            &temp_dir.path().join(format!(file_pattern_str!(), n)), "Status");

        println!("{:?}", field);
        
        // Todo: Refactor once assert_matches is stable
        assert!(field.is_ok());
        assert_eq!(field.unwrap(), "drafted");
    }
}