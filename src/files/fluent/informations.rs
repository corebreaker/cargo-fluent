use super::helpers::add_header;
use regex::Regex;
use titlecase::titlecase;
use itertools::Itertools;
use std::{io::{Write, Result}, collections::{HashMap}, hash::Hash, borrow::Borrow};

#[derive(Debug)]
pub struct FluentInformations {
    headers: HashMap<String, String>,
    comments: Vec<String>,
}

impl FluentInformations {
    pub(crate) fn new() -> Self {
        FluentInformations {
            headers: HashMap::new(),
            comments: vec![],
        }
    }

    pub(super) fn from_lines(re: &Regex, lines: Vec<String>) -> Self {
        let mut headers = HashMap::new();
        let mut comments = vec![];

        for comment in lines {
            if let Some(caps) = re.captures(&comment) {
                if let Some(name) = caps.get(1) {
                    if let Some(value) = caps.get(2) {
                        add_header(&mut headers, name.as_str(), value.as_str());
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

    pub(crate) fn contains_comments(&self, infos: &Self) -> bool {
        self.comments.join("|").contains(&infos.comments.join("|"))
    }

    pub(crate) fn add_comments(&mut self, infos: Self) {
        self.comments.extend(infos.comments)
    }

    pub(crate) fn clear(&mut self) {
        self.headers.clear();
        self.comments.clear();
    }

    pub(crate) fn set_header(&mut self, name: &str, value: String) {
        self.headers.insert(name.to_lowercase(), value);
    }

    pub(crate) fn add_header(&mut self, name: &str, value: String) {
        add_header(&mut self.headers, name, &value);
    }

    pub(crate) fn remove_header<N: Hash + Eq + ?Sized>(&mut self, name: &N) -> Option<String> where String: Borrow<N> {
        self.headers.remove(name)
    }

    pub(crate) fn extends_headers(&mut self, iter: impl IntoIterator<Item = (String, String)>) {
        self.headers.extend(iter)
    }

    pub(crate) fn add_comment(&mut self, comment: String) {
        self.comments.push(comment);
    }

    pub(super) fn write<W: Write>(&self, w: &mut W, header: Option<&String>, prefix: &str) -> Result<()> {
        for comment in &self.comments {
            writeln!(w, "{} {}", prefix, comment)?;
        }

        if let Some(header) = header {
            writeln!(w, "{} @{}", prefix, header)?;
        }

        for name in self.headers.keys().sorted() {
            let value = &self.headers[name];

            if name == "X-Generator" && value.starts_with("Poedit") || name == "Plural-Forms" {
                continue;
            }

            let prefix = format!("{} @{}:", prefix, titlecase(name));
            let prefix_size = prefix.chars().count();
            let mut line = prefix.clone();
            let mut size = prefix_size;

            for v in value.split(" ") {
                let v_sz = v.chars().count() + 1;

                if (size + v_sz) >= 100 {
                    writeln!(w, "{}", line)?;

                    line = format!("{} {}", prefix, v);
                    size = prefix_size + v_sz;
                } else {
                    size += v_sz;
                    line.push_str(" ");
                    line.push_str(v);
                }
            }

            writeln!(w, "{}", line)?;
        }

        Ok(())
    }
}
