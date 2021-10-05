use super::files::{i18n::I18nFile, read_i18n};
use std::{path::{Path, PathBuf}, collections::HashMap, io::Result};

pub struct CrateConfig {
    name: String,
    path: PathBuf,
    fallback_language: Option<String>,
    fluent_files: HashMap<String, PathBuf>,
    gettext_files: HashMap<String, PathBuf>,
}

impl CrateConfig {
    fn new(name: &str, path: PathBuf, file: I18nFile) -> CrateConfig {
        CrateConfig {
            name: name.to_string(),
            path,
            fluent_files: HashMap::new(),
            gettext_files: HashMap::new(),
            fallback_language: file.fallback_language,
        }
    }

    pub(super) fn import_config(name: String, path: PathBuf, confs: &mut HashMap<String, CrateConfig>) -> Result<()> {
        if let Some(file) = read_i18n(&path)? {
            let conf = Self::new(&name, path, file);

            confs.insert(name, conf);
        }

        Ok(())
    }
}
