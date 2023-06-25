use t_funk::{
    macros::impl_adt,
    typeclass::{
        category::{Compose, ComposeT},
        semigroup::Mappend,
    },
};

use crate::{Alias, Combine, Domains, Modify, Run};

impl_adt! {
    impl<A, B, C, T> Mappend<T> for Run<A> | Modify<A> | Domains<A> | Alias<A> | Combine<A, B, C>
    where
        Self: Compose<T>,
    {
        type Mappend = ComposeT<Self, T>;

        fn mappend(self, t: T) -> Self::Mappend {
            self.compose(t)
        }
    }
}
