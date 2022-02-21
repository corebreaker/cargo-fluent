use regex::Regex;
use std::{io::{Write, Result}, collections::HashMap, hash::Hash, borrow::Borrow};

pub struct FluentInformations {
    headers: HashMap<String, String>,
    comments: Vec<String>,
}

impl FluentInformations {
    pub(super) fn new(re: &Regex, lines: Vec<String>) -> Self {
        let mut headers = HashMap::new();
        let mut comments = vec![];

        for comment in lines {
            if let Some(caps) = re.captures(&comment) {
                if let Some(name) = caps.get(1) {
                    if let Some(value) = caps.get(2) {
                        headers.insert(name.as_str().to_string(), value.as_str().to_string());
                        continue
                    }
                }
            }

            comments.push(comment);
        }

        Self { headers, comments }
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn comments(&self) -> &Vec<String> {
        &self.comments
    }

    pub(crate) fn clear(&mut self) {
        self.headers.clear();
        self.comments.clear();
    }

    pub(crate) fn set_header(&mut self, name: String, value: String) {
        self.headers.insert(name, value);
    }

    pub(crate) fn remove_header<N: Hash + Eq + ?Sized>(&mut self, name: &N) -> Option<String> where String: Borrow<N> {
        self.headers.remove(name)
    }

    pub(crate) fn extends_headers<I: IntoIterator<Item = (String, String)>>(&mut self, iter: I) {
        self.headers.extend(iter)
    }

    pub(crate) fn comments_mut(&mut self) -> &mut Vec<String> {
        &mut self.comments
    }

    pub(super) fn write<W: Write>(&self, w: &mut W, header: Option<&String>, prefix: &str) -> Result<()> {
        for comment in &self.comments {
            write!(w, "\n{} {}", prefix, comment)?;
        }

        if let Some(header) = header {
            write!(w, "\n{} @{}", prefix, header)?;
        }

        for (name, value) in &self.headers {
            write!(w, "\n{} @{}: {}", prefix, name, value)?;
        }

        Ok(())
    }
}
