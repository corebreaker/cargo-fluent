mod fluent;
mod gettext;

use super::MsgProducer;
use syn::{parse::Parser, spanned::Spanned, Result};
use proc_macro2::{Spacing, Span, TokenStream, TokenTree};

pub(super) struct MacroParser<'t, P: MsgProducer> {
    producer: &'t mut P,
}

impl<'t, P: MsgProducer> MacroParser<'t, P> {
    pub(super) fn new(producer: &'t mut P) -> Self {
        MacroParser { producer }
    }

    pub(super) fn parse_tr(&mut self, macro_span: Span, tokens: TokenStream) -> Result<()> {
        gettext::parse_macro(self.producer, macro_span, tokens)
    }

    pub(super) fn parse_fl(&mut self, macro_span: Span, tokens: TokenStream) -> Result<()> {
        fluent::parse_macro(self.producer, macro_span, tokens)
    }

    fn parse_step(&mut self, state: usize, span: &mut Span, token: TokenTree) -> Result<usize> {
        match state {
            0 => match token {
                TokenTree::Group(g) => { MacroParser::new(self.producer).parse2(g.stream())?; }
                TokenTree::Ident(id) => {
                    if id == "tr" {
                        *span = id.span();

                        return Ok(1);
                    } else if id == "fl" {
                        *span = id.span();

                        return Ok(3);
                    }
                }
                _ => {}
            }

            1 => if let TokenTree::Punct(p) = token {
                if p.as_char() == '!' && p.spacing() == Spacing::Alone {
                    return Ok(2);
                }
            }

            2 => if let TokenTree::Group(g) = token {
                self.parse_tr(*span, g.stream())?;
            }

            3 => if let TokenTree::Punct(p) = token {
                if p.as_char() == '!' && p.spacing() == Spacing::Alone {
                    return Ok(4);
                }
            }

            4 => if let TokenTree::Group(g) = token {
                self.parse_fl(*span, g.stream())?;
            }

            _ => {}
        }

        Ok(0)
    }
}

impl<'t, P: MsgProducer> Parser for MacroParser<'t, P> {
    type Output = ();

    fn parse2(mut self, tokens: TokenStream) -> Result<Self::Output> {
        let mut state = 0;
        let mut span = tokens.span();

        for token in tokens {
            state = self.parse_step(state, &mut span, token)?;
        }

        Ok(())
    }
}
