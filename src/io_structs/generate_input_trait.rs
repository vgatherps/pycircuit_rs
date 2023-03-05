// One could argue that making this a trait abstraction would prevent
// optimisations like only including the inputs pointer and regenerating
// the references on demand. In almost all cases, the root-level calls
// will get inlined making this concern pointless. It might even be harmful to the
// optimizer to always try and reference the struct instead of the stack.
//
// Further, this should be *far* faster to compile than the trait abstraction

#[macro_export]
macro_rules! generate_input_type {
    ($ty:ty) => {
        $crate::generate_input_type!(invalid $ty)
    };
    (invalid $ty:ty) => {
         Option<&'a $ty>
    };
    (valid $ty:ty) => {
        &'a $ty
    };
}

#[macro_export]
macro_rules! generate_input_struct {
    ($struct_name:ident: $($tokens:tt)*) => {
        $crate::generate_input_struct!(@munch $struct_name @gen {} @ongoing {} @rest {$($tokens)*});
    };
    ($struct_name:ident<$($gen:ident),+>: $($tokens:tt)*) => {
        $crate::generate_input_struct!(@munch $struct_name @gen {$($gen),+} @ongoing {} @rest {$($tokens)*});
    };

    (@munch $struct_name:ident @gen {$($gen:ident),*} @ongoing {$($ongoing:tt)*} @rest {}) => {
        pub struct $struct_name <'a, $($gen),*> {
            $($ongoing)*
        }
    };

    (@munch $struct_name:ident @gen {$($gen:ident),*} @ongoing {$($ongoing:tt)*} @rest {$name:ident -> $ty:ty}) => {
        $crate::generate_input_struct!(@munch $struct_name @gen {$($gen),*} @ongoing {
            $($ongoing)*
            pub $name: $crate::generate_input_type!($ty),
        } @rest {});
    };

    (@munch $struct_name:ident @gen {$($gen:ident),*} @ongoing {$($ongoing:tt)*} @rest {$name:ident -> $how:ident $ty:ty}) => {
        $crate::generate_input_struct!(@munch $struct_name @gen {$($gen),*} @ongoing {
            $($ongoing)*
            pub $name: $crate::generate_input_type!($how $ty),
        } @rest {});
    };

        (@munch $struct_name:ident @gen {$($gen:ident),*} @ongoing {$($ongoing:tt)*} @rest {$name:ident -> $ty:ty, $($rest:tt)*}) => {
        $crate::generate_input_struct!(@munch $struct_name @gen {$($gen),*} @ongoing {
            $($ongoing)*
            pub $name: $crate::generate_input_type!($ty),
        } @rest {$($rest)*});
    };

    (@munch $struct_name:ident @gen {$($gen:ident),*} @ongoing {$($ongoing:tt)*} @rest {$name:ident -> $how:ident $ty:ty, $($rest:tt)*}) => {
        $crate::generate_input_struct!(@munch $struct_name @gen {$($gen),*} @ongoing {
            $($ongoing)*
            pub $name: $crate::generate_input_type!($how $ty),
        } @rest {$($rest)*});
    };
}

#[allow(dead_code)]
#[cfg(test)]
mod example_compiles {

    generate_input_struct!(Input: a -> u32, b -> valid u64, c -> invalid f64);

    fn create_input<'a>(a: &'a u32, b: &'a u64) -> Input<'a> {
        Input {
            a: Some(a),
            b,
            c: None,
        }
    }

    generate_input_struct!(GenInput<T, G>: a -> T, b -> valid G, c -> invalid f64);

    fn create_generic_input<'a, T, G>(a: &'a T, b: &'a G) -> GenInput<'a, T, G> {
        GenInput {
            a: Some(a),
            b,
            c: None,
        }
    }
}
