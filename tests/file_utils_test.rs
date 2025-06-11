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

#[test]
fn should_find_next_number() {
    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"),
                       "example/src/site/asciidoc/architecture-decision-records");

    let number = file_utils::find_next_val(&*path);
    
    // Todo: Refactor once assert_matches is stable
    assert!(number.is_ok());
    assert_eq!(number.unwrap(), 1);
}