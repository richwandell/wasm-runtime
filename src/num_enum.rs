#[macro_export]
macro_rules! num_enum {
    ($name:ident $type:ident [$($keys:tt)*] $impl:ident) => {
        impl From<$type> for $name {
            fn from(v: $type) -> Self {
                match v {
                    $(x if x == $name::$keys as $type => $name::$keys),*,
                    _ => panic!("Cannot convert {}", v)
                }
            }
        }
    };

    ($name:ident [$($keys:tt)*]) => {
        num_enum! {$name u8 [$($keys)*] IMPL}
        num_enum! {$name u16 [$($keys)*] IMPL}
        num_enum! {$name u32 [$($keys)*] IMPL}
        num_enum! {$name u64 [$($keys)*] IMPL}
        num_enum! {$name u128 [$($keys)*] IMPL}
        num_enum! {$name i8 [$($keys)*] IMPL}
        num_enum! {$name i16 [$($keys)*] IMPL}
        num_enum! {$name i32 [$($keys)*] IMPL}
        num_enum! {$name i64 [$($keys)*] IMPL}
        num_enum! {$name i128 [$($keys)*] IMPL}
        num_enum! {$name isize [$($keys)*] IMPL}
        num_enum! {$name usize [$($keys)*] IMPL}
    };

    ($name:ident {$($keys:tt=$vals:tt),*} ) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        #[repr(u8)]
        pub(crate) enum $name {
            $($keys = $vals),*
        }
        num_enum! {$name [$($keys)*]}
    };

    ($name:ident {$($keys:tt),*} ) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        #[repr(u8)]
        pub(crate) enum $name {
            $($keys),*
        }
        num_enum! {$name [$($keys)*]}
    };
}

#[test]
fn test_num_enum() {
    num_enum! {NewEnum {A, B}}
    num_enum! {MyEnum {A=1, B=2}}

    assert_eq!(NewEnum::A as u8, 0);
    assert_eq!(NewEnum::B as u8, 1);
    assert_eq!(NewEnum::from(0), NewEnum::A);
    assert_eq!(NewEnum::from(1), NewEnum::B);
    assert_eq!(MyEnum::from(1), MyEnum::A);
    assert_eq!(MyEnum::from(2), MyEnum::B);
}
