use poreader::{CatalogueReader, PoParser, Origin, note::Note, comment::Comment, unit::Unit};
use std::{path::Path, io::Result, fs::File, collections::HashMap};

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct PoComment {
    comment: Comment
}

impl PoComment {
    pub(crate) fn kind(&self) -> char {
        self.comment.kind()
    }

    pub(crate) fn comment(&self) -> &String {
        self.comment.comment()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) struct PoNote {
    note: Note
}

impl PoNote {
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

#[derive(Debug)]
pub(crate) struct PoUnit {
    unit: Unit,
}

#[derive(Debug)]
pub(crate) struct PoFile {
    target_language: String,
    headers: HashMap<String, String>,
    comments: Vec<PoComment>,
    notes: Vec<PoNote>,
    units: Vec<PoUnit>,
}

impl PoFile {
    pub(crate) fn new(path: &Path) -> Result<PoFile> {
        let parser = PoParser::new();
        let reader = parser.parse( File::open(path)?)?;

        let target_language = reader.target_language().to_string();
        let headers = reader.header_properties().clone();
        let comments = reader.header_comments().iter().map(|c| PoComment { comment: c.clone() }).collect();
        let notes = reader.header_notes().iter().map(|n| PoNote { note: n.clone() }).collect();

        let mut units = vec![];

        for u in reader {
            units.push(PoUnit { unit: u? });
        }

        Ok(PoFile { target_language, headers, comments, notes, units })
    }

    pub(crate) fn target_language(&self) -> &str {
        &self.target_language
    }

    pub(crate) fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub(crate) fn comments(&self) -> &Vec<PoComment> {
        &self.comments
    }

    pub(crate) fn notes(&self) -> &Vec<PoNote> {
        &self.notes
    }

    pub(crate) fn units(&self) -> &Vec<PoUnit> {
        &self.units
    }
}
