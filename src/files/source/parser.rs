use super::{error::ParseErrorFormatter, macro_parser::MacroParser, messages::Messages, Message, MsgProducer};
use crate::files::source::Origin;
use syn::{visit::Visit, spanned::Spanned, Macro, File};
use proc_macro2::{Literal, Span};
use regex::Regex;
use std::io::Result;

pub(super) struct FileParser {
    strlit_re: Regex,
    name: String,
    errors: Vec<syn::Error>,
    translations: Messages,
}

impl FileParser {
    pub(super) fn visit_ast(name: &String, ast: &File) -> Result<Vec<Message>> {
        let mut parser = FileParser::new(name);

        parser.visit_file(ast);
        parser.result(name)
    }

    fn new(name: &String) -> FileParser {
        FileParser {
            strlit_re: Regex::new(r#"^"(.*)"$"#).unwrap(),
            name: name.clone(),
            errors: vec![],
            translations: Messages::new(),
        }
    }

    fn result(self, name: &String) -> Result<Vec<Message>> {
        match ParseErrorFormatter::from_list(self.errors) {
            None => Ok(self.translations.take()),
            Some(err) => Err(err.into_error(name)),
        }
    }

    fn push_error(&mut self, err: syn::Error) {
        self.errors.push(err)
    }
}

impl MsgProducer for FileParser {
    fn new_translation(&mut self, origin: Origin, msg_id: String, span: Span) -> &mut Message {
        self.translations.new_translation(origin, msg_id, span)
    }

    fn parse_string_literal(&self, lit: &Literal) -> syn::Result<String> {
        let s = lit.to_string();

        if let Some(caps) = self.strlit_re.captures(&s) {
            if let Some(v) =  caps.get(1) {
                return Ok(v.as_str().to_string());
            }
        }

        Err(syn::Error::new(lit.span(), format!("String literal was expected, instead `{}` was found", lit)))
    }
}

impl<'ast> Visit<'ast> for FileParser {
    fn visit_macro(&mut self, m: &'ast Macro) {
        if let Some(id) = m.path.get_ident() {
            let mut parser = MacroParser::new(self);

            if id == "tr" {
                if let Err(err) = parser.parse_tr(m.span(), m.tokens.clone()) {
                    self.push_error(err);
                }

                return;
            } else if id == "fl" {
                if let Err(err) = parser.parse_fl(m.span(), m.tokens.clone()) {
                    self.push_error(err);
                }

                return;
            }
        }

        if let Err(err) = m.parse_body_with(MacroParser::new(self)) {
            self.push_error(err);
        }
    }
}
