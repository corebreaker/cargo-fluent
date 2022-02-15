use toml;
use serde::Deserialize;
use std::{io::{Result, Error, ErrorKind, Read}, fs::File, path::Path};

pub(super) fn read_toml<'de, C: Deserialize<'de>>(path: &'de Path) -> Result<C> {
    let mut input = String::new();

    File::open(path).and_then(|mut f| f.read_to_string(&mut input))?;
    toml::from_str(&input).map_err(|err| Error::new(ErrorKind::Other, err))
}
