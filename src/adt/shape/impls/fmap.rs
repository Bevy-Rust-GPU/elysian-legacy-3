use t_funk::{
    closure::{Closure, OutputT},
    typeclass::functor::Fmap,
};

use crate::Shape;

impl<A, F> Fmap<F> for Shape<A>
where
    F: Clone + Closure<A>,
{
    type Fmap = Shape<OutputT<F, A>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Shape(f.clone().call(self.0))
    }
}
