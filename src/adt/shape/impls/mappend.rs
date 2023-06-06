use t_funk::{
    macros::impl_adt,
    typeclass::{
        category::{Compose, ComposeT},
        semigroup::Mappend,
    },
};

use crate::{Field, Input, Nil, Output};

impl_adt! {
    impl<A, B, T> Mappend<T> for Input<A, B> | Field<A, B> | Output<A, B>
    where
        Self: Compose<T>,
    {
        type Mappend = ComposeT<Self, T>;

        fn mappend(self, t: T) -> Self::Mappend {
            self.compose(t)
        }
    }
}

impl<U> Mappend<U> for Nil {
    type Mappend = ComposeT<Self, U>;

    fn mappend(self, t: U) -> Self::Mappend {
        self.compose(t)
    }
}
