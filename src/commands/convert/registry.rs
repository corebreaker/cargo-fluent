use super::entry::FluentFileEntry;
use crate::files::fluent::FluentFile;
use std::{collections::{hash_map::Entry, HashMap}, path::Path, io::Result};

pub(super) struct FluentFileRegistry(HashMap<String, HashMap<String, FluentFileEntry>>);

impl FluentFileRegistry {
    pub(super) fn new() -> Self {
        FluentFileRegistry(HashMap::new())
    }

    pub(super) fn fetch(&mut self, output_dir: &Path, language: String, domain: String) -> Result<&mut FluentFile> {
        let domains = self.0.entry(language.clone()).or_default();
        let entry = match domains.entry(domain.clone()) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(FluentFileEntry::fetch(output_dir, language, domain)?),
        };

        Ok(entry.file_mut())
    }

    pub(super) fn drain(self) -> impl Iterator<Item=FluentFileEntry> {
        self.0.into_iter().flat_map(|(_, domains)| domains.into_iter().map(|(_, v)| v))
    }
}
