use std::{
    ffi::OsStr,
    fs::{self, File},
    io::ErrorKind,
    path::Path,
};

/// Represents the parsed source file
pub struct SourceFile {
    content: String,
    file_name: String,
    file_stem: String,
    ext: String,
}

impl SourceFile {
    /// Create a new `SourceFile` with parsed content and metadata
    pub fn new(path_to_file: &str) -> Result<SourceFile, &str> {
        let path = Path::new(path_to_file);

        let content = match fs::read_to_string(path) {
            Ok(string) => string,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => return Err("File does not exist"),
                _ => return Err("Fail to open file"),
            },
        };

        let file_stem = parse_os_str_to_string(path.file_stem());
        let ext = parse_os_str_to_string(path.extension());
        let file_name = parse_os_str_to_string(path.file_name());

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
}

/// Try parsing the os_str, fallback to empty string
fn parse_os_str_to_string(os_str: Option<&OsStr>) -> String {
    match os_str.and_then(OsStr::to_str) {
        Some(string) => string,
        None => "",
    }
    .to_string()
}
