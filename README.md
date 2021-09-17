# paper-cli

Static site generator (SSG) made with [Rust](https://www.rust-lang.org/)

To start running:

1. [Install Rust](https://www.rust-lang.org/tools/install)

2. Run

```bash
$ cargo run -- --help

USAGE:
    paper.exe [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -i, --input <FILE>...             Path to file(s)
    -o, --output <FILE>               Path to output file
    -s, --stylesheet <URL or FILE>    Link to stylesheet
```

## Implemented optional features

#### 🎉 Generate `index.html`

The index file includes paths to all the generated html files (recursively)

```bash
$ cargo run -- -i dir-with-nested-dirs-and-files
```

#### 🌟 Pass in stylesheet's file OR url as a CLI arg

Content of `.css` files are bundled into all the generated `.html` files

```bash
$ cargo run -- -i page.txt --stylesheet ./my-style.css
```

#### 🎉 Keep source folder structure

If a directory is passed as `--input`, `dist` keeps the structure of the source dir

```bash
$ cargo run -- -i sample-dir

├── sample-dir
├── Cargo.toml
├── sample-dir
│   ├── sub-dir-1
│   └── sub-dir-2
│       └── page-1.txt
├── dist
│   ├─ sample-dir
│       ├── sub-dir-1
│       └── sub-dir-2
│           └── page-1.html
```

#### 🌟 Parse title

Title is the first line of the file, followed by 2 empty lines

#### 🌟 Pass in output dir as argument

Specify a different output directory, default is `dist`

```bash
$ cargo run -- -i sample.txt --output pages
```
