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
                    .multiple_values(true)
                    .short('i')
                    .long("input")
                    .value_name("FILE")
                    .about("Path to file(s)"),
            )
            .get_matches();

        Config { matches }
    }
}
