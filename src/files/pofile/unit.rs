use poreader::Message;
use super::{comment::PoComment, note::PoNote};
use poreader::unit::Unit;

#[derive(Debug)]
pub(crate) struct PoUnit {
    context: Option<String>,
    message: String,
    values: Vec<String>,
    notes: Vec<PoNote>,
    locations: Vec<String>,
    comments: Vec<PoComment>,
    is_translated: bool,
    is_obsolete: bool,
}

impl PoUnit {
    pub(super) fn new(unit: Unit) -> Self {
        let mut values = vec![];
        let message = match unit.message() {
            Message::Simple { id, text } => {
                if let Some(v) = text {
                    values.push(v.clone());
                }

                id.clone()
            }
            Message::Plural(plural) => {
                values.extend(plural.values().iter().cloned());

                plural.singular().to_string()
            }
        };

        PoUnit {
            context: unit.context().map(String::from),
            message,
            values,
            notes: unit.notes().iter().cloned().map(PoNote::new).collect::<Vec<_>>(),
            locations: unit.locations().clone(),
            comments: unit.comments().iter().cloned().map(PoComment::new).collect::<Vec<_>>(),
            is_translated: unit.is_translated(),
            is_obsolete: unit.is_obsolete(),
        }
    }

    pub fn context(&self) -> Option<&str> {
        self.context.as_ref().map(String::as_str)
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn values(&self) -> &Vec<String> {
        &self.values
    }

    pub fn notes(&self) -> &Vec<PoNote> {
        &self.notes
    }

    pub fn locations(&self) -> &Vec<String> {
        &self.locations
    }

    pub fn comments(&self) -> &Vec<PoComment> {
        &self.comments
    }

    pub fn is_translated(&self) -> bool {
        self.is_translated
    }

    pub fn is_obsolete(&self) -> bool {
        self.is_obsolete
    }
}
