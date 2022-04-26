// Utility for converting Markdown to HTML

pub mod metadata;
pub mod document;

use pulldown_cmark::{Parser, html};


pub fn parse(text: &str) -> String {
    let (meta, text) = metadata::extract(text.lines());
    let mut output = String::new();

    let parser = Parser::new(&text);
    html::push_html(&mut output, parser);

    document::format(meta, output)
}
