use t_funk::typeclass::{
    category::{Compose, ComposeT},
    semigroup::Mappend,
};

use crate::Modify;

impl<T, U> Mappend<U> for Modify<T>
where
    Self: Compose<U>,
{
    type Mappend = ComposeT<Self, U>;

    fn mappend(self, t: U) -> Self::Mappend {
        self.compose(t)
    }
}

