
use std::fs;

pub fn clean_file(name: &str) {
    let _ = fs::remove_file(name.to_string());
}