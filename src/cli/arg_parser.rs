use std::{
    path::{Path, PathBuf},
    process,
};

use clap::{App, Arg, ArgMatches};

/// Represent the parsed arguments from CLI
pub struct ArgParser {
    dist_dir: PathBuf,
    stylesheet: String,
    input_paths: Vec<PathBuf>,
}

impl ArgParser {
    /// Create a new `ArgParser` with parsed CLI arguments
    pub fn new() -> ArgParser {
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
                    .about("Path to output file"),
            )
            .arg(
                Arg::new("stylesheet")
                    .short('s')
                    .long("stylesheet")
                    .value_name("URL or FILE")
                    .about("Link to stylesheet"),
            )
            .get_matches();

        ArgParser {
            dist_dir: get_output_dir(&matches),
            input_paths: get_input_paths(&matches),
            stylesheet: get_stylesheet(&matches),
        }
    }

    /// Get a reference to the arg parser's dist dir.
    pub fn dist_dir(&self) -> &PathBuf {
        &self.dist_dir
    }

    /// Get a reference to the arg parser's file paths.
    pub fn input_paths(&self) -> Vec<PathBuf> {
        self.input_paths.clone()
    }

    /// Get a reference to the arg parser's stylesheet.
    pub fn stylesheet(&self) -> &str {
        self.stylesheet.as_str()
    }
}

/// Get the output dir from CLI arg
fn get_output_dir(matches: &ArgMatches) -> PathBuf {
    let mut output_dir = PathBuf::from("./dist");
    if let Some(path) = matches.value_of("output") {
        output_dir = PathBuf::from(path);
    }

    output_dir
}

/// Get the input file(s) from CLI arg
fn get_input_paths(matches: &ArgMatches) -> Vec<PathBuf> {
    let mut input_paths = vec![];

    if let Some(i) = matches.values_of("input") {
        input_paths = i.map(|value| PathBuf::from(value)).collect();
    }

    input_paths
}

/// Get the stylesheet's URL from CLI arg
fn get_stylesheet(matches: &ArgMatches) -> String {
    let mut url = "asset/style.css";
    if let Some(value) = matches.value_of("stylesheet") {
        url = value;
    }

    url.to_string()
}
