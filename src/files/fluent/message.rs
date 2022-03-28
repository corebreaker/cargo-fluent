use super::FluentInformations as Infos;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Debug)]
pub struct FluentMessage {
    id: String,
    value: Option<String>,
    attributes: HashMap<String, String>,
    locations: HashSet<String>,
    infos: Infos,
}

impl FluentMessage {
    pub(super) fn empty(id: String) -> Self {
        Self {
            id,
            value: None,
            attributes: HashMap::new(),
            locations: HashSet::new(),
            infos: Infos::new(),
        }
    }

    pub(super) fn new(id: String, value: Option<String>, attributes: HashMap<String, String>, infos: Infos) -> Self {
        let locations = infos.headers().get("locations")
            .map_or("", String::as_str)
            .split(" ")
            .map(String::from)
            .collect();

        Self { id, value, attributes, locations, infos }
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

    pub(crate) fn add_location(&mut self, location: String) {
        if self.locations.insert(location) {
            self.infos.set_header("locations", self.locations.iter().sorted().join(" "));
        }
    }

    pub(crate) fn attributes_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.attributes
    }

    pub(crate) fn informations_mut(&mut self) -> &mut Infos {
        &mut self.infos
    }
}
