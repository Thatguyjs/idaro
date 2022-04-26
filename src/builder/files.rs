use std::{io, fs, path::PathBuf, ffi::OsStr};


#[derive(PartialEq)]
pub enum Action {
    Copy(PathBuf),
    Parse(PathBuf)
}


pub fn dir(path: &PathBuf) -> io::Result<(Vec<Action>, Vec<PathBuf>)> {
    let mut files = vec![];
    let mut dirs = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?.path();

        if entry.is_file() {
            if entry.extension() == Some(&OsStr::new("md")) {
                files.push(Action::Parse(entry));
            }
            else {
                files.push(Action::Copy(entry));
            }
        }
        else if entry.is_dir() {
            dirs.push(entry);
        }
        else {
            eprintln!("Unsupported directory entry, skipping: {:?}", entry);
        }
    }

    Ok((files, dirs))
}

// Finds all files in a directory, including subdirectories
pub fn dir_all(path: &PathBuf) -> io::Result<Vec<Action>> {
    let (mut files, mut dirs) = dir(path)?;

    while !dirs.is_empty() {
        let mut new_dirs = vec![];

        for d in &dirs {
            let (mut new_files, mut temp_dirs) = dir(d)?;

            files.append(&mut new_files);
            new_dirs.append(&mut temp_dirs);
        }

        dirs = new_dirs;
    }

    Ok(files)
}
