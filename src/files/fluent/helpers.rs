use crate::error::mk_error_with_msg;
use sha3::{Digest, Sha3_256};
use std::{path::Path, io::Error, collections::{HashMap, hash_map::Entry}};

#[inline]
pub(super) fn add_header(headers: &mut HashMap<String, String>, key: &str, value: &str) {
    match headers.entry(key.to_lowercase()) {
        Entry::Vacant(entry) => { entry.insert(value.to_string()); }
        Entry::Occupied(mut entry) => { entry.get_mut().push_str(value); }
    }
}

#[inline]
pub(super) fn make_error_from_error_list<E: std::error::Error>(prefix: &str, path: &Path, errs: Vec<E>) -> Error {
    let mut msg = format!("{} {:?}:", prefix, path);

    for err in errs {
        msg.push_str(&format!("  - {}", err));
    }

    mk_error_with_msg(msg)
}

pub(super) fn message_hash(text: &str) -> String {
    let mut hasher = Sha3_256::new();

    hasher.update(text);
    format!("{:x}", hasher.finalize())
}
