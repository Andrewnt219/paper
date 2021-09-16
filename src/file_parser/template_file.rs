use std::{fs::read_to_string, path::Path, process};

pub struct Template {
    content: String,
}

impl Template {
    pub fn new() -> Template {
        let content = read_to_string(Path::new("asset/template.html")).unwrap_or_else(|error| {
            println!("Fail to read template file: {}", error);
            process::exit(1);
        });

        return Template { content };
    }

    fn set_title(&mut self, title: &str) {
        self.content = self.content.replace("$TITLE", title);
    }

    fn set_body(&mut self, body: &str) {
        self.content = self.content.replace("$BODY", body);
    }

    pub fn content(&self) -> &str {
        self.content.as_str()
    }

    pub fn parse(&mut self, content: &str) {
        self.parse_body(content);
    }

    fn parse_body(&mut self, body: &str) {
        let mut result = String::from("");

        for line in body.lines() {
            result += format!("<p>{}</p>", line).as_str();
        }

        self.set_body(body);
    }
}
