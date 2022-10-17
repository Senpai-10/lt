// fs wrapper

use std::fs;

// functions

// pub create_file(file) -> Error
// pub read_file(file) -> String
// pub write_file(file, contents) -> Error
// pub exists

pub fn create_file(file: &str, contents: &str) {
    return fs::write(file, contents).expect("Failed to create file");
}

pub fn read_file(file: &str) -> String {
    return fs::read_to_string(file).expect("Unable to read file");
}

pub fn file_exists(file: &str) -> bool {
    return fs::metadata(file).is_ok();
}
