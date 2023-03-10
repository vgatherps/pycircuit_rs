use proc_macro2::TokenStream;
use quote::quote;

use crate::parse::ComponentDefinition;

pub(crate) mod case;
mod input_trait;
mod output_trait;

pub fn codegen(component: &ComponentDefinition) -> TokenStream {
    let call_traits = component
        .all_calls
        .calls
        .map
        .iter()
        .map(|(call_name, call)| {
            let input_trait = input_trait::generate_input_trait(component, call_name, call);
            let output_trait = output_trait::generate_output_trait(component, call_name, call);
            let output_export_trait = output_trait::generate_output_export_trait(component);
            let output_export_impl = output_trait::generate_output_export_impl(component);
            let output_valid_trait =
                output_trait::generate_output_valid_struct(component, call_name, call);
            quote! {
                #input_trait
                #output_trait
                #output_export_trait
                #output_export_impl
                #output_valid_trait
            }
        });

    quote! {
        #(#call_traits)*
    }
}
