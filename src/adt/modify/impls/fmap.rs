use t_funk::{
    closure::{Closure, OutputT},
    typeclass::functor::Fmap,
};

use crate::Modify;

impl<T, F> Fmap<F> for Modify<T>
where
    F: Closure<T>,
{
    type Fmap = Modify<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Modify(f.call(self.0))
    }
}

