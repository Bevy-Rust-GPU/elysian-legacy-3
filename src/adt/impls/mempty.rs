use t_funk::{macros::impl_adt, typeclass::monoid::Mempty};

use crate::{Combine, Sequence, Unit, Nil};

impl_adt! {
    impl<A, B, C> Mempty for Unit<A> | Sequence<A, B> | Combine<A, B, C> {
        type Mempty = Nil;

        fn mempty() -> Self::Mempty {
            Nil
        }
    }
}
