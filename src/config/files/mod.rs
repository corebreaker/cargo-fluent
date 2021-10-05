pub mod i18n;
mod cargo;

use super::read_toml::read_toml;
use std::{io::Result, collections::HashMap, path::PathBuf};

fn make_path_from_name(root: &PathBuf, name: &String) -> Option<(String, PathBuf)> {
    let path = root.join(PathBuf::from(name.clone()));

    if path.exists() {
        Some((name.clone(), path))
    } else {
        None
    }
}

pub(super) fn get_crates(root: &PathBuf) -> Result<HashMap<String, PathBuf>> {
    let conf: cargo::CargoFile = read_toml(&root.join("Cargo.toml"))?;
    let mut members = HashMap::new();

    conf.get_members(root, &mut members);
    for p in members.values() {
        members.extend(get_crates(p)?.into_iter())
    }

    Ok(members)
}

pub(super) fn read_i18n(dir: &PathBuf) -> Result<Option<i18n::I18nFile>> {
    let path = dir.join("i18n.toml");

    if path.exists() {
        read_toml(&path).map(|c: i18n::I18nFile| if c.is_empty() { None } else { Some(c) } )
    } else {
        Ok(None)
    }
}
