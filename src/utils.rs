use std::fs;
use std::path::Path;

/// Deletes a directory recursively
pub fn delete_venv(path: &Path) -> Result<(), std::io::Error> {
    fs::remove_dir_all(path)
}
