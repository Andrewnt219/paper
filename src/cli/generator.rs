use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    process,
};

use crate::file_parser::{source_file::SourceFile, template_file::Template};

use super::arg_parser::ArgParser;

/// The core system for managing static site generation
pub struct Generator {
    args: ArgParser,
}

impl Generator {
    /// Create a new generator
    pub fn new() -> Generator {
        Generator {
            args: ArgParser::new(),
        }
    }

    /// Start generating .html files
    pub fn run(&self) {
        self.create_dist_dir();
        self.generate_dist();
    }

    /// generate .html files from input files
    fn generate_dist(&self) {
        for file_path in &self.args.file_paths() {
            let file = SourceFile::new(&file_path).unwrap_or_else(|err| {
                println!("Problem parsing '{}': {}", &file_path, err);
                process::exit(1);
            });

            let dest_path =
                Path::new(&self.args.dist_dir()).join(format!("{}.html", file.file_stem()));

            let mut template = Template::new();
            template.parse(file.content(), &self.args);

            File::create(&dest_path)
                .and_then(|mut file| file.write_all(template.content().as_bytes()))
                .unwrap_or_else(|error| {
                    println!("Problem generating '{}': {}", &file_path, error);
                    process::exit(1);
                });
        }
    }

    /// Create the dist dir for .html files
    fn create_dist_dir(&self) {
        create_dir_all(&self.args.dist_dir()).unwrap_or_else(|error| {
            println!("Failed to create dist: {}", error);
            process::exit(1);
        });
    }
}
