mod entry;
mod reader;
mod writer;

use super::{FluentGroup, FluentMessage, FluentInformations};
use std::{io::{Write, Result}, collections::HashMap, path::Path, borrow::Borrow, hash::Hash};

#[derive(Debug)]
pub struct FluentFile {
    lang: String,
    messages: HashMap<String, FluentMessage>,
    groups: Vec<FluentGroup>,
    infos: FluentInformations,
    junk: Vec<String>,
    entries: Vec<entry::EntryType>,
}

impl FluentFile {
    pub(crate) fn new(lang: String) -> Self {
        FluentFile {
            lang,
            messages: HashMap::new(),
            groups: vec![],
            infos: FluentInformations::new(),
            junk: vec![],
            entries: vec![entry::EntryType::ResourceHeader],
        }
    }

    pub(crate) fn read(lang: String, path: &Path) -> Result<Self> {
        reader::read(lang, path)
    }

    pub(crate) fn write<W: Write>(&self, w: W) -> Result<()> {
        writer::write(self, w)
    }

    pub(crate) fn lang(&self) -> &str {
        &self.lang
    }

    pub(crate) fn groups(&self) -> &Vec<FluentGroup> {
        &self.groups
    }

    pub(crate) fn infos(&self) -> &FluentInformations {
        &self.infos
    }

    pub(crate) fn informations_mut(&mut self) -> &mut FluentInformations {
        &mut self.infos
    }

    pub(crate) fn add_group(&mut self, name: Option<String>) -> &mut FluentGroup {
        let idx = self.groups.len();

        self.entries.push(entry::EntryType::Group);
        self.groups.push(FluentGroup::new(name, vec![], FluentInformations::new()));

        &mut self.groups[idx]
    }

    pub(crate) fn messages(&self) -> &HashMap<String, FluentMessage> {
        &self.messages
    }

    pub(crate) fn get_message_mut<Q>(&mut self, id: &Q) -> Option<&mut FluentMessage>
        where String: Borrow<Q>, Q: Hash + Eq + ?Sized {
        self.messages.get_mut(id)
    }

    pub(crate) fn fetch_message_from_id(&mut self, id: impl AsRef<str>) -> &mut FluentMessage {
        self.messages.entry(id.as_ref().to_string()).or_insert_with(|| FluentMessage::empty(id.as_ref().to_string()))
    }

    pub(crate) fn fetch_message_from_text(&mut self, text: impl AsRef<str>) -> &mut FluentMessage {
        self.messages.entry(text.as_ref().to_string()).or_insert_with(|| FluentMessage::empty(text.as_ref().to_string()))
    }

    fn write_header<W: Write>(&self, w: &mut W) -> Result<()> {
        let header = format!("Language: {}", self.lang);

        self.infos.write(w, Some(&header), "###")
    }
}
