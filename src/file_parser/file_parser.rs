use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{self, ErrorKind, Write},
    path::Path,
};

use super::template_file::Template;

pub struct SourceFile {
    content: String,
    file_name: String,
    file_stem: String,
    ext: String,
}

impl SourceFile {
    pub fn new(path_to_file: &str) -> Result<SourceFile, &str> {
        let path = Path::new(path_to_file);

        let content = match fs::read_to_string(path) {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => return Err("File does not exist"),
                _ => return Err("Fail to open file"),
            },
        };

        let file_stem = match path.file_stem().and_then(OsStr::to_str) {
            Some(string) => string,
            None => "",
        }
        .to_string();

        let ext = match path.extension().and_then(OsStr::to_str) {
            Some(string) => string,
            None => "",
        }
        .to_string();

        let file_name = match path.file_name().and_then(OsStr::to_str) {
            Some(string) => string,
            None => "",
        }
        .to_string();

        Ok(SourceFile {
            content,
            file_stem,
            file_name,
            ext,
        })
    }

    /// Get a reference to the text file's content.
    pub fn content(&self) -> &str {
        self.content.as_str()
    }

    /// Get a reference to the text file's file name.
    pub fn file_stem(&self) -> &str {
        self.file_stem.as_str()
    }

    /// Get a reference to the text file's ext.
    pub fn ext(&self) -> &str {
        self.ext.as_str()
    }

    pub fn file_name(&self) -> &str {
        self.file_name.as_str()
    }

    pub fn write_to_html(&self) -> Result<(), io::Error> {
        let dest_path = Path::new("dist").join(format!("{}.html", self.file_stem));

        let mut template = Template::new();
        template.parse(self.content());

        File::create(&dest_path)
            .and_then(|mut file| file.write_all(template.content().as_bytes()))?;

        Ok(())
    }
}
