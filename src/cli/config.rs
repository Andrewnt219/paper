use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    process,
};

use clap::{App, Arg, ArgMatches};

use crate::file_parser::{file_parser::SourceFile, template_file::Template};

pub struct Config {
    matches: ArgMatches,
    dist_dir: String,
    file_paths: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        let matches = App::new("Paper")
            .version("0.1")
            .author("Andrew N. <hey@andrewnt.dev>")
            .about("Generate static site")
            .arg(
                Arg::new("input")
                    .multiple_values(true)
                    .short('i')
                    .long("input")
                    .value_name("FILE")
                    .about("Path to file(s)"),
            )
            .arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .about("Path to file"),
            )
            .get_matches();

        Config {
            dist_dir: get_output_dir(&matches),
            file_paths: get_input_files(&matches),
            matches,
        }
    }

    fn write_to_html(&self) {
        for file_path in &self.file_paths {
            let file = SourceFile::new(&file_path).unwrap_or_else(|err| {
                println!("Problem parsing '{}': {}", &file_path, err);
                process::exit(1);
            });

            let dest_path = Path::new(&self.dist_dir).join(format!("{}.html", file.file_stem()));

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
        create_dir_all(&self.dist_dir).unwrap_or_else(|error| {
            println!("Failed to create dist: {}", error);
            process::exit(1);
        });

        self.write_to_html();
    }
}

fn get_output_dir(matches: &ArgMatches) -> String {
    let mut dir = "dist";
    if let Some(path) = matches.value_of("output") {
        dir = path;
    }

    dir.to_string()
}

fn get_input_files(matches: &ArgMatches) -> Vec<String> {
    let mut input_files = vec![];

    if let Some(i) = matches.values_of("input") {
        input_files = i.map(|value| value.to_string()).collect();
    }

    input_files
}
