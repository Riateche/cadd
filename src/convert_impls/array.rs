use {
    crate::{convert::Cfrom, convert_impls::LimitedSliceDebug},
    alloc::{boxed::Box, rc::Rc, sync::Arc, vec::Vec},
    core::fmt::Debug,
};

impl<'a, T: Debug, const N: usize> Cfrom<&'a [T]> for &'a [T; N] {
    type Error = crate::Error;

    #[inline]
    fn cfrom(from: &'a [T]) -> Result<Self, Self::Error> {
        from.try_into()
            .map_err(|_err| slice_to_array_error(N, from))
    }
}

impl<'a, T: Debug, const N: usize> Cfrom<&'a mut [T]> for &'a mut [T; N] {
    type Error = crate::Error;

    #[inline]
    fn cfrom(from: &'a mut [T]) -> Result<Self, Self::Error> {
        // We have to do it with an extra check because of borrow checker.
        if from.len() == N {
            #[expect(
                clippy::unwrap_used,
                clippy::unwrap_in_result,
                reason = "always succeeds after length check"
            )]
            Ok(from.try_into().unwrap())
        } else {
            Err(slice_to_array_error(N, from))
        }
    }
}

impl<'a, T: Copy + Debug, const N: usize> Cfrom<&'a [T]> for [T; N] {
    type Error = crate::Error;

    #[inline]
    fn cfrom(from: &'a [T]) -> Result<Self, Self::Error> {
        from.try_into()
            .map_err(|_err| slice_to_array_error(N, from))
    }
}

impl<'a, T: Copy + Debug, const N: usize> Cfrom<&'a mut [T]> for [T; N] {
    type Error = crate::Error;

    #[inline]
    fn cfrom(from: &'a mut [T]) -> Result<Self, Self::Error> {
        from.try_into()
            .map_err(|_err| slice_to_array_error(N, from))
    }
}

macro_rules! impl_cfrom_owned_to_array {
    ($(($from:ty, $to:ty),)*) => {
        $(
            impl<T: Debug, const N: usize> Cfrom<$from> for $to {
                type Error = crate::Error;

                #[inline]
                fn cfrom(from: $from) -> Result<Self, Self::Error> {
                    if from.len() == N {
                        Ok(from.try_into().unwrap())
                    } else {
                        Err(slice_to_array_error(N, &from))
                    }
                }
            }
        )*
    };
}

fn slice_to_array_error<T: Debug>(target_len: usize, value: &[T]) -> crate::Error {
    crate::Error::new(alloc::format!(
        "expected {} items, got {:?}",
        target_len,
        LimitedSliceDebug {
            data: value,
            items_text: "items"
        },
    ))
}

impl_cfrom_owned_to_array!(
    (Rc<[T]>, Rc<[T; N]>),
    (Arc<[T]>, Arc<[T; N]>),
    (Vec<T>, [T; N]),
    (Box<[T]>, Box<[T; N]>),
    (Vec<T>, Box<[T; N]>),
);
