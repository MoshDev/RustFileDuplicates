use std::env;
use std::vec::Vec;
use std::path::PathBuf;

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


fn process_dir(input_path: String) -> () {
    println!("{:?}", input_path);

    let dir = PathBuf::from(&input_path);
    let dir_files = files::list_dir_files(dir);
    let mapped: Vec<files::FileInfo> = dir_files.iter().map(|p| files::FileInfo::new(p)).collect();
    println!("Size of list: {}",mapped.len());
    for _file in mapped {
        //        println!("{filename:?} : {filesize:?}", filename = _file.file_name, filesize = _file.file_size);
    }
}