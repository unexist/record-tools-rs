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

use std::fs::File;
use std::path::Path;
use proptest::prelude::*;
use tempfile::TempDir;
use anyhow::Result;

fn create_n_records(n: u16) -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    
    for i in 1..n {
        File::create(
            Path::new(&temp_dir.path().join(format!("{:04}-test-adr.adoc", i))))
            .expect("Can't create file");
    }
    
    Ok(temp_dir)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(5))]
    #[test]
    fn should_find_next_file_number(n in 1u16..20) {
        let temp_dir = create_n_records(n)
            .expect("Can't create temp dir");


        let number = file_utils::find_next_num(temp_dir.path().to_str()
            .expect("Can't get path of temp dir"));

        // Todo: Refactor once assert_matches is stable
        assert!(number.is_ok());
        assert_eq!(number.unwrap(), n);
    }
}