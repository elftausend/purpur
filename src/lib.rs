mod utils;
mod transforms;

pub use utils::*;
pub use transforms::*;


use std::path::PathBuf;
use walkdir::DirEntry;

pub struct ImageReturn {
    pub found_in: Vec<PathBuf>,
    pub exact_paths: Vec<DirEntry>,
    pub data: Vec<u8>,
    pub channels: Vec<u8>,
    pub dims: Vec<(usize, usize)>,
}