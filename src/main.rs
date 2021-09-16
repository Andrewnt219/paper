mod cli;
mod file_parser;

use cli::config::Config;

fn main() {
    let config = Config::new();

    config.run();
}
