use super::{super::{super::{Origin, Message}, MsgProducer}, plural::PluralArg};
use proc_macro2::{TokenTree, Spacing, Span};
use syn::{Result, Error};

pub(super) struct Parser<'t, P: MsgProducer, I: Iterator<Item=TokenTree>> {
    producer: &'t mut P,
    macro_span: Span,
    iterator: I,
}

impl<'t, P: MsgProducer, I: Iterator<Item=TokenTree>> Parser<'t, P, I> {
    pub(super) fn new(producer: &'t mut P, macro_span: Span, iterator: I) -> Self {
        Parser { producer, macro_span, iterator }
    }

    fn new_translation(&mut self, msg_id: String, context: Option<String>, plural: PluralArg) -> &mut Message {
        let t = self.producer.new_translation(Origin::Gettext, msg_id, self.macro_span);

        if let Some(context) = context {
            t.set_attribute(String::from("$context"), context);
        }

        if let PluralArg::Some { msg_id, count } = plural {
            t.set_attribute(String::from("@plural"), msg_id);
            t.set_attribute(String::from("@count"), count);
        }

        t
    }

    fn parse_args(&mut self, mut msg_id: String, plural: PluralArg, context: Option<String>) -> Result<()> {
        #[derive(Copy, Clone)]
        enum State { Search, Skip }

        let mut args = vec![];
        let mut state = State::Search;

        for tok in &mut self.iterator {
            state = match state {
                State::Search => {
                    let name = match tok {
                        TokenTree::Ident(id) => id.to_string().replace("_", "-"),
                        _ => format!("arg-{}", args.len()),
                    };

                    let placeholder = format!("{{ ${} }}", name);
                    let msg = msg_id.replacen("{}", &placeholder, 1);

                    if msg_id != msg {
                        msg_id = msg;
                        args.push(name);
                    }

                    State::Skip
                }

                State::Skip => match tok {
                    TokenTree::Punct(p) if p.spacing() == Spacing::Alone && p.as_char() == ',' => State::Search,
                    _ => State::Skip,
                }
            }
        }

        let t = self.new_translation(msg_id, context, plural);

        for arg in args {
            t.add_paramter(arg);
        }

        Ok(())
    }

    fn parse_tail(&mut self, context: Option<String>, msg_id: String, tok: TokenTree) -> Result<bool> {
        if let TokenTree::Punct(p) = tok {
            match p.as_char() {
                '=' if context.is_none() && p.spacing() == Spacing::Joint => {
                    if let Some(arrow) = self.iterator.next() {
                        if let TokenTree::Punct(p_next) = arrow {
                            if p_next.as_char() == '>' && p_next.spacing() == Spacing::Alone {
                                self.parse(Some(msg_id))?;

                                return Ok(false);
                            }
                        }
                    }
                }
                ',' => {
                    self.parse_args(msg_id, PluralArg::None, context)?;

                    return Ok(false);
                }
                '|' => {
                    let plural = PluralArg::parse(p.span(), self.producer, &mut self.iterator)?;

                    if let Some(tail) = self.iterator.next() {
                        let (tok_str, span) = match tail {
                            TokenTree::Punct(p) => {
                                if p.as_char() == ',' && p.spacing() == Spacing::Alone {
                                    self.parse_args(msg_id, plural, context)?;

                                    return Ok(false);
                                }

                                (p.to_string(), p.span())
                            }
                            v => (v.to_string(), v.span()),
                        };

                        let message = format!("A comma was expected, instead `{}` was found", tok_str);

                        return Err(Error::new(span, message))
                    } else {
                        self.new_translation(msg_id, context, plural);

                        return Ok(false);
                    }
                }
                _ => {}
            }
        }

        Ok(true)
    }

    pub(super) fn parse(&mut self, context: Option<String>) -> Result<()> {
        let tok = match self.iterator.next() {
            Some(v) => v,
            None => {
                return Err(Error::new(self.macro_span,"A string literal was expected, nothing was found instead"));
            }
        };

        let translation_msg = match tok {
            TokenTree::Literal(v) => v,
            _ => {
                return Err(Error::new(tok.span(), "A string literal was expected"));
            }
        };

        let msg_id = self.producer.parse_string_literal(&translation_msg)?;
        let tail = match self.iterator.next() {
            Some(v) => v,
            None => {
                self.new_translation(msg_id, context.clone(), PluralArg::None);

                return Ok(());
            }
        };

        let has_context = context.is_some();
        if self.parse_tail(context, msg_id, tail.clone())? {
            let exp = (if has_context {vec![",", "|"]} else {vec!["=>", ",", "|"]}).join("`, `");
            let message = format!("A punctuation was expected among `{}`, instead `{}` was found", exp, tail);

            Err(Error::new(tail.span(), message))
        } else {
            Ok(())
        }
    }
}
