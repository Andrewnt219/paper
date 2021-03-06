use std::{
    fs,
    path::{Path, PathBuf},
    process,
};

use crate::cli::arg_parser::ArgParser;
use crate::file_parser::source_file::SourceFile;
use crate::file_parser::markdown_parser::MarkdownDocument;

pub enum TemplateState {
    PARSED,
    RAW,
}

/**
A struct for parsing raw file to html

## Examples

```rust
let args = ArgsParser::new();
let content = "This is a text";

let mut template = Template::new();
template.parse(content, args);

println!("{}", template.content());
```
*/
pub struct Template {
    /// The content of the template
    /// Call parse() to consume the current content
    /// and generate a parsed content
    content: String,
    /// The current parsing state of the template
    state: TemplateState,
}

impl Template {
    /// Create a template with raw content
    pub fn new() -> Template {
        let content = include_str!("./asset/template.html").to_string();
        return Template {
            content,
            state: TemplateState::RAW,
        };
    }

    /// Get a reference to the template's state.
    pub fn state(&self) -> &TemplateState {
        &self.state
    }

    /// Replace the title inside content
    fn set_title(&mut self, title: &str) {
        self.content = self.content.replace("$TITLE", title);
    }

    /// Replace the stylesheet url inside content
    fn set_styleshet(&mut self, url: &str) {
        self.content = self.content.replace("$STYLESHEET_URL", url);
    }

    /// Replace the body inside content
    fn set_body(&mut self, body: &str) {
        self.content = self.content.replace("$BODY", body);
    }

    /// Get the current content
    pub fn content(&self) -> &str {
        self.content.as_str()
    }

    pub fn parse(&mut self, source_file: &SourceFile, args: &ArgParser) {
	if source_file.ext() == "txt" {
	    self.parse_raw_text(source_file.content(), args);
	} else if source_file.ext() == "md" {
	    self.parse_markdown_text(source_file.content(), args);
	}
    }

    /// Parse the raw content into html content
    fn parse_raw_text(&mut self, content: &str, args: &ArgParser) {
        let mut body = String::from("");
        let mut title = String::from("");
        let mut blank_line_count = 0;

        // Parse lines into html
        for (index, line) in content.trim().lines().enumerate() {
            match line {
                "" if index <= 2 => blank_line_count += 1,
                s => {
                    if index == 0 {
                        title += s;
                    } else {
                        body += parse_body(s).as_str();
                    }
                }
            }
        }

        // Corresponding actions if the source has a title or not
        if blank_line_count == 2 {
            body = format!("<h1>{}</h1>{}", title, body);
        } else {
            body = parse_body(&title) + &body;
            title.clear();
        }

        // Update the raw content
        self.set_title(&title);
        self.set_body(&body);
        self.set_styleshet(parse_stylesheet_url(args.stylesheet()).as_str());
        self.state = TemplateState::PARSED;
    }

    fn parse_markdown_text(&mut self, content: &str, args: &ArgParser) {
	let mut body = parse_markdown_to_html(content);
	let mut title = String::from("");

	self.set_title(&title);
	self.set_body(&body);
	self.set_styleshet(parse_stylesheet_url(args.stylesheet()).as_str());
	self.state = TemplateState::PARSED;
    }
}

fn parse_markdown_to_html(content: &str) -> String {
    let mut doc = MarkdownDocument::new();

    for line in content.lines() {
	doc.add_line_to_document(&line);
    }

    doc.print()
}

/// parse the content to suitable html tags
fn parse_body(content: &str) -> String {
    format!("<p>{}</p>", content)
}

/// Parse stylesheet url to <style> or <link>
fn parse_stylesheet_url(url: &str) -> String {
    let path = PathBuf::from(url);

    if path.is_file() {
        let content = fs::read_to_string(&path).unwrap_or_else(|error| {
            println!(
                "Fail to read content of stylesheet at '{}': {}",
                path.display(),
                error
            );
            process::exit(0);
        });

        format!("<style>{}</style>", content)
    } else {
        format!("<link rel='stylesheet' href='{}' />", url)
    }
}
