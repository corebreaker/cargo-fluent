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

        let mut res = Self { id, value, attributes, locations, infos };

        res.update_location_header();
        res
    }

    fn update_location_header(&mut self) {
        let locations = self.locations.iter()
            .map(|loc| {
                let mut parts = loc.split(":");
                let file = parts.next().unwrap_or_default();
                let line: usize = parts.next().unwrap_or("0").parse().unwrap_or(0usize);

                (file, line, loc)
            })
            .sorted()
            .map(|(_, _, loc)| loc)
            .join(" ");

        self.infos.set_header("locations", locations);
    }

    pub(super) fn equals(&self, ctxt: Option<&str>, hash: &str) -> bool {
        let headers = self.infos.headers();

        if let Some(msgid) = headers.get("message-id") {
            if msgid != hash {
                return false;
            }
        }

        if let Some(ctxt) = ctxt {
            if let Some(msg_ctx) = headers.get("context") {
                ctxt == msg_ctx
            } else {
                let mut prefix = ctxt.to_string();

                prefix.push_str("-");
                self.id.starts_with(&prefix)
            }
        } else {
            true
        }
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
            self.update_location_header();
        }
    }

    pub(crate) fn attributes_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.attributes
    }

    pub(crate) fn informations_mut(&mut self) -> &mut Infos {
        &mut self.infos
    }
}
