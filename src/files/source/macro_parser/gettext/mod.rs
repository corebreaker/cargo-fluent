mod plural;
mod parser;

use super::MsgProducer;
use proc_macro2::{TokenStream, Span};
use syn::Result;

pub(super) fn parse_macro(producer: &mut impl MsgProducer, macro_span: Span, tokens: TokenStream) -> Result<()> {
    parser::Parser::new(producer, macro_span, tokens.into_iter()).parse(None)
}
