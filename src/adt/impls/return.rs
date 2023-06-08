use t_funk::{macros::impl_adt, typeclass::monad::Return};

use crate::{Combine, Field, Input, Modify, End, Output, Then};

impl_adt! {
    impl<A, B, C, T> Return<T> for End | Input<A> | Field<A> | Output<A> | Modify<A> | Then<A, B> | Combine<A, B, C> {
        type Return = Modify<T>;

        fn r#return(t: T) -> Self::Return {
            Modify(t)
        }
    }
}

