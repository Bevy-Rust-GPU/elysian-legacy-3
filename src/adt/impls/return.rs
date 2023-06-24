use t_funk::{macros::impl_adt, typeclass::monad::Return};

use crate::{Alias, Combine, Run};

impl_adt! {
    impl<A, B, C, T> Return<T> for Run<A> | Alias<A> | Combine<A, B, C> {
        type Return = Run<T>;

        fn r#return(t: T) -> Self::Return {
            Run(t)
        }
    }
}
