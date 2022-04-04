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

impl Default for ImageReturn {
    fn default() -> Self {
        Self { found_in: Default::default(), exact_paths: Default::default(), data: Default::default(), channels: Default::default(), dims: Default::default() }
    }
}