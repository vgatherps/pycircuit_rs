use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::{
    codegen::case::pascal_ident_span,
    parse::{bodies::CallBody, types::OutputType, ComponentDefinition},
};

use super::case::pascal_ident;

const OUTPUT_GENERIC_NAME: &str = "_OutputGeneric";
const OUTPUT_TRAIT_SUFFIX: &str = "Output";
const OUTPUT_VALID_TRAIT_SUFFIX: &str = "OutputValid";

// TODO always valid inputs
fn generate_fnc_for(name: &Ident, ty: &OutputType) -> TokenStream {
    let pascal_ident = pascal_ident(&[name]);
    let inner_ty = match ty {
        OutputType::FromTrait(the_trait) => {
            let ident = Ident::new(OUTPUT_GENERIC_NAME, Span::call_site());
            quote! {<#ident as #the_trait>::#pascal_ident}
        }
        OutputType::Concrete(concrete) => {
            quote! {#concrete}
        }
    };
    quote! {fn #name(&self) -> &mut #inner_ty;}
}

pub fn generate_output_trait(
    component: &ComponentDefinition,
    call_name: &Ident,
    call: &CallBody,
) -> TokenStream {
    let input_to_call: HashMap<_, _> = call
        .writes
        .set
        .iter()
        .map(|output| (output, component.outputs.outputs.map.get(output).unwrap()))
        .collect();

    let all_traits: HashSet<_> = input_to_call
        .values()
        .filter_map(|ty| match ty {
            OutputType::FromTrait(the_trait) => Some(the_trait),
            OutputType::Concrete(_) => None,
        })
        .collect();

    let all_fncs = input_to_call
        .iter()
        .map(|(input, ty)| generate_fnc_for(input, ty));

    let call_ident = pascal_ident_span(
        &[&component.name, call_name, &OUTPUT_TRAIT_SUFFIX],
        call_name.span(),
    );

    let trait_gen = if all_traits.is_empty() {
        quote! {}
    } else {
        let ident = Ident::new(OUTPUT_GENERIC_NAME, Span::call_site());
        let all_traits = all_traits.into_iter();
        quote! {<#ident : #(#all_traits)+*>}
    };

    quote! {
        pub trait #call_ident #trait_gen {
            #(#all_fncs)*
        }
    }
}

pub fn generate_output_valid_struct(
    component: &ComponentDefinition,
    call_name: &Ident,
    call: &CallBody,
) -> TokenStream {
    let bool_fields = call
        .writes
        .set
        .iter()
        .map(|output| quote! {pub #output: bool});

    let call_ident = pascal_ident_span(
        &[&component.name, call_name, &OUTPUT_VALID_TRAIT_SUFFIX],
        call_name.span(),
    );
    quote! {
        pub struct #call_ident {
            #(#bool_fields),*
        }
    }
}
