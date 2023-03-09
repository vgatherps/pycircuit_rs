use std::ops::Add;

use oxidiser_macro::oxidiser_component;

oxidiser_component! {
    Name: Add;

    // The total set of possible inputs
    // TODO add metadata layer
    Inputs: {
        a -> generic A, // A generic type which will be named A
        b -> generic B, // A generic type which will be named B
    };

    // Total set of possible outputs

    // For this either specify an exact type OR a trait to reference to deduce the type
    Outputs: {
        // the graph will essentially do <Self as AdderOutput>::out2
        out -> using AdderOutput,
    };

    Calls: {
        // The name of the function to call
        op: {
            // The inputs to this specific call. If the whole set of inputs is triggered,
            // then this call will be called
            Takes: {
                a,
                b,
            };

            // inputs which are visible to the call but do not impact triggering
            Observes: {};

            // The outputs to this specific call
            Writes: {
                out,
            };
        };
    };
}

pub trait AdderOutput {
    type Out;
}

#[derive(Default)]
pub struct Adder<A, B> {
    a: std::marker::PhantomData<A>,
    b: std::marker::PhantomData<B>,
}

impl<A, B> AdderOutput for Adder<A, B>
where
    // TODO - this *needs* to be made able to work well on non-copy types
    // but as far as I can tell, that requires endless shuffling of lifetimes
    // anywhere and everywhere since we need to implement add for all sorts of reference lifetimes
    // and it just becomes a nightmare
    A: Add<B>,
{
    type Out = <A as Add<B>>::Output;
}

impl<A: Copy, B: Copy> Adder<A, B>
where
    A: Add<B>,
{
    #[inline]
    pub fn _op(
        &self,
        inputs: impl AddOpInput<A = A, B = B>,
        outputs: impl AddOpOutput<Self>,
    ) -> Option<()> {
        let a = inputs.a()?;
        let b = inputs.b()?;

        let rval = *a + *b;
        *outputs.out() = rval;

        Some(())
    }

    #[inline]
    pub fn op(
        &self,
        inputs: impl AddOpInput<A = A, B = B>,
        outputs: impl AddOpOutput<Self>,
    ) -> AddOpOutputValid {
        AddOpOutputValid {
            out: self._op(inputs, outputs).is_some(),
        }
    }
}
