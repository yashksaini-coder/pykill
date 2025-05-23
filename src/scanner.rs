use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct VenvInfo {
    pub path: PathBuf,
    pub size: u64,
    pub last_modified: Option<DateTime<Local>>,
}

impl VenvInfo {
    pub fn new(path: PathBuf) -> Option<Self> {
        // Check if this is really a virtualenv
        let pyvenv_cfg = path.join("pyvenv.cfg");
        if !pyvenv_cfg.exists() {
            return None;
        }

        // Calculate size
        let size = get_dir_size(&path).unwrap_or(0);

        // Get last modified time
        let metadata = fs::metadata(&path).ok()?;
        let modified = metadata.modified().ok()
            .and_then(|t| DateTime::<Local>::from(t).into());

        Some(Self {
            path,
            size,
            last_modified: modified,
        })
    }
}

/// Scan a directory recursively for potential Python virtualenv folders
pub fn scan_for_venvs(root: &Path) -> Vec<VenvInfo> {
    let mut results = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_lowercase();
        if name == "venv" || name == ".venv" || name == "env" {
            if let Some(venv) = VenvInfo::new(entry.path().to_path_buf()) {
                results.push(venv);
            }
        }
    }

    results
}

/// Recursively calculate directory size in bytes
fn get_dir_size(path: &Path) -> Result<u64, std::io::Error> {
    let mut size = 0;

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            size += entry.metadata()?.len();
        }
    }

    Ok(size)
}
