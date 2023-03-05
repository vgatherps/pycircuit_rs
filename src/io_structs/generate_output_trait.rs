#[macro_export]
macro_rules! generate_input_type {
    ($name:ident -> $ty:ty) => {
        $crate::generate_input_type!($name -> invalid $ty);
    };
    ($name:ident -> invalid $ty:ty) => {
        pub $name: Option<&$ty>;
    };
    ($name:ident -> valid $ty:ty) => {
        pub $name: &$ty;
    };
}

#[macro_export]
macro_rules! generate_output_struct {
    ($trait_name:ident: $($name: ident -> $type: ident)*) => {
        pub struct $trait_name {
            $crate::slurp_input_type!($($tokens)*),
        }
    };
    ($trait_name:ident<$($gen:ident)>: $($name: ident -> $type: ident)*) => {
        pub struct $trait_name<$($gen)> {
            $crate::slurp_input_type!($($tokens)*),
        }
    };
}

#[allow(dead_code)]
#[cfg(test)]
mod example_compiles {

    generate_input_struct!(Input: a -> u32, b -> valid u64, c -> invalid f64);

    fn create_input(a: &u32, b: &u64, c: &f64) -> Input {
        Input {
            a: Some(a),
            b: b,
            c: None,
        }
    }
}
