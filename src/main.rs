mod cli;
mod file_parser;

use cli::generator::Generator;

fn main() {
    let generator = Generator::new();

    generator.run();
}
