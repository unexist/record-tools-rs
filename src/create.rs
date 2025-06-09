///
/// @package record-tools-rs
///
/// @file Main file
/// @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv3.
/// See the file LICENSE for details.
///

use crate::Config;

pub(crate) fn create(title: String, config: &Config) {
    println!("title: {}, superseded: {}", title, config.option_s);
}