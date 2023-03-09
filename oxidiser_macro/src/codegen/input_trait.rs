use std::collections::{HashMap, HashSet};

use convert_case::{Case::Camel, Converter};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::parse::{bodies::CallBody, types::InputType, ComponentDefinition};

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
        #(#all_types);*
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
        .map(|input| (input, component.inputs.inputs.map.get(input).unwrap()))
        .collect();

    let relevant_inputs: Vec<_> = input_to_call.values().copied().collect();

    let all_types = generate_all_types(&relevant_inputs);

    let all_fncs = input_to_call
        .iter()
        .map(|(input, ty)| generate_fnc_for(input, ty));

    let to_camel = Converter::new().from_case(Camel);
    // CamelCase
    let camelcase_component = to_camel.convert(component.name.to_string());
    let camelcase_call = to_camel.convert(call_name.to_string());

    let call_trait = format!("{camelcase_component}{camelcase_call}{INPUT_TRAIT_SUFFIX}");

    let call_ident = Ident::new(&call_trait, call_name.span());

    quote! {
        pub trait #call_ident {
            #all_types

            #(#all_fncs)*
        }
    }
}
