use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::{Foldl, FoldlT},
};

use crate::{Field, Input, Nil, Output};

impl_adt! {
    impl<A, B, F, Z> Foldl<F, Z> for Input<A, B> | Field<A, B> | Output<A, B>
    where
        A: Foldl<F, Z>,
        B: Foldl<F, OutputT<F, (Z, A)>>,
        F: Clone + Closure<(Z, A)>,
    {
        type Foldl = FoldlT<B, F, OutputT<F, (Z, A)>>;

        fn foldl(self, f: F, z: Z) -> Self::Foldl {
            self.1.foldl(f.clone(), f.call((z, self.0)))
        }
    }
}

impl<F, Z> Foldl<F, Z> for Nil {
    type Foldl = Z;

    fn foldl(self, _f: F, z: Z) -> Self::Foldl {
        z
    }
}
