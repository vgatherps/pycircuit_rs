use proc_macro2::Ident;
use syn::{braced, parse::Parse};

use crate::parse::{
    kws::{Observes, Takes, Writes},
    parse_helpers::{parse_body_term, parse_named, ParseMapOf, ParseSetOf},
    types::InputType,
};

use super::{parse_helpers::ParseNamed, types::OutputType};

pub struct InputBody {
    pub inputs: ParseMapOf<Ident, InputType>,
}

impl Parse for InputBody {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(InputBody {
            inputs: input.parse()?,
        })
    }
}

pub struct OutputBody {
    pub outputs: ParseMapOf<Ident, OutputType>,
}

impl Parse for OutputBody {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(OutputBody {
            outputs: input.parse()?,
        })
    }
}

pub struct CallBody {
    pub takes: ParseSetOf<Ident>,
    pub observes: ParseSetOf<Ident>,
    pub writes: ParseSetOf<Ident>,
}

impl Parse for CallBody {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        braced!(content in input);
        let body = CallBody {
            takes: parse_body_term(parse_named::<Takes, _>, &content)?,
            observes: parse_body_term(parse_named::<Observes, _>, &content)?,
            writes: parse_body_term(parse_named::<Writes, _>, &content)?,
        };

        let lookahead = content.lookahead1();

        if lookahead.peek(syn::token::Comma) {
            content.parse::<syn::token::Comma>()?;
        } else if !content.is_empty() {
            return Err(lookahead.error());
        }

        if !content.is_empty() {
            return Err(syn::Error::new(
                content.span(),
                "unexpected tokens after call body",
            ));
        }

        Ok(body)
    }
}

pub struct AllCallsBody {
    pub calls: ParseNamed<CallBody>,
}

impl Parse for AllCallsBody {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(AllCallsBody {
            calls: input.parse()?,
        })
    }
}
