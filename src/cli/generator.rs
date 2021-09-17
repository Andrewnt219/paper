use std::{
    ffi::OsStr,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process,
};

use crate::file_parser::{source_file::SourceFile, template_file::Template};

use super::arg_parser::ArgParser;

/// The core system for managing static site generation
pub struct Generator {
    args: ArgParser,
}

impl Generator {
    /// Create a new generator
    pub fn new() -> Generator {
        Generator {
            args: ArgParser::new(),
        }
    }

    /// Start generating .html files
    pub fn run(&self) {
        self.create_dist_dir();
        self.generate_dist();
        self.generate_stylesheet();
        self.generate_dist_index_file();
    }

    /// Create the dist dir for .html files
    fn create_dist_dir(&self) {
        if self.args.dist_dir().is_dir() {
            fs::remove_dir_all(self.args.dist_dir()).unwrap_or_else(|error| {
                println!("Fail to remove dist dir: {}", error);
                process::exit(1);
            })
        }

        fs::create_dir_all(&self.args.dist_dir()).unwrap_or_else(|error| {
            println!("Failed to create dist: {}", error);
            process::exit(1);
        });
    }

    /// Generate dist files from input files
    fn generate_dist(&self) {
        for input_path in self.args.input_paths() {
            self.generate_dist_from_path(&input_path);
        }
    }

    /// Recursively generate dist files from a path
    fn generate_dist_from_path(&self, path: &PathBuf) {
        if path.is_dir() {
            self.generate_dist_from_dir(&path);
        }

        if path.is_file() {
            self.generate_dist_from_file(&path);
        }

        println!("Path is not regconize as file or dir. Try removing trailing slash");
        process::exit(0);
    }

    /// Recursively gEnerate dist file from a dir path
    fn generate_dist_from_dir(&self, dir_path: &PathBuf) {
        if !dir_path.is_dir() {
            return;
        }

        fs::create_dir_all(self.args.dist_dir().join(dir_path)).unwrap_or_else(|error| {
            println!("Fail to generate dir from '{}'", error);
            process::exit(1);
        });

        if let Ok(dir) = fs::read_dir(dir_path) {
            for entry in dir {
                if let Ok(entry) = entry {
                    self.generate_dist_from_path(&entry.path());
                }
            }
        }
    }

    /// Generate dist from a file path
    fn generate_dist_from_file(&self, file_path: &PathBuf) {
        if !file_path.is_file() {
            return;
        }

        let file = SourceFile::new(&file_path).unwrap_or_else(|err| {
            println!("Problem parsing '{}': {}", file_path.display(), err);
            process::exit(1);
        });

        let file_path_prefix = file_path.parent().unwrap_or_else(|| {
            println!("Fail to get path prefix at '{}'", file_path.display());
            process::exit(1);
        });

        let dest_path_prefix = Path::new(&self.args.dist_dir()).join(file_path_prefix);

        fs::create_dir_all(&dest_path_prefix).unwrap_or_else(|error| {
            println!(
                "Fail to create dir(s) for '{}': {}",
                file_path_prefix.display(),
                error
            );
            process::exit(1);
        });

        let dest_path = dest_path_prefix.join(format!("{}.html", file.file_stem()));

        let mut template = Template::new();
        template.parse(file.content(), &self.args);

        File::create(&dest_path)
            .and_then(|mut file| file.write_all(template.content().as_bytes()))
            .unwrap_or_else(|error| {
                println!(
                    "Problem generating file '{}': {}",
                    file_path.display(),
                    error
                );
                process::exit(1);
            });
    }

    fn generate_stylesheet(&self) {
        let stylesheet_path = PathBuf::from(self.args.stylesheet());

        if !stylesheet_path.is_file() {
            return;
        };

        let dest_path = self.args.dist_dir().join(&stylesheet_path);
        let dir_path = self
            .args
            .dist_dir()
            .join(&stylesheet_path.parent().unwrap_or_else(|| {
                println!("Fail to get parent file of '{}'", stylesheet_path.display());
                process::exit(1);
            }));

        fs::create_dir_all(dir_path).unwrap_or_else(|error| {
            println!("Fail to create dir for stylesheet: {}", error);
            process::exit(1);
        });

        fs::copy(&stylesheet_path, &dest_path).unwrap_or_else(|error| {
            println!(
                "Fail to copy stylesheet from {} to {}: {}",
                stylesheet_path.display(),
                dest_path.display(),
                error
            );
            process::exit(1);
        });
    }

    /// Create the index.html file
    fn generate_dist_index_file(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(self.args.dist_dir().join(PathBuf::from("index.html")))
            .unwrap_or_else(|error| {
                println!("Fail to create dist index.html: {}", error);
                process::exit(1);
            });

        for path in &self.read_paths(self.args.dist_dir()) {
            file.write_all(
                format!(
                    "<a style='display:block' href='{}'>{}</a>",
                    path.display(),
                    path.file_stem().unwrap().to_str().unwrap()
                )
                .as_bytes(),
            )
            .unwrap_or_else(|error| {
                println!("Fail to write path '{}': {}", path.display(), error);
                process::exit(1);
            });
        }
    }

    /// Add relative links to index.html
    fn read_paths(&self, path: &PathBuf) -> Vec<PathBuf> {
        let mut paths: Vec<PathBuf> = Vec::new();

        if let Ok(dir) = fs::read_dir(path) {
            for entry in dir {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_dir() {
                        paths.append(&mut self.read_paths(&path));
                    }

                    if path.is_file() && !path.ends_with("index.html") {
                        let ext = path.extension().unwrap_or(OsStr::new(""));

                        if ext.eq("html") {
                            let path = path
                                .strip_prefix(self.args.dist_dir())
                                .unwrap_or_else(|error| {
                                    println!("Fail to strip prefix: {}", error);
                                    process::exit(9);
                                })
                                .to_path_buf();

                            paths.push(path);
                        }
                    }
                }
            }
        }

        paths
    }
}
