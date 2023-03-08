// Helper to get around concat_ident issues
#[macro_export]
macro_rules! __output_helper_gen_struct_from {
    ($struct_name:ident, $($tokens:tt)*) => {
        pub struct $struct_name {
            $($tokens)*
        }
    };
    ($struct_name:ident, {$($gen:tt)*}, $($tokens:tt)*) => {
        pub struct $struct_name <'a, $($gen)*> {
            $($tokens)*
        }
    };
}

#[macro_export]
macro_rules! generate_output_struct {
    ($struct_name:ident : $($tokens:tt)*) => {
        $crate::generate_output_struct!(@munch $struct_name @gen {} @ongoing {} @ongoing_bool {} @rest {$($tokens)*});
    };
    ($struct_name:ident<$($gen:ident),+>: $($tokens:tt)*) => {
        $crate::generate_output_struct!(@munch $struct_name @gen {$($gen),+} @ongoing {} @ongoing_bool {} @rest {$($tokens)*});
    };

    (
        @munch $struct_name:ident
        @gen {$($gen:ident),*}
        @ongoing {$($ongoing:tt)*}
        @ongoing_bool {$($ongoing_bool:tt)*}
        @rest {}
    ) => {
        concat_idents::concat_idents!(st_name = $struct_name, Output {
            pub struct st_name <'a, $($gen),*> {
                $($ongoing)*
            }
        });
        concat_idents::concat_idents!(st_name = $struct_name, OutputValid {
            #[derive(Default)]
            pub struct st_name {
                $($ongoing_bool)*
            }
        });

    };

    (
        @munch $struct_name:ident
        @gen {$($gen:ident),*}
        @ongoing {$($ongoing:tt)*}
        @ongoing_bool {$($ongoing_bool:tt)*}
        @rest {$name:ident -> $ty:ty}
    ) => {
        $crate::generate_output_struct!(
            @munch $struct_name
            @gen {$($gen),*}
            @ongoing {
                $($ongoing)*
                pub $name: &'a mut $ty,
            }
            @ongoing_bool {
                $($ongoing_bool)*
                pub $name: bool,
            }
            @rest {}
        );
    };

    (@munch $struct_name:ident
        @gen {$($gen:ident),*}
        @ongoing {$($ongoing:tt)*}
        @ongoing_bool {$($ongoing_bool:tt)*}
        @rest {$name:ident -> valid $ty:ty}
    ) => {
        $crate::generate_output_struct!(
            @munch $struct_name
            @gen {$($gen),*}
            @ongoing {
                $($ongoing)*
                pub $name: &'a mut $ty,
            }
            @ongoing_bool {
                $($ongoing_bool)*
            }
            @rest {}
        );
    };

    (
        @munch $struct_name:ident
        @gen {$($gen:ident),*}
        @ongoing {$($ongoing:tt)*}
        @ongoing_bool {$($ongoing_bool:tt)*}
        @rest {$name:ident -> $ty:ty, $($rest:tt)*}
    ) => {
        $crate::generate_output_struct!(
            @munch $struct_name
            @gen {$($gen),*}
            @ongoing {
                $($ongoing)*
                pub $name: &'a mut $ty,
            }
            @ongoing_bool {
                $($ongoing_bool)*
                pub $name: bool,
            }
            @rest {$($rest)*}
    );
    };

    (
        @munch $struct_name:ident
        @gen {$($gen:ident),*}
        @ongoing {$($ongoing:tt)*}
        @ongoing_bool {$($ongoing_bool:tt)*}
        @rest {$name:ident -> valid $ty:ty, $($rest:tt)*}
    ) => {
        $crate::generate_output_struct!(
            @munch $struct_name
            @gen {$($gen),*}
            @ongoing {
                $($ongoing)*
                pub $name: $crate::generate_input_type!($how $ty),
            }
            @ongoing_bool {
                $($ongoing_bool)*
            }
            @rest {$($rest)*});
    };
}

#[allow(dead_code)]
#[cfg(test)]
mod test_compiles {
    generate_output_struct!(Test: a -> u32, b -> valid u64);
    fn create_output<'a>(a: &'a mut u32, b: &'a mut u64) -> TestOutput<'a> {
        TestOutput { a, b }
    }

    fn create_valid() -> TestOutputValid {
        TestOutputValid { a: true }
    }

    #[test]
    fn test_default_valid<'a>() {
        let t = TestOutputValid::default();
        assert!(!t.a);
    }

    generate_output_struct!(GenTest<T, G>: a -> T, b -> valid G);
    fn create_output_gen<'a, T, G>(a: &'a mut T, b: &'a mut G) -> GenTestOutput<'a, T, G> {
        GenTestOutput { a, b }
    }

    fn create_gen_valid() -> GenTestOutputValid {
        GenTestOutputValid { a: true }
    }

    #[test]
    fn test_gen_default_valid<'a>() {
        let t = GenTestOutputValid::default();
        assert!(!t.a);
    }
}
