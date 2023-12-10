use std::fs;

pub fn read_file(loc: &str) -> String {
    let f = fs::read_to_string(loc)
        .expect("Failed to open file!");
    return f
}