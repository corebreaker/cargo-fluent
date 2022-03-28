use super::{FluentFile, FluentMessage, entry::EntryType};
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

        for (msg_id, message) in &self.file.messages {
            if !self.visited_messages.contains(msg_id) {
                self.write_message(message)?;
            }
        }

        Ok(())
    }

    fn write_message(&mut self, message: &FluentMessage) -> Result<()> {
        let value = message.value().map_or_else(String::new, |v| format!(" = {}", v));
        let attributes = message.attributes().iter()
            .map(|(k, v)| format!("\n  .{} = {}", k, v))
            .join("");

        self.group_visited_messages.insert(message.id().clone());
        writeln!(self.w, "\n{}{}{}", message.id(), value, attributes)
    }

    fn write_entry(&mut self, entry: &EntryType) -> Result<()> {
        match entry {
            EntryType::Junk => {
                let idx = self.junk_idx;

                self.junk_idx += 1;
                writeln!(self.w, "\n{}", self.file.junk[idx])?;
            }
            EntryType::Group => {
                self.flush_group()?;
                self.file.groups[self.group_idx].write(&mut self.w)?;
            }
            EntryType::ResourceHeader => { self.file.write_header(&mut self.w)?; }
            EntryType::Message(message) => { self.write_message(&self.file.messages[message])?; }
            EntryType::Comment(lines) => {
                for line in lines {
                    write!(self.w, "\n# {}", line)?;
                }

                writeln!(self.w, "")?;
            }
        }

        self.flush_file()
    }

    fn write(&mut self) -> Result<()> {
        for entry in &self.file.entries {
            self.write_entry(entry)?;
        }

        Ok(())
    }
}

pub(super) fn write<W: Write>(file: &FluentFile, w: W) -> Result<()> {
    FileWriter::new(file, w).write()
}
