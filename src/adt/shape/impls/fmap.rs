use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::functor::{Fmap, FmapT},
};

use crate::{Field, Input, Output, ShapeEnd};

impl_adt! {
    impl<A, B, F> Fmap<F> for Input<A, B> | Field<A, B> | Output<A, B>
    where
        F: Clone + Closure<A>,
        B: Fmap<F>
    {
        type Fmap = This<OutputT<F, A>, FmapT<B, F>>;

        fn fmap(self, f: F) -> Self::Fmap {
            This(f.clone().call(self.0), self.1.fmap(f))
        }
    }
}

impl<F> Fmap<F> for ShapeEnd {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}
