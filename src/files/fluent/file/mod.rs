mod entry;
mod reader;
mod writer;

use super::{FluentGroup, FluentMessage, FluentInformations, helpers::message_hash};
use std::{io::{Write, Result}, collections::HashMap, path::Path, borrow::Borrow, hash::Hash};
use crate::files::fluent::file::entry::EntryType;

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

    pub(crate) fn fetch_message_from_text(&mut self, ctxt: Option<&str>, text: impl AsRef<str>) -> &mut FluentMessage {
        let hash = message_hash(text.as_ref());
        let id = match self.find_message_id(ctxt, &hash) {
            Some(v) => v,
            None => match ctxt {
                Some(ctxt) => format!("{}-msg-{:04}", ctxt, self.messages.len() + 1),
                None => format!("msg-{:04}", self.messages.len() + 1),
            }
        };

        self.messages.entry(id.clone()).or_insert_with(|| {
            let mut res = FluentMessage::empty(id);
            let infos = res.informations_mut();

            infos.set_header("message-id", hash);
            if let Some(ctxt) = ctxt {
                infos.set_header("context", ctxt.to_string());
            }

            self.entries.push(EntryType::Message(res.id().clone()));

            res
        })
    }

    fn find_message_id(&self, ctxt: Option<&str>, hash: &str) -> Option<String> {
        for (_, msg) in &self.messages {
            if msg.equals(ctxt, &hash) {
                return Some(msg.id().clone());
            }
        }

        None
    }

    fn write_header<W: Write>(&self, w: &mut W) -> Result<()> {
        let header = format!("Language: {}", self.lang);

        self.infos.write(w, Some(&header), "###")
    }
}
