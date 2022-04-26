// Build all pages

mod files;
use files::Action;

use crate::parser;

use std::{io, fs, path::PathBuf};


pub struct BuildStats {
    pub files_copied: usize,
    pub files_parsed: usize
}

impl BuildStats {
    pub fn new() -> Self {
        BuildStats {
            files_copied: 0,
            files_parsed: 0
        }
    }
}


pub fn build(from: PathBuf, to: PathBuf) -> io::Result<BuildStats> {
    let actions = files::dir_all(&from)?;
    let mut stats = BuildStats::new();

    for action in actions {
        match action {
            Action::Copy(path) => {
                let to = to.join(&path);

                if let Err(e) = fs::create_dir_all(to.parent().unwrap()) {
                    eprintln!("Error creating output directory: {}", e);
                }

                match fs::copy(&path, to) {
                    Ok(_) => stats.files_copied += 1,
                    Err(e) => eprintln!("Error copying file {}: {}", path.display(), e)
                }
            },
            Action::Parse(mut path) => {
                let orig = path.clone();
                path = path.strip_prefix(&from).unwrap().with_extension("html");
                let to = to.join(&path);

                if let Err(e) = fs::create_dir_all(to.parent().unwrap()) {
                    eprintln!("Error creating output directory: {}", e);
                }

                match fs::read_to_string(&orig) {
                    Ok(text) => {
                        let text = parser::parse(&text);

                        match fs::write(&to, text) {
                            Ok(_) => stats.files_parsed += 1,
                            Err(e) => eprintln!("Error writing file {}: {}", to.display(), e)
                        }
                    },
                    Err(e) => eprintln!("Error reading file {}: {}", orig.display(), e)
                }
            }
        }
    }

    Ok(stats)
}
