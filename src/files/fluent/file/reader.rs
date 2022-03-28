use super::{super::{helpers::{make_error, IMessage}, FluentInformations, FluentGroup}, entry::EntryType, FluentFile};
use regex::Regex;
use fluent::FluentResource;
use fluent_syntax::ast::Entry;
use std::{io::{Result, Error, ErrorKind}, collections::HashMap, path::Path, fs::read_to_string};

pub(super) fn read(lang: String, path: &Path) -> Result<FluentFile> {
    let infos_re = match Regex::new(r"^\s*@(\w+)[^:]*:\s*(\S.*)\s*$") {
        Ok(re) => re,
        Err(err) => { return Err(Error::new(ErrorKind::Other, err)); }
    };

    let resource = match FluentResource::try_new(read_to_string(path)?) {
        Ok(r) => r,
        Err((_, errs)) => {
            return Err(make_error("Parse error while reading fluent file", path, errs));
        }
    };

    let mut messages = HashMap::new();
    let mut groups = vec![];
    let mut info_lines = vec![];
    let mut junk = vec![];
    let mut entries = vec![];

    let mut current_group_name: Option<String> = None;
    let mut current_group_infos: Option<FluentInformations> = None;
    let mut current_group_message_ids = vec![];

    for entry in resource.entries() {
        match entry {
            Entry::Message(msg) => {
                let msg = msg.decode_normalized_message(&infos_re);
                let id = msg.id().clone();

                if current_group_infos.is_some() {
                    current_group_message_ids.push(id.clone());
                }

                entries.push(EntryType::Message(id.clone()));
                messages.insert(id, msg);
            }
            Entry::Term(msg) => {
                let msg = msg.decode_normalized_message(&infos_re);
                let id = msg.id().clone();

                entries.push(EntryType::Message(id.clone()));
                messages.insert(id, msg);
            }
            Entry::Comment(c) => {
                entries.push(EntryType::Comment(c.content.iter().map(|c| c.to_string()).collect::<Vec<_>>()));
            }
            Entry::GroupComment(c) => {
                if let Some(infos) = current_group_infos.take() {
                    groups.push(FluentGroup::new(current_group_name.take(), current_group_message_ids, infos));
                    current_group_message_ids = vec![];
                }

                let lines = c.content.iter().map(|c| c.to_string()).collect::<Vec<_>>();
                let mut infos = FluentInformations::from_lines(&infos_re, lines);

                entries.push(EntryType::Group);
                current_group_name = infos.remove_header("name");
                current_group_infos.replace(infos);
            }
            Entry::ResourceComment(c) => {
                if info_lines.is_empty() {
                    entries.push(EntryType::ResourceHeader);
                }

                info_lines.extend(c.content.iter().map(|l| l.to_string()));
            }
            Entry::Junk { content } => {
                entries.push(EntryType::Junk);
                junk.push(content.to_string());
            }
        }
    }

    if let Some(infos) = current_group_infos.take() {
        groups.push(FluentGroup::new(current_group_name.take(), current_group_message_ids, infos));
    }

    Ok(FluentFile {
        lang,
        messages,
        groups,
        infos: FluentInformations::from_lines(&infos_re, info_lines),
        junk,
        entries,
    })
}
