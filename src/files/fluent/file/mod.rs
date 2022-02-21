mod entry;
mod reader;
mod writer;

use super::{FluentGroup, FluentMessage, FluentInformations};
use unic_langid::LanguageIdentifier;
use std::{io::{Write, Result}, collections::HashMap, path::Path};

pub struct FluentFile {
    lang: LanguageIdentifier,
    messages: HashMap<String, FluentMessage>,
    groups: Vec<FluentGroup>,
    infos: FluentInformations,
    junk: Vec<String>,
    entries: Vec<entry::EntryType>,
}

impl FluentFile {
    pub(crate) fn read(lang: LanguageIdentifier, path: &Path) -> Result<FluentFile> {
        reader::read(lang, path)
    }

    pub(crate) fn write<W: Write>(&self, w: W) -> Result<()> {
        writer::write(self, w)
    }

    fn write_header<W: Write>(&self, w: &mut W) -> Result<()> {
        let header = format!("Language: {}", self.lang);

        self.infos.write(w, Some(&header), "###")
    }
}
