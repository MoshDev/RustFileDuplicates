#[allow(non_snake_case)]
use std::env;
use std::fs;
use std::vec::Vec;
use std::path::PathBuf;
use std::path::Path;
use std::os::macos::fs::MetadataExt;

mod files;

fn main() {
    match find_arg_input_path() {
        Some(input_path) => process_dir(input_path),
        None => panic!("Target Dir is missing")
    }
}

pub fn find_arg_input_path() -> Option<String> {
    return env::args().collect::<Vec<_>>().get(1).cloned();
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

fn process_dir(input_path: String) -> () {
    println!("{:?}", input_path);

    let dir = Path::new(&input_path);
    let dir_files = list_dir_files(dir.to_path_buf());
    let mapped: Vec<files::FileInfo> = dir_files.iter().map(|p| files::FileInfo::new(p)).collect();
    for _file in mapped {
        println!("{filename:?} : {filesize:?}", filename = _file.file_name, filesize = _file.file_size);
    }
    //    for _file in dir_files {
    //                println!("{filename:?} | {filesize:?}", filename = _file.file_name().unwrap(), filesize = _file.metadata().unwrap().st_size());
    //    }
}