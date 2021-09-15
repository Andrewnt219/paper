use clap::{App, Arg, ArgMatches};

pub struct Config {
    pub matches: ArgMatches,
}

impl Config {
    pub fn new() -> Config {
        let matches = App::new("Paper")
            .version("0.1")
            .author("Andrew N. <hey@andrewnt.dev>")
            .about("Generate static site")
            .arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .value_name("FILE")
                    .about("Path to the file"),
            )
            .get_matches();

        Config { matches }
    }
}
