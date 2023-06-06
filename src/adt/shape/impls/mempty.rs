use t_funk::{macros::impl_adt, typeclass::monoid::Mempty};

use crate::{Field, Input, Nil, Output};

impl_adt! {
    impl<A, B> Mempty for Input<A, B> | Field<A, B> | Output<A, B> | Nil {
        type Mempty = Nil;

        fn mempty() -> Self::Mempty {
            Nil
        }
    }
}
