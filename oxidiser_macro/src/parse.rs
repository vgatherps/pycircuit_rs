use bodies::{AllCallsBody, InputBody, OutputBody};
use kws::{Calls, Inputs, Name, Outputs};
use parse_helpers::{parse_body_term, parse_named};
use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};

pub mod bodies;
pub mod kws;
pub mod parse_helpers;
pub mod types;

pub struct ComponentDefinition {
    pub name: Ident,
    pub inputs: InputBody,
    pub outputs: OutputBody,
    pub all_calls: AllCallsBody,
}

impl Parse for ComponentDefinition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = parse_body_term(parse_named::<Name, _>, input)?;

        let inputs = parse_body_term(parse_named::<Inputs, _>, input)?;
        let outputs = parse_body_term(parse_named::<Outputs, _>, input)?;
        let all_calls = parse_body_term(parse_named::<Calls, _>, input)?;

        let def = ComponentDefinition {
            name,
            inputs,
            outputs,
            all_calls,
        };

        def.validate()?;

        Ok(def)
    }
}

impl ComponentDefinition {
    fn validate(&self) -> syn::Result<()> {
        self.validate_call_inputs()?;
        self.validate_call_outputs()?;
        Ok(())
    }
    fn validate_call_inputs(&self) -> syn::Result<()> {
        // Ensure that for every single  call, the set of components matches the set of inputs

        for (call_name, call) in &self.all_calls.calls.map {
            let first_bad_input = call
                .takes
                .set
                .iter()
                .chain(call.observes.set.iter())
                .find(|ident| !self.inputs.inputs.map.contains_key(ident));

            if let Some(bad) = first_bad_input {
                return Err(syn::Error::new(
                    bad.span(),
                    format!("Call {call_name} requested nonexistent input {bad}"),
                ));
            }
        }

        Ok(())
    }
    fn validate_call_outputs(&self) -> syn::Result<()> {
        for (call_name, call) in &self.all_calls.calls.map {
            let first_bad_ident = call
                .writes
                .set
                .iter()
                .find(|ident| !self.outputs.outputs.map.contains_key(ident));

            if let Some(bad) = first_bad_ident {
                return Err(syn::Error::new(
                    bad.span(),
                    format!("Call {call_name} requested nonexistent output {bad}"),
                ));
            }
        }

        Ok(())
    }
}
