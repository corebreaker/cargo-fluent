use super::{PoComment, PoNote, PoUnit};
use poreader::{CatalogueReader, PoParser};
use std::{path::Path, io::Result, fs::File, collections::HashMap};

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
        let comments = reader.header_comments().iter().map(|c| PoComment::new(c.clone())).collect();
        let notes = reader.header_notes().iter().map(|n| PoNote::new(n.clone())).collect();

        let mut units = vec![];

        for u in reader {
            units.push(PoUnit::new(u?));
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
