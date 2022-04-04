use std::path::{Path, PathBuf};

use rand::{distributions::Alphanumeric, prelude::ThreadRng, Rng};
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


pub fn create_new_path_from_old<P: AsRef<Path>, P1: AsRef<Path>>(image_path: P, output_to: P1, rng: &mut ThreadRng) -> Result<PathBuf, std::io::Error> {
    let addition: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(22)
        .collect();

    let image_path = image_path.as_ref();

    let file_name = format!("{}_{}", addition, image_path.file_name().unwrap().to_str().unwrap());
    let folder_from_file_path = image_path.parent().unwrap();
    let folder_name = folder_from_file_path.file_name().unwrap().to_str().unwrap();
    let path = format!("{}/{}", folder_name, file_name);
    let output_to = Path::new(output_to.as_ref());
    let path = output_to.join(path);
    Ok(path)
}