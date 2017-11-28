use std::path::PathBuf;
use std::ffi::OsStr;

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