use super::info_helpers::add_comments_and_notes;
use crate::{files::{pofile::PoFile, fluent::FluentFile}, error::mk_error};
use std::{path::{Path, PathBuf}, io::{Result, ErrorKind}, fmt::Write};

pub(super) struct InputPoFile {
    pub(super) path: PathBuf,
    pub(super) language: String,
    pub(super) domain: String,
    content: PoFile,
}

impl InputPoFile {
    pub(super) fn read(path: &Path) -> Result<Self> {
        let parent = match path.parent() {
            Some(v) => v,
            None => {
                let msg = format!("The domain cannot be extracted from the path `{}` (unknown parent)", path.display());

                return Err(mk_error(ErrorKind::NotFound, msg));
            }
        };

        let language = match parent.file_name() {
            Some(v) => v.to_string_lossy().to_string(),
            None => {
                let msg = format!("The domain cannot be extracted from the path `{}` (no name for the parent)", path.display());

                return Err(mk_error(ErrorKind::NotFound, msg));
            }
        };

        let domain = match path.file_stem() {
            Some(v) => v.to_string_lossy().to_string(),
            None => {
                let msg = format!("The language cannot be extracted from the path `{}`", path.display());

                return Err(mk_error(ErrorKind::NotFound, msg));
            }
        };

        Ok(InputPoFile {
            path: path.to_path_buf(),
            language,
            domain,
            content: PoFile::read(path)?,
        })
    }

    pub(super) fn convert(&self, into: &mut FluentFile, include_fuzzy: bool) {
        let content = &self.content;

        into.informations_mut().extends_headers(content.headers().iter().map(|(k, v)| (k.clone(), v.clone())));
        add_comments_and_notes(content.comments(), content.notes(), into.informations_mut());

        for unit in content.units() {
            if unit.is_obsolete() || (!unit.is_translated() && !include_fuzzy) {
                continue;
            }

            let msg = into.fetch_message_from_text(unit.context(), unit.message());

            add_comments_and_notes(unit.comments(), unit.notes(), msg.informations_mut());

            for loc in unit.locations() {
                msg.add_location(loc.clone());
            }

            match unit.values().len() {
                0 => {}
                1 => {
                    msg.set_value(unit.values().iter().next().cloned());
                }
                sz => {
                    let mut text = String::from("{ $count ->");

                    for (i, v) in unit.values().iter().enumerate() {
                        let num = i + 1;

                        if num < sz {
                            write!(text, "\n     [{}] {}", num, v).unwrap();
                        } else {
                            write!(text, "\n    *[other] {}", v).unwrap();
                        }
                    }

                    text.push_str("\n}");

                    msg.set_value(Some(text));
                }
            }
        }
    }
}