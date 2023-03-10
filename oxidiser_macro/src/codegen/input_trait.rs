use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::parse::{bodies::CallBody, types::InputType, ComponentDefinition};

use super::case::pascal_ident_span;

const INPUT_TRAIT_SUFFIX: &str = "Input";

// TODO always valid inputs
fn generate_fnc_for(name: &Ident, ty: &InputType) -> TokenStream {
    let inner_ty = match ty {
        InputType::Generic(generic) => {
            quote! {Self::#generic}
        }
        InputType::Concrete(concrete) => {
            quote! {#concrete}
        }
    };
    quote! {fn #name(&self) -> Option<& #inner_ty>;}
}

fn generate_type_for(generic: &Ident) -> TokenStream {
    quote! {type #generic;}
}

fn generate_all_types(input: &[&InputType]) -> TokenStream {
    let all_generics: HashSet<_> = input
        .iter()
        .filter_map(|ty| match ty {
            InputType::Generic(generic) => Some(generic),
            InputType::Concrete(_) => None,
        })
        .collect();

    let all_types = all_generics.into_iter().map(generate_type_for);

    quote! {
        #(#all_types)*
    }
}

pub fn generate_input_trait_for(
    input_to_call: HashMap<Ident, InputType>,
    trait_name: Ident,
) -> TokenStream {
    let relevant_inputs: Vec<_> = input_to_call.values().collect();

    let all_types = generate_all_types(&relevant_inputs);

    let all_fncs = input_to_call
        .iter()
        .map(|(input, ty)| generate_fnc_for(input, ty));

    quote! {
        pub trait #trait_name {
            #all_types

            #(#all_fncs)*
        }
    }
}

pub fn generate_input_trait(
    component: &ComponentDefinition,
    call_name: &Ident,
    call: &CallBody,
) -> TokenStream {
    let input_to_call: HashMap<_, _> = call
        .takes
        .set
        .iter()
        .chain(call.observes.set.iter())
        .map(|input| {
            (
                input.clone(),
                component.inputs.inputs.map.get(input).unwrap().clone(),
            )
        })
        .collect();
    let call_ident = pascal_ident_span(
        &[&component.name, call_name, &INPUT_TRAIT_SUFFIX],
        call_name.span(),
    );
    generate_input_trait_for(input_to_call, call_ident)
}
