use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    process,
};

use clap::{App, Arg};

use crate::file_parser::{file_parser::SourceFile, template_file::Template};

use super::arg_parser::ArgParser;

pub struct Generator {
    args: ArgParser,
}

impl Generator {
    pub fn new() -> Generator {
        Generator {
            args: ArgParser::new(),
        }
    }

    fn generate_dist(&self) {
        for file_path in &self.args.file_paths() {
            let file = SourceFile::new(&file_path).unwrap_or_else(|err| {
                println!("Problem parsing '{}': {}", &file_path, err);
                process::exit(1);
            });

            let dest_path =
                Path::new(&self.args.dist_dir()).join(format!("{}.html", file.file_stem()));

            let mut template = Template::new();
            template.parse(file.content());

            File::create(&dest_path)
                .and_then(|mut file| file.write_all(template.content().as_bytes()))
                .unwrap_or_else(|error| {
                    println!("Problem generating '{}': {}", &file_path, error);
                    process::exit(1);
                });
        }
    }

    pub fn run(&self) {
        create_dir_all(&self.args.dist_dir()).unwrap_or_else(|error| {
            println!("Failed to create dist: {}", error);
            process::exit(1);
        });

        self.generate_dist();
    }
}
