use std::path::PathBuf;
use std::fs;

pub struct FileInfo {
    pub file_name: String,
    pub file_size: u64,
}

impl FileInfo {
    pub fn new(p: &PathBuf) -> FileInfo {
        return FileInfo {
            file_name: String::from(p.file_name().unwrap().to_string_lossy()),
            file_size: p.symlink_metadata().unwrap().len()
        };
    }
}

pub fn list_dir_files(input_path: PathBuf) -> Vec<PathBuf> {
    let mut result_files: Vec<PathBuf> = Vec::new();

    if input_path.is_file() {
        result_files.push(input_path)
    } else if input_path.is_dir() {
        if let Ok(dir_files) = fs::read_dir(input_path) {
            for file in dir_files.filter_map(|file| file.ok()) {
                let file_path = file.path();
                if file_path.is_file() {
                    result_files.push(file_path)
                } else if file_path.is_dir() {
                    result_files.append(&mut list_dir_files(file_path))
                }
            }
        }
    }
    return result_files;
}
