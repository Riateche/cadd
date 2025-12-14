//! Extension traits for checked operations.

use crate::ops::{cadd, doc_text, impl_fn_literal, wrapper_doc_with_link};

#[rustfmt::skip]
macro_rules! declare_func {
    (
        self_type = $self_type:ty,
        fn_name = $fn_name:ident, call_fn = $call_fn:ident, impl_fn = $impl_fn:literal, other_type = $other_type:ty,
        out_type = $out_type:ty,
    ) => {
        #[doc = concat!(
            doc_text!($fn_name),
            "\n\nWrapper for [`",
            stringify!($self_type),
            "::",
            $impl_fn,
            "`]."
        )]
        fn $fn_name(self, other: $other_type) -> $crate::Result<$out_type>;
    };
}

#[rustfmt::skip]
macro_rules! impl_func {
    (
        self_type = $self_type:ty,
        fn_name = $fn_name:ident, call_fn = $call_fn:ident, impl_fn = $impl_fn:literal, other_type = $other_type:ty,
        out_type = $out_type:ty,
    ) => {
        #[doc = concat!(
            doc_text!($fn_name),
            "\n\nWrapper for [`",
            stringify!($self_type),
            "::",
            $impl_fn,
            "`]."
        )]
        fn $fn_name(self, other: $other_type) -> $crate::Result<$out_type> {
            $call_fn(self, other)
        }
    };
}

macro_rules! declare_extension_trait {
    ($trait_:ident, $type_:ty, $doc:literal, $(($($func:tt)+),)+) => {
        #[doc = $doc]
        pub trait $trait_: Sized {
            $(
                declare_func!($($func)+);
            )+
        }

        impl $trait_ for $type_ {
            $(
                impl_func!($($func)+);
            )+
        }
    };
}

declare_extension_trait!(
    U8Ext,
    u8,
    "Enhanced checked arithmetics functions for `u8`.",
    (
        self_type = u8,
        fn_name = cadd,
        call_fn = cadd,
        impl_fn = "checked_add",
        other_type = u8,
        out_type = u8,
    ),
);
