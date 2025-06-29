use {
    crate::convert::Cfrom,
    alloc::boxed::Box,
    alloc::vec::Vec,
    core::fmt::Debug,
    std::{rc::Rc, sync::Arc},
};

struct SliceLimitedDebug<'a, T>(&'a [T]);

impl<'a, T: Debug> Debug for SliceLimitedDebug<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const MAX_ITEMS: usize = 32;
        if self.0.len() > MAX_ITEMS {
            let mut list = f.debug_list();
            for item in &self.0[0..MAX_ITEMS / 2] {
                list.entry(item);
            }
            // TODO: avoid quotes in "..."
            list.entry(&"...");
            for item in &self.0[self.0.len() - MAX_ITEMS / 2..] {
                list.entry(item);
            }
            list.finish()
        } else {
            write!(f, "{:?}", self.0)
        }
    }
}

impl<'a, T: Debug, const N: usize> Cfrom<&'a [T]> for &'a [T; N] {
    type Error = crate::Error;

    fn cfrom(from: &'a [T]) -> Result<Self, Self::Error> {
        from.try_into().map_err(|_| slice_to_array_error(N, from))
    }
}

impl<'a, T: Debug, const N: usize> Cfrom<&'a mut [T]> for &'a mut [T; N] {
    type Error = crate::Error;

    fn cfrom(from: &'a mut [T]) -> Result<Self, Self::Error> {
        // We have to do it with an extra check because of borrow checker.
        if from.len() == N {
            Ok(from.try_into().unwrap())
        } else {
            Err(slice_to_array_error(N, from))
        }
    }
}

impl<'a, T: Copy + Debug, const N: usize> Cfrom<&'a [T]> for [T; N] {
    type Error = crate::Error;

    fn cfrom(from: &'a [T]) -> Result<Self, Self::Error> {
        from.try_into().map_err(|_| slice_to_array_error(N, from))
    }
}

impl<'a, T: Copy + Debug, const N: usize> Cfrom<&'a mut [T]> for [T; N] {
    type Error = crate::Error;

    fn cfrom(from: &'a mut [T]) -> Result<Self, Self::Error> {
        from.try_into().map_err(|_| slice_to_array_error(N, from))
    }
}

macro_rules! impl_cfrom_owned_to_array {
    ($(($from:ty, $to:ty),)*) => {
        $(
            impl<T: Debug, const N: usize> Cfrom<$from> for $to {
                type Error = crate::Error;

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
        "expected slice of length {}, got length {}: {:?}",
        target_len,
        value.len(),
        SliceLimitedDebug(value),
    ))
}

impl_cfrom_owned_to_array!(
    (Rc<[T]>, Rc<[T; N]>),
    (Arc<[T]>, Arc<[T; N]>),
    (Vec<T>, [T; N]),
    (Box<[T]>, Box<[T; N]>),
    (Vec<T>, Box<[T; N]>),
);
