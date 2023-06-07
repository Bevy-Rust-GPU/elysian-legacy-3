use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::functor::Fmap,
};

use crate::{Field, Input, Nil, Output};

impl_adt! {
    impl<A, F> Fmap<F> for Input<A> | Field<A> | Output<A>
    where
        F: Clone + Closure<A>,
    {
        type Fmap = This<OutputT<F, A>>;

        fn fmap(self, f: F) -> Self::Fmap {
            This(f.clone().call(self.0))
        }
    }
}

