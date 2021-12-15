use std::path::PathBuf;
use std::{fs, io::Result};

/// Read text content of file into string.
/// relative paths are resolved from cargo's root dir
pub fn read_input(filename: &str) -> Result<String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(filename);
    fs::read_to_string(path)
}
