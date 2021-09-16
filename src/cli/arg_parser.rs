use clap::{App, Arg, ArgMatches};

/// Represent the parsed arguments from CLI
pub struct ArgParser {
    dist_dir: String,
    stylesheet: String,
    file_paths: Vec<String>,
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
                    .value_name("URL")
                    .about("Link to stylesheet"),
            )
            .get_matches();

        ArgParser {
            dist_dir: get_output_dir(&matches),
            file_paths: get_input_files(&matches),
            stylesheet: get_stylesheet(&matches),
        }
    }

    /// Get a reference to the arg parser's dist dir.
    pub fn dist_dir(&self) -> &str {
        self.dist_dir.as_str()
    }

    /// Get a reference to the arg parser's file paths.
    pub fn file_paths(&self) -> Vec<String> {
        self.file_paths.clone()
    }

    /// Get a reference to the arg parser's stylesheet.
    pub fn stylesheet(&self) -> &str {
        self.stylesheet.as_str()
    }
}

/// Get the output dir from CLI arg
fn get_output_dir(matches: &ArgMatches) -> String {
    let mut dir = "dist";
    if let Some(path) = matches.value_of("output") {
        dir = path;
    }

    dir.to_string()
}

/// Get the input file(s) from CLI arg
fn get_input_files(matches: &ArgMatches) -> Vec<String> {
    let mut input_files = vec![];

    if let Some(i) = matches.values_of("input") {
        input_files = i.map(|value| value.to_string()).collect();
    }

    input_files
}

/// Get the stylesheet's URL from CLI arg
fn get_stylesheet(matches: &ArgMatches) -> String {
    let mut url = "style.css";
    if let Some(value) = matches.value_of("stylesheet") {
        url = value;
    }

    url.to_string()
}
