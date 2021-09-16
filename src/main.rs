mod cli;
mod file_parser;

use cli::config::Config;

fn main() {
    let Config { matches } = Config::new();

    paper::parse_files(&matches);
}
