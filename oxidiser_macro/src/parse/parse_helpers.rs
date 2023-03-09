use std::fmt::Debug;
use std::hash::Hash;
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use proc_macro2::Ident;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub type CollectionSep = syn::Token![,];
pub type BodySep = syn::Token![;];
pub type NameSep = syn::Token![:];
pub type DictSep = syn::Token![->];

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
    tokens.parse::<BodySep>()?;
    Ok(val)
}

pub fn parse_named<K: Parse, V: Parse>(tokens: ParseStream) -> syn::Result<V> {
    Ok(parse_separated::<K, NameSep, V>(tokens)?.1)
}

pub fn parse_vec_of<T>(tokens: ParseStream) -> syn::Result<Vec<T>>
where
    T: Parse,
{
    let content;
    braced!(content in tokens);

    let punctuated: Punctuated<_, CollectionSep> = content.parse_terminated(T::parse)?;
    Ok(punctuated.into_iter().collect())
}

pub fn parse_set_of<T>(tokens: ParseStream) -> syn::Result<HashSet<T>>
where
    T: Parse + Hash + Eq + Debug,
{
    let content;
    braced!(content in tokens);

    let punctuated: Punctuated<_, CollectionSep> = content.parse_terminated(T::parse)?;
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

struct DictEntry<K, V, P> {
    key: K,
    value: V,
    _dummy: PhantomData<P>,
}

impl<K: Parse, V: Parse, P: Parse> Parse for DictEntry<K, V, P> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (key, value) = parse_separated::<K, P, V>(input)?;
        Ok(DictEntry {
            key,
            value,
            _dummy: PhantomData,
        })
    }
}

pub fn parse_dict_of<K, V, P, N>(tokens: ParseStream) -> syn::Result<HashMap<K, V>>
where
    K: Parse + Hash + Eq + Debug,
    V: Parse,
    P: Parse,
    N: Parse,
{
    let content;
    braced!(content in tokens);

    let punctuated: Punctuated<_, P> = content.parse_terminated(DictEntry::<K, V, N>::parse)?;
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
            set: parse_vec_of::<_>(input)?,
        })
    }
}

pub struct ParseSetOf<T> {
    pub set: HashSet<T>,
}

impl<T: Parse + Hash + Eq + Debug> Parse for ParseSetOf<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ParseSetOf {
            set: parse_set_of::<_>(input)?,
        })
    }
}

pub struct ParseMapOf<K, V> {
    pub map: HashMap<K, V>,
}

impl<K: Parse + Hash + Eq + Debug, V: Parse> Parse for ParseMapOf<K, V> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ParseMapOf {
            map: parse_dict_of::<_, _, CollectionSep, DictSep>(input)?,
        })
    }
}

pub struct ParseNamed<V> {
    pub map: HashMap<Ident, V>,
}

impl<V: Parse> Parse for ParseNamed<V> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(ParseNamed {
            map: parse_dict_of::<_, _, BodySep, NameSep>(input)?,
        })
    }
}
