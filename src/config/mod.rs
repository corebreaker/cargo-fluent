mod files;
mod infos;
mod read_toml;
mod path_helper;

use crate::{error::{mk_error, mk_error_with_msg}, path_utils::path_to_string};
use std::{io::{Result, ErrorKind}, path::{Path, PathBuf}, env::current_dir};

#[derive(Debug)]
pub struct Config {
    name: String,
    root: PathBuf,
    output: PathBuf,
    po_dir: Option<PathBuf>,
    fallback_language: String,
}

impl Config {
    pub fn read(output: Option<String>) -> Result<Config> {
        let root = current_dir()?;
        let name = files::get_crate_name(&root)?;

        let output = output.map_or(Ok(None), |p| path_helper::make_dirpath(p).map(Some))?;
        let (output, po_dir, fallback_language) = match infos::CrateInfos::import_config(&root)? {
            Some(infos) => (output.clone().or(infos.fluent_assets), infos.po_dir, infos.fallback_language),
            None => (output.clone(), None, String::from("en-US")),
        };

        let output = output.unwrap_or_else(|| root.join("i18n"));

        if let Some(po_dir) = &po_dir {
            if !po_dir.exists() {
                let msg = format!("The path `{}` does not exist", path_to_string(po_dir));

                return Err(mk_error(ErrorKind::NotFound, msg));
            }

            if !po_dir.is_dir() {
                return Err(mk_error_with_msg(format!("The path `{}` is not a directory", path_to_string(po_dir))));
            }
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

    pub(crate) fn po_dir(&self) -> Option<&Path> {
        self.po_dir.as_ref().map(PathBuf::as_path)
    }

    pub(crate) fn fallback_language(&self) -> &str {
        &self.fallback_language
    }
}
