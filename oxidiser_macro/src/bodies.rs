use proc_macro2::Ident;
use syn::parse::Parse;

use crate::{
    kws::{Observes, Takes, Writes},
    parse_helpers::{parse_named, ParseMapOf, ParseSetOf},
    types::InputType,
};

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
    pub outputs: ParseMapOf<Ident, InputType>,
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
        Ok(CallBody {
            takes: parse_named::<Takes, _>(input)?,
            observes: parse_named::<Observes, _>(input)?,
            writes: parse_named::<Writes, _>(input)?,
        })
    }
}

pub struct AllCallsBody {
    pub calls: ParseMapOf<Ident, CallBody>,
}

impl Parse for AllCallsBody {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(AllCallsBody {
            calls: input.parse()?,
        })
    }
}
