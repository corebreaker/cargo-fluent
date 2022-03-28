use crate::files::fluent::FluentFile;
use std::{path::{Path, PathBuf}, fs::File, io::Result};

pub(super) struct FluentFileEntry {
    language: String,
    domain: String,
    file: FluentFile,
}

impl FluentFileEntry {
    pub(super) fn fetch(output_dir: &Path, language: String, domain: String) -> Result<Self> {
        let file = FluentFile::new(language.clone());
        let mut res = FluentFileEntry { language, domain, file };

        let path = res.mk_filepath(output_dir);

        if path.exists() {
            res.file = FluentFile::read(res.language.clone(), &path)?;
        }

        Ok(res)
    }

    pub(super) fn file_mut(&mut self) -> &mut FluentFile {
        &mut self.file
    }

    pub(super) fn write(&self, dir: &Path) -> Result<()> {
        self.file.write(File::create(self.mk_filepath(dir))?)
    }

    fn mk_filepath(&self, dir: &Path) -> PathBuf {
        dir.join(self.domain.clone()).join(format!("{}.flt", self.language))
    }
}
