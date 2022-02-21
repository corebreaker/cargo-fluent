use super::FluentInformations as Infos;
use std::collections::HashMap;

pub struct FluentMessage {
    id: String,
    value: Option<String>,
    attributes: HashMap<String, String>,
    infos: Infos,
}

impl FluentMessage {
    pub(super) fn new(id: String, value: Option<String>, attributes: HashMap<String, String>, infos: Infos) -> Self {
        Self { id, value, attributes, infos }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    pub fn informations(&self) -> &Infos {
        &self.infos
    }

    pub(crate) fn set_value(&mut self, value: Option<String>) {
        self.value = value
    }

    pub(crate) fn attributes_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.attributes
    }

    pub(crate) fn informations_mut(&mut self) -> &mut Infos {
        &mut self.infos
    }
}
