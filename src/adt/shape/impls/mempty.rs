use t_funk::{macros::impl_adt, typeclass::monoid::Mempty};

use crate::{Field, Input, Output, ShapeEnd};

impl_adt! {
    impl<A, B> Mempty for Input<A, B> | Field<A, B> | Output<A, B> | ShapeEnd {
        type Mempty = ShapeEnd;

        fn mempty() -> Self::Mempty {
            ShapeEnd
        }
    }
}
