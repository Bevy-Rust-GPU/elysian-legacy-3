use t_funk::{function::Id, macros::impl_adt, typeclass::monoid::Mempty};

use crate::{Combine, Modify, Sequence, Shape};

impl_adt! {
    impl<A, B, C> Mempty for Shape<A> | Modify<A> | Sequence<A, B> | Combine<A, B, C> {
        type Mempty = Modify<Id>;

        fn mempty() -> Self::Mempty {
            Modify(Id)
        }
    }
}
