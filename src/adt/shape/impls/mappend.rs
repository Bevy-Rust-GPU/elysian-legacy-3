use crate::{Field, Input, Output, ShapeEnd};
use t_funk::{
    macros::impl_adt,
    typeclass::{
        category::{Compose, ComposeT},
        semigroup::Mappend,
    },
};

impl_adt! {
    impl<A, B, C> Mappend<C> for Input<A, B> | Field<A, B> | Output<A, B> | ShapeEnd
    where
        Self: Compose<C>
    {
        type Mappend = ComposeT<Self, C>;

        fn mappend(self, t: C) -> Self::Mappend {
            self.compose(t)
        }
    }
}
