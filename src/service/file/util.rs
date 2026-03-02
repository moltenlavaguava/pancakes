use anyhow::Result;
use anyhow::anyhow;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub fn setup_directories() -> Result<(PathBuf, PathBuf)> {
    // get directories
    let proj_dirs = ProjectDirs::from("", "", "pancakes")
        .ok_or_else(|| anyhow!("Failed to access project directories"))?;

    let cache_dir = proj_dirs.cache_dir();
    let data_dir = proj_dirs.data_dir();

    // make sure they actually exist
    fs::create_dir_all(cache_dir)?;
    fs::create_dir_all(data_dir)?;

    Ok((cache_dir.to_path_buf(), data_dir.to_path_buf()))
}
