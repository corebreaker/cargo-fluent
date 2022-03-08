use toml;
use serde::Deserialize;
use std::{io::{Result, Error, ErrorKind, Read}, fs::File, path::Path};

pub(super) struct TomlFile {
    source: String
}

impl TomlFile {
    pub(super) fn new(path: &Path) -> Result<Self> {
        let mut source = String::new();

        File::open(path).and_then(|mut f| f.read_to_string(&mut source))?;
        Ok(Self { source })
    }

    pub(super) fn decode<'de, 'a: 'de, C: Deserialize<'de>>(&'a self) -> Result<C> {
        toml::from_str(&self.source).map_err(|err| Error::new(ErrorKind::Other, err))
    }
}
