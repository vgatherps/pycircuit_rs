use std::ops::Add;

use crate::{generate_input_struct, generate_output_struct};

generate_input_struct!(Adder<A, B>: a -> A, b -> B);
generate_output_struct!(Adder<O>: out -> O);

// This is supremely gross in my opinion...
// Huge amount of boilerplate and lifetime games
// The copy constraint is *really* bad, but without this constraint,
// everything turns into a horrifying mess of passing lifetimes around
// The C++ version uses concepts which are weird and opaque, but otherwise,
// it Just Works without being 90% boilerplate. We've already put some of the worst
// into macros

// codegen might have to resort to unsafe or something, this is unacceptable
// as is and the copy constraint is another level of unacceptability
pub trait AdderTypes {
    type Output;
    type OutputStruct<'a>;
    type InputStruct<'a>;
}

impl<A: 'static, B: 'static> AdderTypes for Adder<A, B>
where
    A: Add<B>,
    <A as Add<B>>::Output: 'static,
{
    type Output = <A as Add<B>>::Output;
    type OutputStruct<'a> = AdderOutput<'a, Self::Output>;
    type InputStruct<'a> = AdderInput<'a, A, B>;
}

#[derive(Default)]
pub struct Adder<A, B> {
    _a: std::marker::PhantomData<A>,
    _b: std::marker::PhantomData<B>,
}

impl<A: 'static + Copy, B: 'static + Copy> Adder<A, B>
where
    A: Add<B>,
{
    #[inline]
    pub fn _add(
        &self,
        inputs: AdderInput<A, B>,
        outputs: <Adder<A, B> as AdderTypes>::OutputStruct<'_>,
    ) -> Option<()> {
        *outputs.out = *inputs.a? + *inputs.b?;
        Some(())
    }

    #[inline]
    pub fn add(
        &self,
        inputs: AdderInput<A, B>,
        outputs: <Adder<A, B> as AdderTypes>::OutputStruct<'_>,
    ) -> AdderOutputValid {
        AdderOutputValid {
            out: self._add(inputs, outputs).is_some(),
        }
    }
}
