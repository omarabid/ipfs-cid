mod error;

use crate::error::Result;

pub fn generate_from_file(file: std::fs::File) -> Result<String> {
    Ok(String::from("tbd"))
}

pub fn generate_from_bytes(bytes: &[u8]) -> Result<String> {
    Ok(String::from("tbd"))
}
