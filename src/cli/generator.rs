use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
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

    /// Create the dist dir for .html files
    fn create_dist_dir(&self) {
        fs::create_dir_all(&self.args.dist_dir()).unwrap_or_else(|error| {
            println!("Failed to create dist: {}", error);
            process::exit(1);
        });
    }

    /// Generate dist files from input files
    fn generate_dist(&self) {
        for input_path in self.args.input_paths() {
            self.generate_dist_from_path(&input_path);
        }
    }

    /// Recursively generate dist files from a path
    fn generate_dist_from_path(&self, path: &PathBuf) {
        if path.is_dir() {
            self.generate_dist_from_dir(&path);
        }

        if path.is_file() {
            self.generate_dist_from_file(&path);
        }
    }

    /// Recursively gEnerate dist file from a dir path
    fn generate_dist_from_dir(&self, dir_path: &PathBuf) {
        if !dir_path.is_dir() {
            return;
        }

        if let Ok(paths) = fs::read_dir(dir_path) {
            for path in paths {
                if let Ok(dir_entry) = path {
                    self.generate_dist_from_path(&dir_entry.path());
                }
            }
        }
    }

    /// Generate dist from a file path
    fn generate_dist_from_file(&self, file_path: &PathBuf) {
        if !file_path.is_file() {
            return;
        }

        let file = SourceFile::new(&file_path).unwrap_or_else(|err| {
            println!("Problem parsing '{}': {}", file_path.display(), err);
            process::exit(1);
        });

        let dest_path = Path::new(&self.args.dist_dir()).join(format!("{}.html", file.file_stem()));

        let mut template = Template::new();
        template.parse(file.content(), &self.args);

        File::create(&dest_path)
            .and_then(|mut file| file.write_all(template.content().as_bytes()))
            .unwrap_or_else(|error| {
                println!("Problem generating '{}': {}", file_path.display(), error);
                process::exit(1);
            });
    }
}
