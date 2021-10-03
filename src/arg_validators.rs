use std::fs;

pub fn file_exists(path: String) -> Result<(), String> {
    if fs::metadata(&path).is_ok() {
        Ok(())
    } else {
        Err(format!("{}: {}", "This file doesn't exist", path))
    }
}
