use super::{
    super::{helpers::{make_error_from_error_list, filter_comment}, decoder::IDecoder, FluentInformations, FluentGroup},
    entry::EntryType,
    FluentFile,
};

use regex::Regex;
use fluent_syntax::{ast::Entry, parser::parse as parse_fluent};
use std::{io::{Result, Error, ErrorKind}, collections::HashMap, path::Path, fs::read_to_string};

pub(super) fn read(lang: String, path: &Path) -> Result<FluentFile> {
    let infos_re = match Regex::new(r"^\s*@([\w-]+)[^:]*:\s*(\S.*)\s*$") {
        Ok(re) => re,
        Err(err) => { return Err(Error::new(ErrorKind::Other, err)); }
    };

    let source = read_to_string(path)?;
    let resource = match parse_fluent(source.as_str()) {
        Ok(r) => r,
        Err((_, errs)) => {
            return Err(make_error_from_error_list("Parse error while reading fluent file", path, errs));
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

    for entry in resource.body {
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
                let lines = c.content.iter().filter_map(filter_comment).collect::<Vec<_>>();

                if !lines.is_empty() {
                    entries.push(EntryType::Comment(lines));
                }
            }
            Entry::GroupComment(c) => {
                let lines = c.content.iter().filter_map(filter_comment).collect::<Vec<_>>();
                if lines.is_empty() {
                    continue;
                }

                if let Some(infos) = current_group_infos.take() {
                    groups.push(FluentGroup::new(current_group_name.take(), current_group_message_ids, infos));
                    current_group_message_ids = vec![];
                }

                let mut infos = FluentInformations::from_lines(&infos_re, lines);

                entries.push(EntryType::Group);
                current_group_name = infos.remove_header("group-name");
                current_group_infos.replace(infos);
            }
            Entry::ResourceComment(c) => {
                if info_lines.is_empty() {
                    entries.push(EntryType::ResourceHeader);
                }

                info_lines.extend(c.content.iter().filter_map(filter_comment));
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
