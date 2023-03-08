use std::ops::Add;

use crate::{generate_input_trait, generate_output_struct};

// Using the module gives us namespacing/impl-associated-types like capabilities
// This is still grosser than the C++ version but not as bad as trying to use traits
// It does tie down how many implementations you can have per-file though?

// This api is just really gross to specify when generic over lifetimes
// is there some better way? higher rank lifetimes?
// It onle really matter here since we're trying to
// be generic over reference addition, not generic over raw type addition
// and rust forces us to attach lifetimes to each trait impl
//
// The reason for even using these in the first place is to avoid having to repeatedly and constantly
// specify generic types for the input/output structs. This is somewhere that C++ really excels,
// as you just add a concept and attach some structs/types to the class and you're done,
// whereas with rust, 90% of the code is trait/lifetime ceremony

generate_input_trait!(Adder<A, B>: a -> A, b -> B);
generate_output_struct!(Adder<O>: out -> O);

#[derive(Default)]
pub struct Adder<A, B> {
    _a: std::marker::PhantomData<A>,
    _b: std::marker::PhantomData<B>,
}

pub trait AdderOutputs {
    type Output<'a>;
}

pub trait AdderAddIO {
    type Input<'a>;
    type Output<'a>;
}

impl<A: 'static, B: 'static> AdderOutputs for Adder<A, B>
where
    for<'a> &'a A: Add<&'a B>,
{
    type Output<'a> = <&'a A as Add<&'a B>>::Output;
}

impl<A: 'static, B: 'static> AdderAddIO for Adder<A, B>
where
    Adder<A, B>: AdderOutputs,
{
    type Input<'a> = AdderInput<'a, A, B>;
    type Output<'a> = AdderOutput<'a, <Adder<A, B> as AdderOutputs>::Output<'a>>;
}

pub type AddOutputStruct<'a, A, B> = AdderOutput<'a, <Adder<A, B> as AdderOutputs>::Output<'a>>;
pub type AddInputStruct<'a, A, B> = AdderInput<'a, A, B>;

impl<A: 'static, B: 'static> Adder<A, B>
where
    for<'a> &'a A: Add<&'a B>,
{
    #[inline]
    pub fn _add<'a>(
        &self,
        inputs: AddInputStruct<'a, A, B>,
        outputs: AddOutputStruct<'a, A, B>,
    ) -> Option<()> {
        *outputs.out = inputs.a? + inputs.b?;
        Some(())
    }

    #[inline]
    pub fn add<'a>(
        &self,
        inputs: AddInputStruct<'a, A, B>,
        outputs: AddOutputStruct<'a, A, B>,
    ) -> AdderOutputValid {
        AdderOutputValid {
            out: self._add(inputs, outputs).is_some(),
        }
    }
}
