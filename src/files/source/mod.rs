mod error;
mod origin;
mod parser;
mod message;
mod macro_parser;
mod messages;

pub(crate) use self::{
    origin::Origin,
    message::Message,
};

use self::error::ParseErrorFormatter;
use pathdiff::diff_paths;
use path_absolutize::Absolutize;
use proc_macro2::{Span, Literal};
use std::{io::{Result, Read, Error, ErrorKind}, fs::File, path::Path, env::current_dir};

#[derive(Debug)]
pub(crate) struct RustSource {
    pub(crate) name: String,
    pub(crate) translations: Vec<Message>,
}

impl RustSource {
    pub(crate) fn read(path: &Path) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        let path = if path.is_absolute() { path.to_path_buf() } else { path.absolutize().map(|p| p.to_path_buf())? };
        let rel_path = diff_paths(&path, current_dir()?).unwrap_or(path);
        let name = rel_path.to_string_lossy().to_string();

        let ast = match syn::parse_file(&content) {
            Ok(file) => file,
            Err(err) => {
                return Err(Error::new(ErrorKind::Other, ParseErrorFormatter::new(err).into_error(&name)));
            }
        };

        let translations = parser::FileParser::visit_ast(&name, &ast)?;

        Ok(RustSource { name, translations })
    }
}

trait MsgProducer {
    fn new_translation(&mut self, origin: Origin, msg_id: String, span: Span) -> &mut Message;
    fn parse_string_literal(&self, lit: &Literal) -> syn::Result<String>;
}