use proc_macro2::Ident;
use syn::{parse::Parse, token::Type as TType, Type};

use crate::parse::kws;

#[derive(Clone)]
pub enum InputType {
    Generic(Ident),
    Concrete(Type),
}

#[derive(Clone)]
pub enum OutputType {
    FromTrait(Ident),
    Concrete(Type),
}

impl Parse for InputType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kws::generic) {
            input.parse::<kws::generic>()?;
            Ok(InputType::Generic(input.parse()?))
        } else if lookahead.peek(TType) {
            Ok(InputType::Concrete(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for OutputType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kws::using) {
            input.parse::<kws::using>()?;
            Ok(OutputType::FromTrait(input.parse()?))
        } else if lookahead.peek(TType) {
            Ok(OutputType::Concrete(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}
