use std::collections::{HashMap, HashSet};

use convert_case::{Case::Camel, Converter};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::parse::{bodies::CallBody, types::OutputType, ComponentDefinition};

const OUTPUT_GENERIC_NAME: &str = "_OutputGeneric";
const OUTPUT_TRAIT_SUFFIX: &str = "Output";

// TODO always valid inputs
fn generate_fnc_for(name: &Ident, ty: &OutputType) -> TokenStream {
    let to_camel = Converter::new().from_case(Camel);
    let camel_name_str = to_camel.convert(name.to_string());
    let camel_ident = Ident::new(&camel_name_str, Span::call_site());
    let inner_ty = match ty {
        OutputType::FromTrait(the_trait) => {
            let _ident = Ident::new(OUTPUT_GENERIC_NAME, Span::call_site());
            quote! {<ident as #the_trait>::#camel_ident}
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

    let to_camel = Converter::new().from_case(Camel);
    // CamelCase
    let camelcase_component = to_camel.convert(component.name.to_string());
    let camelcase_call = to_camel.convert(call_name.to_string());

    let call_trait = format!("{camelcase_component}{camelcase_call}{OUTPUT_TRAIT_SUFFIX}");

    let call_ident = Ident::new(&call_trait, call_name.span());

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
