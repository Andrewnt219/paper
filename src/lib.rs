mod file_parser;

use std::{fs::create_dir_all, io, process};

use clap::ArgMatches;
use file_parser::file_parser::SourceFile;

/// Parse all the specified files into `.html` files
pub fn parse_files(matches: &ArgMatches) {
    create_dist_dir();

    if let Some(i) = matches.values_of("input") {
        for file_path in i {
            parse_file_to_html(file_path);
        }
    }
}

fn create_dist_dir() {
    create_dir_all("dist").unwrap_or_else(|error| {
        println!("Failed to create dist: {}", error);
        process::exit(1);
    });
}

fn parse_file_to_html(file_path: &str) {
    let source_file = SourceFile::new(file_path).unwrap_or_else(|err| {
        println!("Problem parsing '{}': {}", file_path, err);
        process::exit(1);
    });

    source_file.write_to_html().unwrap_or_else(|err| {
        println!(
            "Problem generating file '{}': {}",
            source_file.file_name(),
            err
        );
        process::exit(1);
    });
}
