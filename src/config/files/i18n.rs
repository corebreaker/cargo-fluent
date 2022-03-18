use super::super::path_helper::path_join;
use i18n_config::{FluentConfig, GettextConfig, I18nConfig};
use std::{io::{Result, Error, ErrorKind}, path::{Path, PathBuf}};

#[derive(Debug)]
pub(in super::super) struct I18nGettext {
    pub(in super::super) output_dir: PathBuf,
    pub(in super::super) po_dir: PathBuf,
}

impl I18nGettext {
    fn from_conf(dir: &Path, conf: &GettextConfig) -> Result<Self> {
        Ok(I18nGettext {
            output_dir: path_join(dir, &conf.output_dir)?,
            po_dir: path_join(dir, &conf.po_dir())?,
        })
    }

    fn from_file(dir: &Path, file: &I18nConfig) -> Result<Option<Self>> {
        file.gettext.as_ref().map_or(Ok(None), |conf| I18nGettext::from_conf(dir, conf).map(Some))
    }
}

#[derive(Debug)]
pub(in super::super) struct I18nFluent {
    pub(in super::super) assets_dir: PathBuf,
}

impl I18nFluent {
    fn from_conf(dir: &Path, conf: &FluentConfig) -> Result<Self> {
        Ok(I18nFluent {
            assets_dir: path_join(dir, &conf.assets_dir)?,
        })
    }

    fn from_file(dir: &Path, file: &I18nConfig) -> Result<Option<Self>> {
        file.fluent.as_ref().map_or(Ok(None), |conf| I18nFluent::from_conf(dir, conf).map(Some))
    }
}

#[derive(Debug)]
pub(in super::super) struct I18nFile {
    pub(in super::super) fallback_language: String,
    pub(in super::super) fluent: Option<I18nFluent>,
    pub(in super::super) gettext: Option<I18nGettext>,
}

impl I18nFile {
    pub(in super::super) fn read(dir: &Path) -> Result<Option<Self>> {
        let toml_path = dir.join("i18n.toml");

        Ok(if toml_path.exists() {
            let file = I18nConfig::from_file(&toml_path).map_err(|err| Error::new(ErrorKind::Other, err))?;
            let res = I18nFile {
                fallback_language: file.fallback_language.to_string(),
                fluent: I18nFluent::from_file(dir, &file)?,
                gettext: I18nGettext::from_file(dir, &file)?,
            };

            if res.is_empty() {
                None
            } else {
                Some(res)
            }
        } else {
            None
        })
    }

    fn is_empty(&self) -> bool {
        self.fallback_language.is_empty() || (self.fluent.is_none() && self.gettext.is_none())
    }
}
