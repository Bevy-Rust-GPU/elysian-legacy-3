use crate::Shape;
use t_funk::typeclass::{
    category::{Compose, ComposeT},
    semigroup::Mappend,
};

impl<A, B> Mappend<B> for Shape<A>
where
    Self: Compose<B>,
{
    type Mappend = ComposeT<Self, B>;

    fn mappend(self, t: B) -> Self::Mappend {
        self.compose(t)
    }
}
