use super::files::i18n::I18nFile;
use std::{path::{Path, PathBuf}, io::Result};

#[derive(Debug)]
pub struct CrateInfos {
    pub(super) po_dir: Option<PathBuf>,
    pub(super) fluent_assets: Option<PathBuf>,
    pub(super) fallback_language: String,
}

impl CrateInfos {
    fn new(file: I18nFile) -> Self {
        CrateInfos {
            po_dir: file.gettext.map(|gettext| gettext.po_dir),
            fluent_assets: file.fluent.map(|fluent| fluent.assets_dir),
            fallback_language: file.fallback_language,
        }
    }

    pub(super) fn import_config(dir: &Path) -> Result<Option<Self>> {
        Ok(I18nFile::read(dir)?.map(Self::new))
    }
}
