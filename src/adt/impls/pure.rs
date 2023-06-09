use t_funk::{macros::impl_adt, typeclass::applicative::Pure};

use crate::{AdtEnd, Combine, Run, Then};

impl_adt! {
    impl<A, B, C, T> Pure<T> for AdtEnd | Run<A> | Then<A, B> | Combine<A, B, C> {
        type Pure = Run<T>;

        fn pure(t: T) -> Self::Pure {
            Run(t)
        }
    }
}
