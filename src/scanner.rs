use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct VenvInfo {
    pub path: PathBuf,
    pub size: u64,
    pub last_modified: Option<DateTime<Local>>,
}

impl VenvInfo {
    pub fn new(path: PathBuf) -> Option<Self> {
        let pyvenv_cfg = path.join("pyvenv.cfg");
        if !pyvenv_cfg.exists() {
            return None;
        }

        let size = get_dir_size(&path).unwrap_or(0);

        let metadata = fs::metadata(&path).ok()?;
        let modified = metadata.modified().ok()
            .map(|t| DateTime::<Local>::from(t));

        Some(Self {
            path,
            size,
            last_modified: modified,
        })
    }
}

pub fn scan_for_venvs(root: &Path) -> Vec<VenvInfo> {
    let mut results = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_dir() {
            continue;
        }

        // Check if this directory contains pyvenv.cfg, which makes it a virtual environment
        let pyvenv_cfg = entry.path().join("pyvenv.cfg");
        if pyvenv_cfg.exists() {
            if let Some(venv) = VenvInfo::new(entry.path().to_path_buf()) {
                results.push(venv);
            }
        }
    }

    results
}

fn get_dir_size(path: &Path) -> Result<u64, std::io::Error> {
    let mut size = 0;

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            size += entry.metadata()?.len();
        }
    }

    Ok(size)
}


