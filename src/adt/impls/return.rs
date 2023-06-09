use t_funk::{macros::impl_adt, typeclass::monad::Return};

use crate::{AdtEnd, Combine, Run, Then};

impl_adt! {
    impl<A, B, C, T> Return<T> for AdtEnd | Run<A> | Then<A, B> | Combine<A, B, C> {
        type Return = Run<T>;

        fn r#return(t: T) -> Self::Return {
            Run(t)
        }
    }
}
