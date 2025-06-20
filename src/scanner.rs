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
        let pyvenv_cfg = path.join("pyvenv.cfg");
        if !pyvenv_cfg.exists() {
            return None;
        }

        let size = get_dir_size(&path).unwrap_or(0);

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

pub fn scan_for_venvs(root: &Path) -> Vec<VenvInfo> {
    let mut results = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_dir() {
            continue;
        }

        let name = entry.file_name().to_string_lossy().to_lowercase();
        if name == "venv" || name == ".venv" || name == "env" || name == "virtualenv" || name == "pyenv" || name ==".env" {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_get_dir_size() {
        let dir = tempdir().unwrap();
        let path = dir.path();

        // Test empty directory
        assert_eq!(get_dir_size(path).unwrap(), 0);

        // Create some files and directories
        let mut file1 = File::create(path.join("file1.txt")).unwrap();
        file1.write_all(b"12345").unwrap(); // 5 bytes

        let subdir1 = path.join("subdir1");
        fs::create_dir(&subdir1).unwrap();
        let mut file2 = File::create(subdir1.join("file2.txt")).unwrap();
        file2.write_all(b"1234567890").unwrap(); // 10 bytes

        let subdir2 = path.join("subdir2");
        fs::create_dir(&subdir2).unwrap();
        let mut file3 = File::create(subdir2.join("file3.txt")).unwrap();
        file3.write_all(b"123").unwrap(); // 3 bytes

        // Expected total size = 5 + 10 + 3 = 18 bytes
        assert_eq!(get_dir_size(path).unwrap(), 18);

        // Test with an empty subdirectory
        let empty_subdir = path.join("empty_subdir");
        fs::create_dir(&empty_subdir).unwrap();
        assert_eq!(get_dir_size(path).unwrap(), 18); // Size should not change

        // Test after adding a file to the initially empty subdirectory
        let mut file4 = File::create(empty_subdir.join("file4.txt")).unwrap();
        file4.write_all(b"1234567").unwrap(); // 7 bytes
        // Expected total size = 18 + 7 = 25 bytes
        assert_eq!(get_dir_size(path).unwrap(), 25);
    }

    // Note: Testing VenvInfo::new and scan_for_venvs requires more complex setup
    // to mock pyvenv.cfg files and potentially directory structures.
    // These tests would be more like integration tests for the scanner module.
    // For now, focusing on get_dir_size as it's a self-contained utility.

    #[test]
    fn test_venv_info_new_no_pyvenv_cfg() {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();
        assert!(VenvInfo::new(path).is_none());
    }

    #[test]
    fn test_venv_info_new_with_pyvenv_cfg() {
        let dir = tempdir().unwrap();
        let path = dir.path();
        File::create(path.join("pyvenv.cfg")).unwrap();

        let venv_info = VenvInfo::new(path.to_path_buf());
        assert!(venv_info.is_some());
        let info = venv_info.unwrap();
        assert_eq!(info.path, path);
        assert_eq!(info.size, 0); // pyvenv.cfg is 0 bytes, no other files
        assert!(info.last_modified.is_some());
    }
    
    #[test]
    fn test_scan_for_venvs() {
        let base_dir = tempdir().unwrap();
        let base_path = base_dir.path();

        // Create some directories, some of which are venvs
        let venv1_path = base_path.join("my_venv"); // common name
        fs::create_dir(&venv1_path).unwrap();
        File::create(venv1_path.join("pyvenv.cfg")).unwrap();
        let mut f1 = File::create(venv1_path.join("some_file.txt")).unwrap();
        f1.write_all(b"hello").unwrap(); // 5 bytes

        let venv2_path = base_path.join(".env"); // hidden common name
        fs::create_dir(&venv2_path).unwrap();
        File::create(venv2_path.join("pyvenv.cfg")).unwrap();
         let mut f2 = File::create(venv2_path.join("another.txt")).unwrap();
        f2.write_all(b"world12345").unwrap(); // 10 bytes


        let not_venv_path = base_path.join("not_a_venv");
        fs::create_dir(&not_venv_path).unwrap();
        File::create(not_venv_path.join("some_other_file.txt")).unwrap();
        
        let nested_venv_path = base_path.join("project/env"); // nested
        fs::create_dir_all(&nested_venv_path).unwrap();
        File::create(nested_venv_path.join("pyvenv.cfg")).unwrap();

        let results = scan_for_venvs(base_path);
        assert_eq!(results.len(), 3);

        // Check if found paths are correct (order might vary)
        let found_paths: Vec<_> = results.iter().map(|v| v.path.clone()).collect();
        assert!(found_paths.contains(&venv1_path));
        assert!(found_paths.contains(&venv2_path));
        assert!(found_paths.contains(&nested_venv_path));

        // Check sizes
        for venv in results {
            if venv.path == venv1_path {
                assert_eq!(venv.size, 5); // "hello"
            } else if venv.path == venv2_path {
                 assert_eq!(venv.size, 10); // "world12345" + 0 for pyvenv.cfg
            } else if venv.path == nested_venv_path {
                assert_eq!(venv.size, 0); // only pyvenv.cfg
            }
        }
    }
}
