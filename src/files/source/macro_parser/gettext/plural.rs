use super::MsgProducer;
use proc_macro2::{TokenTree, Span, Spacing};
use syn::{Result, Error};

pub(super) enum PluralArg {
    None,
    Some {
        msg_id: String,
        count: String,
    }
}

impl PluralArg {
    pub(super) fn parse<P, I>(span: Span, producer: &mut P, it: &mut I) -> Result<Self>
        where P: MsgProducer, I: Iterator<Item=TokenTree> {
        let tok = match it.next() {
            Some(v) => v,
            None => {
                return Err(Error::new(span, "A string was expected after the punctuation `|`"));
            }
        };

        let lit_span = tok.span();
        let translation_msg = match tok {
            TokenTree::Literal(v) => v,
            tok => {
                let message = format!("A string literal was expected, instead `{}` was found", tok);

                return Err(Error::new(lit_span, message));
            }
        };

        let p = match it.next() {
            Some(v) => v,
            None => {
                return Err(Error::new(translation_msg.span(), "A percent was expected after the string"));
            }
        };

        let p = match p {
            TokenTree::Punct(p) if p.spacing() == Spacing::Alone && p.as_char() == '%' => p,
            p => {
                let message = format!("A percent sign was expected, instead `{}` was found", p);

                return Err(Error::new(p.span(), message));
            }
        };

        let val = match it.next() {
            Some(v) => match v {
                TokenTree::Punct(p) if p.spacing() == Spacing::Alone && p.as_char() == '$' => match it.next() {
                    Some(v) => v,
                    None => {
                        return Err(Error::new(p.span(), "Something was expected after the dollar sign"));
                    }
                }
                v => v,
            }
            None => {
                return Err(Error::new(p.span(), "Something was expected after the percent sign"));
            }
        };

        let count = match val {
            TokenTree::Ident(id) => id.to_string().replace("_", "-"),
            _ => String::from("count"),
        };

        let placeholder = format!("{{ ${} }}", count);
        let msg_id = producer.parse_string_literal(&translation_msg)?.replace("{n}", &placeholder);

        Ok(Self::Some { msg_id, count })
    }
}
