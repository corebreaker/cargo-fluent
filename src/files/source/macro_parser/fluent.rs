use super::{super::Origin, MsgProducer};
use proc_macro2::{TokenStream, TokenTree, Span, Spacing};
use syn::{Error, Result};

#[derive(Clone)]
enum State { Skip, Message, Arguments, ArgSep(String) }

pub(super) fn parse_macro(producer: &mut impl MsgProducer, macro_span: Span, tokens: TokenStream) -> Result<()> {
    let mut state = State::Skip;
    let mut first: Option<TokenTree> = None;
    let mut last: Option<TokenTree> = None;
    let mut message: Option<String> = None;
    let mut args = vec![];

    for tok in tokens {
        match state {
            State::Skip => {
                match &tok {
                    TokenTree::Punct(p) if p.spacing() == Spacing::Alone && p.as_char() == ',' => {
                        state = if message.is_none() {State::Message} else {State::Arguments};
                    }
                    _ => {}
                }

                if first.is_none() {
                    first.replace(tok);
                } else if message.is_none() {
                    last.replace(tok);
                }
            }
            State::Message => {
                match &tok {
                    TokenTree::Literal(lit) => {
                        message.replace(producer.parse_string_literal(lit)?);
                        state = State::Skip;
                    }
                    _ => {
                        let message = format!("A string literal was expected, instead `{}` was found", tok);

                        return Err(Error::new(tok.span(), message));
                    }
                }
            }
            State::Arguments => {
                match &tok {
                    TokenTree::Ident(id) => {
                        state = State::ArgSep(id.to_string());
                    }
                    _ => {
                        let message = format!("An identifier was expected, instead `{}` was found", tok);

                        return Err(Error::new(tok.span(), message));
                    }
                }
            }
            State::ArgSep(arg) => {
                if let TokenTree::Punct(p) = &tok {
                    if p.spacing() == Spacing::Alone && p.as_char() == '=' {
                        args.push(arg);
                    }
                }

                state = State::Skip;
            }
        }
    }

    match message {
        Some(msg_id) => {
            let t = producer.new_translation(Origin::Fluent, msg_id, macro_span);

            for name in args {
                t.add_paramter(name);
            }

            Ok(())
        }
        None => {
            let (span, message) = match first {
                None => (macro_span, String::from("Call of `fl!` macro should not be empty, loader expected")),
                Some(first) => match last {
                    None => (first.span(), String::from("Coma was expected after the loader, message ID is missing")),
                    Some(last) => (last.span(), format!("Message ID was expected, instead `{}` was found", last)),
                }
            };

            Err(Error::new(span, message))
        }
    }
}
