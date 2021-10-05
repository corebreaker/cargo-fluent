use serde;
use toml;

use std::{io::{Result, Error, ErrorKind, Read}, fs::File, path::Path};

pub(super) fn read_toml<C>(path: &Path) -> Result<C> {
    let mut input = String::new();

    File::open(path).and_then(|mut f| f.read_to_string(&mut input))?;
    toml::from_str(&input).map_err(|err| Error::new(ErrorKind::Other, err))
}
