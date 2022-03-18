mod cargo;
pub mod i18n;

use super::read_toml::TomlFile;
use simple_error::SimpleError;
use std::{path::Path, io::{Result, Error, ErrorKind}};

pub(super) fn get_crate_name(root: &Path) -> Result<String> {
    let conf: cargo::CargoFile = TomlFile::new(&root.join("Cargo.toml"))?.decode()?;

    Ok(match conf.name() {
        Some(name) => name,
        None => match root.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => { return Err(Error::new(ErrorKind::Other, SimpleError::new("This crate has no name"))); }
        }
    })
}
