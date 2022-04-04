use std::path::{Path, PathBuf};

use walkdir::{WalkDir, DirEntry};


pub fn get_paths<P: AsRef<Path>>(path: P) -> Result<(Vec<PathBuf>, Vec<DirEntry>), std::io::Error> {
    // paths / directories that are named after a class
    let mut entries = std::fs::read_dir(&path)?
        .map(|res| res.map(|e| e.path()))
        .flatten()
        .filter(|path| path.is_dir())
        .collect::<Vec<PathBuf>>();

    entries.sort();

    let paths: Vec<DirEntry> = WalkDir::new(path)
        .into_iter()
        .flatten()
        .collect();
        
    Ok((entries, paths))
}