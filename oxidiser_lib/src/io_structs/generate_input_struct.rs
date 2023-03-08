// These are trait abstractions instead of struct abstractions
// since traits require FAR less boilerplate to define per component,
// as opposed to defining concrete types and shuffling them around

// 99% any real production use of this is actually a proc macro

#[macro_export]
macro_rules! __generate_input_type_helper {
    ($ty:ident) => {
        $crate::__generate_input_type_helper!(invalid $ty)
    };
    (invalid $ty:ident) => {
         Option<&Self::$ty>
    };
    (valid $ty:ident) => {
        &Self::$ty
    };
    ($other:ident $ty:ident) => {
        std::compile_error!("This macro only accepts valid or invalid as potential input modifiers")
    };
}

#[macro_export]
macro_rules! generate_input_struct {
    ($struct_name:ident [$($type_names:ident),*] $($tokens:tt)*) => {
        $crate::generate_input_struct!(@munch $struct_name @ongoing {} @rest {$($tokens)*});
    };


    // Munchers for the output names that come afterwards

    (
        @munch $struct_name:ident
        @ongoing_types {$($ongoing_types:tt)*}
        @ongoing {$($ongoing:tt)*}
        @rest {$name:ident -> $ty:ident}
    ) => {
        $crate::generate_input_struct!(
            @munch $struct_name
            @ongoing_types {
                $($ongoing_types)*
            }
            @ongoing {
                $($ongoing)*
                fn $name(&self) -> $crate::__generate_input_type_helper!($ty);
            }
            @rest {}
        );
    };

    (
        @munch $struct_name:ident
        @ongoing_types {$($ongoing_types:tt)*}
        @ongoing {$($ongoing:tt)*}
        @rest {$name:ident -> $how:ident $ty:ident}
    ) => {
        $crate::generate_input_struct!(
            @munch $struct_name
            @ongoing_types {
                $($ongoing_types)*
            }
            @ongoing {
                $($ongoing)*
                fn $name(&self) -> $crate::__generate_input_type_helper!($how Self::$name);
            }
            @rest {}
        );
    };

        (
        @munch $struct_name:ident
        @ongoing {$($ongoing:tt)*}
        @rest {$name:ident -> $ty:ident, $($rest:tt)*}
    ) => {
        $crate::generate_input_struct!(
            @munch $struct_name
            @ongoing_types {
                $($ongoing_types)*
            }
            @ongoing {
                $($ongoing)*
                type $name;
                fn $name(&self) -> $crate::__generate_input_type_helper!(Self::$name);
            }
            @rest {$($rest)*}
        );
    };

    (
        @munch $struct_name:ident
        @ongoing {$($ongoing:tt)*}
        @rest {$name:ident -> $how:ident $ty:ident, $($rest:tt)*}
    ) => {
        $crate::generate_input_struct!(
            @munch $struct_name
            @ongoing {
                $($ongoing)*
                fn $name(&self) -> $crate::__generate_input_type_helper!($how Self::$name);
            }
            @rest {$($rest)*}
        );
    };

    // Generate the actual struct
    (
        @munch $struct_name:ident
        @ongoing_types {$($ongoing_types:tt)*}
        @ongoing {$($ongoing:tt)*}
        @rest {}
    ) => {
        concat_idents::concat_idents!(st_name = $struct_name, Input {
            pub trait st_name {
                $($ongoing_types)*
                $($ongoing)*
            }
        });
    };
}

#[allow(dead_code)]
#[cfg(test)]
mod example_compiles {

    generate_input_struct!(Test: a -> u32);

    struct MyTestInput {}

    fn create_input<'a>(a: &'a u32, b: &'a u64) -> impl TestInput<'a> {
        TestInput {
            a: Some(a),
            b,
            c: None,
        }
    }
}
