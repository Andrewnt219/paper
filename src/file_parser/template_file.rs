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
        let mut body = String::from("");
        let mut title = String::from("");
        let mut blank_line_count = 0;

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

        if blank_line_count == 2 {
            body = format!("<h1>{}</h1>{}", title, body);
        } else {
            body = parse_body(&title) + &body;
            title.clear();
        }

        self.set_title(&title);
        self.set_body(&body);
    }
}

fn parse_body(content: &str) -> String {
    format!("<p>{}</p>", content)
}
