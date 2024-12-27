use std::{
    fs,
    path::{Path, PathBuf},
};

use tempdir::TempDir;
use uuid::Uuid;

const TEMP_DIR_PREFIX: &str = "test_tools";

pub(crate) fn setup_temp_dir() -> TempDir {
    TempDir::new(TEMP_DIR_PREFIX).expect("Failed to create temporary directory")
}

pub(crate) fn create_random_file(dir: &Path) -> PathBuf {
    let file_name = format!("test_file_{}", Uuid::new_v4());
    let file_path = dir.join(file_name);
    fs::write(&file_path, "test").expect("Failed to create test file");
    file_path
}
