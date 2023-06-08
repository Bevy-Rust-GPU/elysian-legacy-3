use t_funk::{
    macros::impl_adt,
    typeclass::{
        category::{Compose, ComposeT},
        semigroup::Mappend,
    },
};

use crate::{Combine, Field, Input, Modify, End, Output, Then};

impl_adt! {
    impl<A, B, C, T> Mappend<T> for End | Input<A> | Field<A> | Output<A> | Modify<A> | Then<A, B> | Combine<A, B, C>
    where
        Self: Compose<T>,
    {
        type Mappend = ComposeT<Self, T>;

        fn mappend(self, t: T) -> Self::Mappend {
            self.compose(t)
        }
    }
}
