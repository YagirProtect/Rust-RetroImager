use std::env;
use std::path::PathBuf;


pub fn get_app_dir() -> PathBuf{
    return env::current_dir().unwrap();
}

pub fn is_file_exist(fileName: &str) -> bool {
    return get_app_dir().join(fileName).is_file();
}
