use super::{FluentMessage, FluentInformations, pattern_stringifier::pattern_as_str};
use crate::error::mk_error_with_msg;
use regex::Regex;
use fluent_syntax::ast::{Attribute, Comment, Identifier, Message, Pattern, Term};
use std::{path::Path, io::Error};

#[inline]
pub(super) fn make_error<E: std::error::Error>(prefix: &str, path: &Path, errs: Vec<E>) -> Error {
    let mut msg = format!("{} {:?}:", prefix, path);

    for err in errs {
        msg.push_str(&format!("  - {}", err));
    }

    mk_error_with_msg(msg)
}

pub(super) trait IMessage {
    fn get_id(&self) -> &Identifier<&str>;
    fn get_value(&self) -> Option<&Pattern<&str>>;
    fn get_attributes(&self) -> &Vec<Attribute<&str>>;
    fn get_comments(&self) -> &Option<Comment<&str>>;

    fn decode_normalized_message(&self, infos_re: &Regex) -> FluentMessage {
        let id = self.get_id().name.to_string();
        let attributes = self.get_attributes().iter()
            .map(|attr| (attr.id.name.to_string(), pattern_as_str(&attr.value)))
            .collect();

        let lines = self.get_comments()
            .as_ref()
            .map(|v| v.content.iter().copied().map(String::from).collect::<Vec<_>>())
            .unwrap_or_default();

        let value = self.get_value().map(pattern_as_str);
        let infos = FluentInformations::new(infos_re, lines);

        FluentMessage::new(id.clone(), value, attributes, infos)
    }
}

impl IMessage for Term<&str> {
    fn get_id(&self) -> &Identifier<&str> {
        &self.id
    }

    fn get_value(&self) -> Option<&Pattern<&str>> {
        Some(&self.value)
    }

    fn get_attributes(&self) -> &Vec<Attribute<&str>> {
        &self.attributes
    }

    fn get_comments(&self) -> &Option<Comment<&str>> {
        &self.comment
    }
}

impl IMessage for Message<&str> {
    fn get_id(&self) -> &Identifier<&str> {
        &self.id
    }

    fn get_value(&self) -> Option<&Pattern<&str>> {
        self.value.as_ref()
    }

    fn get_attributes(&self) -> &Vec<Attribute<&str>> {
        &self.attributes
    }

    fn get_comments(&self) -> &Option<Comment<&str>> {
        &self.comment
    }
}
