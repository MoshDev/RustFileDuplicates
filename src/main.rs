#[allow(non_snake_case)]
use std::env;
use std::fs;
use std::vec::Vec;
use std::path::PathBuf;
use std::path::Path;
use std::os::macos::fs::MetadataExt;

fn main() {

    let dir_path = &find_arg_input_path();
    if dir_path.eq("Missing Argument") {
        panic!("Target Dir is missing");
    }

    let dir = Path::new(dir_path);
    let files = list_dir_files(dir.to_path_buf());
    for _file in files {
        println!("{filename:?} | {filesize:?}", filename = _file.file_name().unwrap(), filesize = _file.metadata().unwrap().st_size());
    }
}

pub fn find_arg_input_path() -> String {
    let arg_input_path = env::args().nth(1).expect("Missing Argument");
    return arg_input_path;
}

fn list_dir_files(input_path: PathBuf) -> Vec<PathBuf> {
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