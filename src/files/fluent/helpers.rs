use crate::{error::{mk_error_with_msg, error_to_string}, files::positions::FilePositions};
use sha3::{Digest, Sha3_256};
use fluent_syntax::parser::ParserError;
use std::{path::Path, io::Error, collections::{HashMap, hash_map::Entry}, fmt::Write};

pub(super) const MSG_SEP: &str = "-----------------------------------------------------------------------------";

pub(super) fn filter_comment(s: &&str) -> Option<String> {
    if *s == MSG_SEP { None } else { Some(s.to_string()) }
}

pub(super) fn add_header(headers: &mut HashMap<String, String>, key: &str, value: &str) {
    match headers.entry(key.to_lowercase()) {
        Entry::Vacant(entry) => { entry.insert(value.to_string()); }
        Entry::Occupied(mut entry) => {
            entry.get_mut().push_str(" ");
            entry.get_mut().push_str(value);
        }
    }
}

pub(super) fn make_error_from_error_list(prefix: &str, path: &Path, errs: Vec<ParserError>) -> Error {
    let positions = match FilePositions::read(path) {
        Err(e) => { return e; }
        Ok(r) => r
    };

    let mut msg = String::new();

    writeln!(msg, "{} `{}`:", prefix, path.display()).expect("Unexpected error while writing in string");
    for err in errs {
        let display = error_to_string(&err);
        let beg = positions.get_position_from_offset(err.pos.start);
        let end = positions.get_position_from_offset(err.pos.end);

        writeln!(msg, "  - at {} .. {}, {}", beg, end, display).expect("Unexpected error while writing in string");
    }

    mk_error_with_msg(msg)
}

pub(super) fn message_hash(text: &str) -> String {
    let mut hasher = Sha3_256::new();

    hasher.update(text);
    format!("{:x}", hasher.finalize())
}
