use std::fs;

pub fn read_build_file(file: &String) -> Option<String> {
    fs::read_to_string(file).ok()
}
