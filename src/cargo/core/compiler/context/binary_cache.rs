use std::path::{PathBuf, Path};
use std::fs::metadata;

/// The global binary cache
pub struct BinaryCache {
    #[allow(dead_code)]
    max_size: usize,
    root_path: PathBuf,
    #[allow(dead_code)]
    data_filename: String,
}

impl BinaryCache {
    pub fn new(max_size: usize, root_path: PathBuf) -> Self {
        BinaryCache {
            max_size,
            root_path,
            data_filename: ".cachedata".to_string()
        }
    }

    pub fn binary_dir(&self) -> &Path {
        self.root_path.as_path()
    }

    pub fn in_cache(&self, path:&Path) -> bool {
        !metadata(path).is_ok()
    }

    #[allow(dead_code)]
    pub fn update_cache_stat(&self, _path:&str) {

    }
   
    #[allow(dead_code)]
    pub fn should_use_cache(&self, _filename:&str) -> bool { 
        true
    }
}
