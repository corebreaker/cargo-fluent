mod files;
mod infos;
mod read_toml;
mod path_helper;

use std::{io::Result, path::{Path, PathBuf}, env::current_dir};

#[derive(Debug)]
pub struct Config {
    name: String,
    root: PathBuf,
    output: PathBuf,
    fallback_language: String,
}

impl Config {
    pub fn read(output: Option<String>) -> Result<Config> {
        let root = current_dir()?;
        let name = files::get_crate_name(&root)?;

        let output = output.map_or(Ok(None), |p| path_helper::make_dirpath(p).map(Some))?;
        let (output, fallback_language) = match infos::CrateInfos::import_config(&root)? {
            Some(infos) => (output.or(infos.fluent_assets), infos.fallback_language),
            None => (output, String::from("en-US")),
        };

        let output = output.unwrap_or_else(|| root.join("i18n"));

        Ok(Config { name, root, output, fallback_language })
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn root(&self) -> &Path {
        &self.root
    }
    pub(crate) fn output(&self) -> &Path {
        &self.output
    }
    pub(crate) fn fallback_language(&self) -> &str {
        &self.fallback_language
    }
}
