use heck::AsPascalCase;
use proc_macro2::{Ident, Span};

pub fn pascal_ident_span(as_strs: &[&dyn ToString], span: Span) -> Ident {
    let mut ident = String::new();
    for s in as_strs {
        let camel = AsPascalCase(s.to_string()).to_string();
        ident.push_str(&camel);
    }

    Ident::new(&ident, span)
}
