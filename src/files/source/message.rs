use super::Origin;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Message {
    pub(crate) origin: Origin,
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) msg_id: String,
    pub(crate) parameters: Vec<String>,
    pub(crate) attributes: HashMap<String, String>,
}

impl Message {
    pub(super) fn new(origin: Origin, msg_id: String, line: usize, column: usize) -> Self {
        Message {
            origin,
            line,
            column,
            msg_id,
            parameters: vec![],
            attributes: HashMap::new(),
        }
    }

    pub(super) fn add_paramter(&mut self, name: String) {
        self.parameters.push(name);
    }

    pub(super) fn set_attribute(&mut self, name: String, value: String) {
        self.attributes.insert(name, value);
    }
}
