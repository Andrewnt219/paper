use std::{fs::read_to_string, path::Path, process};

use crate::cli::arg_parser::ArgParser;

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
        let content = read_to_string(Path::new("asset/template.html")).unwrap_or_else(|error| {
            println!("Fail to read template file: {}", error);
            process::exit(1);
        });

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

    /// Parse the raw content into html content
    pub fn parse(&mut self, content: &str, args: &ArgParser) {
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
        self.set_styleshet(args.stylesheet());
        self.state = TemplateState::PARSED;
    }
}

// parse the content to suitable html tags
fn parse_body(content: &str) -> String {
    format!("<p>{}</p>", content)
}
