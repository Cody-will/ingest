use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use crate::config::Config;


pub fn get_images(dir: &Path, cfg: &Config) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| match entry.path().extension().and_then(|e| e.to_str()) {
            Some(ext) => cfg.allows_ext(ext),
            None => false,
        })
        .map(|entry| entry.path().to_path_buf())
        .collect()
}
