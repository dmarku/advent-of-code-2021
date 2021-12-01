use std::{io::Result, fs};
    use std::path::PathBuf;

pub fn read_input(filename: &str) -> Result<String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(filename);
    fs::read_to_string(path)
}
