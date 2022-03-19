mod files;
mod infos;
mod read_toml;
mod path_helper;

use crate::error::{mk_error, mk_error_with_msg};
use std::{io::{Result, ErrorKind}, path::{Path, PathBuf}, env::current_dir};

#[derive(Debug)]
pub struct Config {
    name: String,
    root: PathBuf,
    output: PathBuf,
    po_dir: PathBuf,
    fallback_language: String,
}

impl Config {
    pub fn read(output: Option<String>) -> Result<Config> {
        let root = current_dir()?;
        let name = files::get_crate_name(&root)?;

        let output = output.map_or(Ok(None), |p| path_helper::make_dirpath(p).map(Some))?;
        let (output, po_dir, fallback_language) = match infos::CrateInfos::import_config(&root)? {
            Some(infos) => (output.clone().or(infos.fluent_assets), infos.po_dir.or(output), infos.fallback_language),
            None => (output.clone(), output, String::from("en-US")),
        };

        let output = output.unwrap_or_else(|| root.join("i18n"));
        let po_dir = po_dir.unwrap_or_else(|| root.join("i18n"));

        if !po_dir.exists() {
            let msg = format!("The path `{}` does not exist", po_dir.to_string_lossy());

            return Err(mk_error(ErrorKind::NotFound, msg));
        }

        if !po_dir.is_dir() {
            return Err(mk_error_with_msg(format!("The path `{}` is not a directory", po_dir.to_string_lossy())));
        }

        Ok(Config { name, root, output, po_dir, fallback_language })
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

    pub(crate) fn po_dir(&self) -> &Path {
        &self.po_dir
    }

    pub(crate) fn fallback_language(&self) -> &str {
        &self.fallback_language
    }
}
