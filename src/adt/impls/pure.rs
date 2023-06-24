use t_funk::{macros::impl_adt, typeclass::applicative::Pure};

use crate::{Alias, Combine, Run};

impl_adt! {
    impl<A, B, C, T> Pure<T> for Run<A> | Alias<A> | Combine<A, B, C> {
        type Pure = Run<T>;

        fn pure(t: T) -> Self::Pure {
            Run(t)
        }
    }
}
