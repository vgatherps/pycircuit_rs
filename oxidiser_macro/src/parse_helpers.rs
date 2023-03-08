use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Token,
};

pub type COLLECTION_SEP = syn::Token![,];
pub type BODY_SEP = syn::Token![;];
pub type NAME_SEP = syn::Token![:];
pub type DICT_SEP = syn::Token![->];

pub fn parse_separated<K: Parse, S: Parse, V: Parse>(tokens: ParseStream) -> syn::Result<(K, V)> {
    let key = tokens.parse()?;
    tokens.parse::<S>()?;
    let value = tokens.parse()?;
    Ok((key, value))
}

pub fn parse_body_term<T>(
    fnc: impl FnOnce(ParseStream) -> syn::Result<T>,
    tokens: ParseStream,
) -> syn::Result<T> {
    let val = fnc(tokens)?;
    tokens.parse::<BODY_SEP>()?;
    Ok(val)
}

pub fn parse_named<K: Parse, V: Parse>(tokens: ParseStream) -> syn::Result<V> {
    Ok(parse_separated::<K, NAME_SEP, V>(tokens)?.1)
}

pub fn parse_vec_of<T, P>(tokens: ParseStream) -> syn::Result<Vec<T>>
where
    T: Parse,
    P: Parse,
{
    let content;
    braced!(content in tokens);

    let punctuated: Punctuated<_, COLLECTION_SEP> = content.parse_terminated(T::parse)?;
    Ok(punctuated.into_iter().collect())
}

pub fn parse_set_of<T, P>(tokens: ParseStream) -> syn::Result<HashSet<T>>
where
    T: Parse + Hash + Eq + Debug,
    P: Parse,
{
    let content;
    braced!(content in tokens);

    let punctuated: Punctuated<_, COLLECTION_SEP> = content.parse_terminated(T::parse)?;
    let as_vec: Vec<_> = punctuated.into_iter().collect();

    let mut set = HashSet::new();

    for v in &as_vec {
        if !set.insert(v) {
            return Err(syn::Error::new(
                content.span(),
                format!("Duplicate entry in set of inputs: {:?}", v),
            ));
        }
    }

    Ok(as_vec.into_iter().collect())
}

struct DictEntry<K, V> {
    key: K,
    value: V,
}

impl<K: Parse, V: Parse> Parse for DictEntry<K, V> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (key, value) = parse_separated::<K, DICT_SEP, V>(input)?;
        Ok(DictEntry { key, value })
    }
}

pub fn parse_dict_of<K, V, P>(tokens: ParseStream) -> syn::Result<HashMap<K, V>>
where
    K: Parse + Hash + Eq + Debug,
    V: Parse,
    P: Parse,
{
    let content;
    braced!(content in tokens);

    let punctuated: Punctuated<_, P> = content.parse_terminated(DictEntry::<K, V>::parse)?;
    let as_vec: Vec<_> = punctuated.into_iter().collect();

    let mut set = HashSet::new();

    for kv in &as_vec {
        if !set.insert(&kv.key) {
            return Err(syn::Error::new(
                content.span(),
                format!("Duplicate key in set of inputs: {:?}", &kv.key),
            ));
        }
    }

    Ok(as_vec.into_iter().map(|kv| (kv.key, kv.value)).collect())
}

pub struct ParseVecOf<T> {
    pub set: Vec<T>,
}

impl<T: Parse> Parse for ParseVecOf<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ParseVecOf {
            set: parse_vec_of::<_, COLLECTION_SEP>(input)?,
        })
    }
}

pub struct ParseSetOf<T> {
    pub set: HashSet<T>,
}

impl<T: Parse + Hash + Eq + Debug> Parse for ParseSetOf<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ParseSetOf {
            set: parse_set_of::<_, COLLECTION_SEP>(input)?,
        })
    }
}

pub struct ParseMapOf<K, V> {
    pub map: HashMap<K, V>,
}

impl<K: Parse + Hash + Eq + Debug, V: Parse> Parse for ParseMapOf<K, V> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ParseMapOf {
            map: parse_dict_of::<_, _, COLLECTION_SEP>(input)?,
        })
    }
}
