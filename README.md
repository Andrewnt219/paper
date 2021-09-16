# paper-cli

Static site generator (SSG) made with [Rust](https://www.rust-lang.org/)

To start running:

1. [Install Rust](https://www.rust-lang.org/tools/install)

2. Run

```bash
$ cargo run -- --help

Paper 0.1

Andrew N. <hey@andrewnt.dev>

Generate static site

USAGE:
    paper.exe [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -i, --input <FILE>...     Path to file(s)
    -o, --output <FILE>       Path to output file
    -s, --stylesheet <URL>    Link to stylesheet
```

## Implemented optional features

- Parse title
- Pass in output dir as argument
- Pass in stylesheet's url as argument
