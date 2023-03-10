use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::{
    codegen::case::pascal_ident_span,
    parse::{bodies::CallBody, types::OutputType, ComponentDefinition},
};

const OUTPUT_GENERIC_NAME: &str = "_OutputGeneric";
const OUTPUT_TRAIT_SUFFIX: &str = "Output";
const OUTPUT_TRAIT_EXPORT_SUFFIX: &str = "OutputExport";
const OUTPUT_VALID_TRAIT_SUFFIX: &str = "OutputValid";

// TODO always valid inputs

fn generate_type_for(name: &Ident, ty: &OutputType) -> TokenStream {
    let ident = Ident::new(OUTPUT_GENERIC_NAME, Span::call_site());
    match ty {
        OutputType::FromTrait(the_trait) => {
            let pascal_name = pascal_ident_span(&[name], name.span());
            quote! {<#ident as #the_trait>::#pascal_name}
        }
        OutputType::Concrete(concrete) => {
            quote! {#concrete}
        }
    }
}

pub fn generate_output_trait(
    component: &ComponentDefinition,
    call_name: &Ident,
    call: &CallBody,
) -> TokenStream {
    let gen_ident = Ident::new(OUTPUT_GENERIC_NAME, Span::call_site());

    let export_ident = pascal_ident_span(
        &[&component.name, &OUTPUT_TRAIT_EXPORT_SUFFIX],
        component.name.span(),
    );

    let call_ident = pascal_ident_span(
        &[&component.name, call_name, &OUTPUT_TRAIT_SUFFIX],
        call_name.span(),
    );
    let fncs = call.writes.set.iter().map(|name| {
        let pascal_out = pascal_ident_span(&[name], name.span());
        quote! {
            fn #name(&self) -> &mut <#gen_ident as #export_ident>::#pascal_out;
        }
    });

    quote! {
        pub trait #call_ident<#gen_ident: #export_ident> {
            #(#fncs)*
        }
    }
}

pub fn generate_output_export_trait(component: &ComponentDefinition) -> TokenStream {
    let named_fields = component.outputs.outputs.map.keys().map(|name| {
        let pascal_out = pascal_ident_span(&[name], name.span());
        quote! {type #pascal_out;}
    });

    let call_ident = pascal_ident_span(
        &[&component.name, &OUTPUT_TRAIT_EXPORT_SUFFIX],
        component.name.span(),
    );

    quote! {
        pub trait #call_ident {
            #(#named_fields)*
        }
    }
}

pub fn generate_output_export_impl(component: &ComponentDefinition) -> TokenStream {
    let output_to_call: HashMap<_, _> = component.outputs.outputs.map.clone();

    let trait_ident = pascal_ident_span(
        &[&component.name, &OUTPUT_TRAIT_EXPORT_SUFFIX],
        component.name.span(),
    );

    let all_traits: HashSet<_> = output_to_call
        .values()
        .filter_map(|ty| match ty {
            OutputType::FromTrait(the_trait) => Some(the_trait),
            OutputType::Concrete(_) => None,
        })
        .collect();

    let all_types = output_to_call.iter().map(|(input, ty)| {
        let trait_ty = generate_type_for(input, ty);
        let name = pascal_ident_span(&[input], input.span());
        quote! {
            type #name = #trait_ty;
        }
    });

    let gen_type_ident = Ident::new(OUTPUT_GENERIC_NAME, Span::call_site());

    let trait_gen = if all_traits.is_empty() {
        quote! {<#gen_type_ident>}
    } else {
        let all_traits = all_traits.into_iter();
        quote! {<#gen_type_ident : #(#all_traits)+*>}
    };

    quote! {
        impl #trait_gen #trait_ident for #gen_type_ident {
            #(#all_types)*
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
