/*!

Parser for the oxidiser macro language.

The macros here take a component specification and generate all the boilerplate
that makes it easy for codegen to communicate with the component.

Initially, this will just focus on the boilerplate, and not generating components themselves
This will also generate code to dump a json definition, which can be used to
easily generate the component definition for oxidiser from the

A definition might look like:

oxidiser_component! {
    Name: Adder;

    // The total set of possible inputs
    // TODO add metadata layer
    Inputs: {
        a -> generic A, // A generic type which will be named A
        b -> generic B, // A generic type which will be named B
        c -> u32,
        c -> generic A, // Same type as A
    };

    // Total set of possible outputs

    // For this either specify an exact type OR a trait to reference to deduce the type
    Outputs: {
        out1 -> u32,
        // the graph will essentially do <Self as AdderOutput>::out2
        out2 -> using Trait,
    };

    Calls: {
        // The name of the function to call
        add: {
            // The inputs to this specific call. If the whole set of inputs is triggered,
            // then this call will be called
            Takes {
                a,
                b,
            };

            // inputs which are visible to the call but do not impact triggering
            Observes {};

            // The outputs to this specific call
            Writer {
                out,
            };
        };
    }
}
*/

use bodies::{AllCallsBody, InputBody, OutputBody};
use kws::{Calls, Inputs, Name, Outputs};
use parse_helpers::{parse_body_term, parse_named};
use proc_macro2::Ident;
use syn::parse::ParseStream;

mod bodies;
mod kws;
mod parse_helpers;
mod types;

fn parse_stream(input: ParseStream) -> syn::Result<()> {
    let name = parse_body_term(parse_named::<Name, Ident>, input)?;

    let inputs = parse_body_term(parse_named::<Inputs, InputBody>, input)?;
    let outputs = parse_body_term(parse_named::<Outputs, OutputBody>, input)?;
    let all_calls = parse_body_term(parse_named::<Calls, AllCallsBody>, input)?;

    Ok(())
}
