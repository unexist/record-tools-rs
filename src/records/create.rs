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

pub(crate) fn create(title: String, config: &Config) {
    println!("Created new decision record (title: {}, type: {}, superseded: {})",
             title, config.record_type, config.superseded);
}