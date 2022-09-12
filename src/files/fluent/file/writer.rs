use super::{FluentFile, FluentMessage, FluentInformations, entry::EntryType};
use itertools::Itertools;
use std::{io::{Write, Result}, collections::HashSet};

struct FileWriter<'f, W: Write> {
    file: &'f FluentFile,
    w: W,
    visited_messages: HashSet<String>,
    group_visited_messages: HashSet<String>,
    group_idx: usize,
    junk_idx: usize,
}

impl<'f, W: Write> FileWriter<'f, W> {
    fn new(file: &'f FluentFile, w: W) -> Self {
        FileWriter {
            file,
            w,
            visited_messages: HashSet::new(),
            group_visited_messages: HashSet::new(),
            group_idx: 0,
            junk_idx: 0,
        }
    }

    fn flush_group(&mut self) -> Result<()> {
        if self.group_idx >= self.file.groups.len() {
            return Ok(())
        }

        for msg_id in self.file.groups[self.group_idx].message_ids() {
            if !self.group_visited_messages.contains(msg_id) {
                self.write_message(&self.file.messages[msg_id])?;
            }
        }

        self.group_idx += 1;
        self.visited_messages.extend(self.group_visited_messages.drain());
        Ok(())
    }

    fn flush_file(&mut self) -> Result<()> {
        self.flush_group()?;

        let sz = self.file.groups.len();

        while self.group_idx < sz {
            self.flush_group()?;
        }

        for msg_id in self.file.messages.keys().sorted() {
            if !self.visited_messages.contains(msg_id) {
                self.write_message(&self.file.messages[msg_id])?;
            }
        }

        Ok(())
    }

    fn write_message(&mut self, message: &FluentMessage) -> Result<()> {
        message.informations().write(&mut self.w, None, "#")?;

        let value = message.value().map_or_else(String::new, |v| format!(" = {}", v));

        self.group_visited_messages.insert(message.id().clone());
        self.w.write_all(message.id().as_bytes())?;
        self.w.write_all(value.as_bytes())?;

        for (k, v) in message.attributes() {
            writeln!(self.w)?;
            write!(self.w, "  .{} = {}", k, v)?;
        }

        writeln!(self.w)
    }

    fn write_entry(&mut self, entry: &EntryType) -> Result<()> {
        match entry {
            EntryType::Junk => {
                let idx = self.junk_idx;

                self.junk_idx += 1;
                writeln!(self.w)?;
                self.w.write_all(self.file.junk[idx].as_bytes())?;
                writeln!(self.w)?;
            }
            EntryType::Group => {
                self.flush_group()?;
                self.file.groups[self.group_idx].write(&mut self.w)?;
            }
            EntryType::ResourceHeader => { self.file.write_header(&mut self.w)?; }
            EntryType::Message(message) => {
                let message = &self.file.messages[message];

                writeln!(self.w)?;
                self.write_message(message)?;

                let id = message.id();

                self.visited_messages.insert(id.clone());
                if self.group_idx < self.file.groups.len() {
                    self.group_visited_messages.insert(id.clone());
                }
            }
            EntryType::Comment(lines) => {
                for line in lines {
                    writeln!(self.w)?;
                    write!(self.w, "# {}", line)?;
                }

                writeln!(self.w)?;
            }
        }

        Ok(())
    }

    fn write(&mut self) -> Result<()> {
        for entry in &self.file.entries {
            self.write_entry(entry)?;
            writeln!(self.w, "# -----------------------------------------------------------------------------")?;
        }

        self.flush_file()
    }
}

pub(super) fn write<W: Write>(file: &FluentFile, w: W) -> Result<()> {
    FileWriter::new(file, w).write()
}
