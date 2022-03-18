use simple_error::SimpleError;
use std::{io::{Error, ErrorKind}, fmt::Write};

fn format_error_line(err: syn::Error) -> String {
    let loc = err.span().start();

    format!("{}:{}> {}", loc.line, loc.column, err)
}

fn format_list_item(s: &mut String, err: syn::Error, indent: &String) {
    write!(s, "\n{}- {}", indent, format_errors(err, &indent, false)).unwrap();
}

fn format_errors(err: syn::Error, indent: &str, is_tail: bool) -> String {
    let mut it = err.into_iter();
    let first = match it.next() {
        Some(v) => v,
        None => { return String::new(); }
    };

    match it.next() {
        None => {
            if is_tail {
                format_error_line(first)
            } else {
                format_errors(first, indent, true)
            }
        }

        Some(second) => {
            let indent = String::from(indent) + "  ";
            let mut res = if is_tail {
                String::from("Following errors occurred:")
            } else {
                String::new()
            };

            format_list_item(&mut res, first, &indent);
            format_list_item(&mut res, second, &indent);

            for err in it {
                format_list_item(&mut res, err, &indent);
            }

            res
        }
    }
}

pub(super) struct ParseErrorFormatter(syn::Error);

impl ParseErrorFormatter {
    pub(super) fn from_list(list: Vec<syn::Error>) -> Option<Self> {
        let mut it = list.into_iter();
        let mut res = match it.next() {
            Some(err) => ParseErrorFormatter(err),
            None => { return None; }
        };

        for err in it {
            res.0.combine(err);
        }

        Some(res)
    }

    pub(super) fn new(err: syn::Error) -> Self {
        ParseErrorFormatter(err)
    }

    pub(super) fn into_error(self, name: &String) -> Error {
        let err = format_errors(self.0, "", false);
        let msg = if name.is_empty() {
            format!("Parse error: {}", err)
        } else {
            format!("Parse error in {}:{}", name, err)
        };

        Error::new(ErrorKind::Other, SimpleError::new(msg))
    }
}
