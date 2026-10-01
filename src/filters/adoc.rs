//!
//! @package record-tools-rs
//!
//! @file List records as digraph
//! @copyright 2025-present Christoph Kappel <christoph@unexist.dev>
//! @version $Id$
//!
//! This program can be distributed under the terms of the GNU GPLv3.
//! See the file LICENSE for details.
//!

use anyhow::Result;
use asciidocr::{backends::htmls::render_htmlbook, parser::Parser, scanner::Scanner};
use log::debug;
use std::env;

/// Execute command
///
/// # Arguments
///
/// * `content` - Content to filter here
///
/// # Returns
///
/// A [`Result`] with either [`String`] on success or otherwise [`anyhow::Error`]
pub(crate) fn filter(content: &String) -> Result<String> {
    let mut html_content: String = String::new();

    if let Ok(asg) = Parser::new(env::current_dir()?).parse(Scanner::new(content)) {
        if let Ok(html) = render_htmlbook(&asg) {
            debug!("HTML out: {}", html);

            html_content = html.to_string().replace(
                "</head>",
                r#"<style type="text/css">${CSS}</style>
</head>"#,
            );

            html_content = html_content.replace(
                "<body>",
                r#"<body>
<div id="header">
<h1>${TITLE}</h1>
<div id="details">${DATE}</div>
</div>
<div id="content">"#,
            );

            html_content = html_content.replace(
                "</body>",
                r#"<div id="footer"></div>
                </body>"#,
            );
        }
    }

    Ok(html_content)
}
