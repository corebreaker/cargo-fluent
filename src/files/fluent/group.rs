use super::FluentInformations;
use itertools::Itertools;
use std::io::{Write, Result};

pub struct FluentGroup {
    name: Option<String>,
    message_ids: Vec<String>,
    infos: FluentInformations,
}

impl FluentGroup {
    pub(super) fn new(name: Option<String>, message_ids: Vec<String>, infos: FluentInformations) -> Self {
        FluentGroup { name, message_ids, infos }
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn message_ids(&self) -> &Vec<String> {
        &self.message_ids
    }

    pub fn has_message_id(&self, id: &String) -> bool {
        self.message_ids.contains(id)
    }

    pub fn informations(&self) -> &FluentInformations {
        &self.infos
    }

    pub(crate) fn add_message_id(&mut self, value: String) {
        if !self.message_ids.contains(&value) {
            self.message_ids.push(value);
        }
    }

    pub(crate) fn remove_message_id(&mut self, value: &String) {
        if let Some((idx, _)) = self.message_ids.iter().find_position(|&m| m == value) {
            self.message_ids.remove(idx);
        }
    }

    pub(crate) fn informations_mut(&mut self) -> &mut FluentInformations {
        &mut self.infos
    }

    pub(super) fn write<W: Write>(&self, w: &mut W) -> Result<()> {
        self.infos.write(w, self.name.as_ref(), "##")
    }
}
