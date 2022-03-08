pub mod i18n;
mod cargo;

use super::read_toml::TomlFile;
use std::{io::Result, collections::HashMap, path::PathBuf};

fn make_path_from_name(root: &PathBuf, name: &String) -> Option<(String, PathBuf)> {
    let path = root.join(PathBuf::from(name.clone()));

    if path.exists() {
        Some((name.clone(), path))
    } else {
        None
    }
}

#[inline]
fn get_members(root: &PathBuf) -> Result<HashMap<String, PathBuf>> {
    let conf: cargo::CargoFile = TomlFile::new(&root.join("Cargo.toml"))?.decode()?;
    let mut members = HashMap::new();

    conf.get_members(root, &mut members);

    Ok(members)
}

pub(super) fn get_crates(root: &PathBuf) -> Result<HashMap<String, PathBuf>> {
    let mut res = HashMap::new();

    for (name, path) in get_members(root)? {
        res.extend(get_crates(&path)?.into_iter());
        res.insert(name, path);
    }

    Ok(res)
}

pub(super) fn read_i18n(dir: &PathBuf) -> Result<Option<i18n::I18nFile>> {
    let path = dir.join("i18n.toml");

    if path.exists() {
        TomlFile::new(&path)?.decode().map(|c: i18n::I18nFile| if c.is_empty() { None } else { Some(c) })
    } else {
        Ok(None)
    }
}
