mod cli;

use cli::config::Config;

fn main() {
    let Config { matches } = Config::new();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(i) = matches.value_of("input") {
        println!("Value for input: {}", i);
    }
}
