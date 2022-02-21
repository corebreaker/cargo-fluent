use poreader::{Origin, note::Note};

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct PoNote {
    note: Note
}

impl PoNote {
    pub(super) fn new(note: Note) -> Self {
        PoNote { note }
    }

    pub(crate) fn origin_developper(&self) -> bool {
        match self.note.origin() {
            Origin::Developer => true,
            _ => false,
        }
    }

    pub(crate) fn origin_translator(&self) -> bool {
        match self.note.origin() {
            Origin::Translator => true,
            _ => false,
        }
    }

    pub(crate) fn value(&self) -> &str {
        self.note.value()
    }
}
