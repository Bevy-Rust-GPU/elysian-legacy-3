use t_funk::{macros::impl_adt, typeclass::applicative::Pure};

use crate::{Combine, Field, Input, Modify, End, Output, Then};

impl_adt! {
    impl<A, B, C, T> Pure<T> for End | Input<A> | Field<A> | Output<A> | Modify<A> | Then<A, B> | Combine<A, B, C> {
        type Pure = Modify<T>;

        fn pure(t: T) -> Self::Pure {
            Modify(t)
        }
    }
}
