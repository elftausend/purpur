use std::path::{Path, PathBuf};

use rand::{distributions::Alphanumeric, prelude::{ThreadRng, SliceRandom}, Rng};
use walkdir::WalkDir;


pub fn get_paths<P: AsRef<Path>>(path: P) -> Result<(Vec<PathBuf>, Vec<PathBuf>), std::io::Error> {
    // paths / directories that are named after a class
    let mut entries = std::fs::read_dir(&path)?
        .flat_map(|res| res.map(|e| e.path()))
        .filter(|path| path.is_dir())
        .collect::<Vec<PathBuf>>();

    entries.sort();

    let paths: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .flat_map(|entry| entry.map(|e| e.path().to_path_buf()))
        .filter(|path| 
            path.is_file() && 
            !path.file_name().unwrap().to_str().unwrap().starts_with("."))
        .collect();
        
    Ok((entries, paths))
}

pub fn create_path_with_hash<P: AsRef<Path>, P1: AsRef<Path>>(image_path: P, output_to: P1, hash: u64) -> Result<PathBuf, std::io::Error> {
    let image_path = image_path.as_ref();

    let file_name = format!("{}_{}", hash, image_path.file_name().unwrap().to_str().unwrap());
    let folder_from_file_path = image_path.parent().unwrap();
    let folder_name = folder_from_file_path.file_name().unwrap().to_str().unwrap();
    let path = format!("{}/{}", folder_name, file_name);
    let output_to = Path::new(output_to.as_ref());
    let path = output_to.join(path);
    Ok(path)
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


pub fn shuffle<T: Copy, G: Copy>(rows: usize, data: &[T], y_values: &[G]) -> (Vec<T>, Vec<G>) {
    let mut x = vec![0usize; rows];
    for idx in 1..rows {
        x[idx] = idx;
    }


    let mut rng = rand::thread_rng();
    x.shuffle(&mut rng);

    let mut shuffeled = Vec::<T>::new();

    let mut y_shuffeled = Vec::<G>::new();

    let cols = data.len() / rows;
    
    for idx in x.iter() {

        unsafe {
            y_shuffeled.push(*y_values.get_unchecked(*idx));
        }
        let i = idx*cols;
        let row = &data[i..i+cols];
        for value in row {
            shuffeled.push(*value);
        }
    }
    
    (shuffeled, y_shuffeled)
}

///(via clone) reduces the len of an array, which is randomized, to a given max len
pub fn random_short_mut<T: Clone+Default>(reduce_arr: &mut [T], max_len: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    let mut final_arr = Vec::<T>::with_capacity(max_len);

    reduce_arr.shuffle(&mut rng);
    for elem in &reduce_arr[..max_len] {
        final_arr.push(elem.clone());
    }
    final_arr
}

pub fn min<T: Copy+PartialOrd>(array: &[T]) -> T {
    let mut min = array[0];
    for value in array {
        if value < &min {
            min = *value;
        }
    }
    min
}

pub fn max<T: Copy+PartialOrd>(array: &[T]) -> T {
    let mut max = array[0];
    for value in array {
        if value > &max {
            max = *value;
        }
    }
    max
}

/// search_for: [0.1, 0.6, 4.,], 
/// search_with: [4.]; 
/// output: [2]
pub fn find_idxs<T: Copy+PartialEq>(rows: usize, search_for: &[T], search_with: &[T]) -> Vec<usize> {
    let cols = search_for.len()/rows;

    let mut idxs = vec![0usize; rows];

    for idx in 0..rows {
        let index = idx*cols;
        let row = &search_for[index..index+cols];
        for (row_idx, value) in row.iter().enumerate() {
            if *value == search_with[idx] {
                idxs[idx] = row_idx;
            }
        }
    }
    idxs
}

