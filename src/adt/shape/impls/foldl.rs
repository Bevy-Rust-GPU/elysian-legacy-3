use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::Foldl,
};

use crate::{Field, Input, Output, ShapeEnd};

impl_adt! {
    impl<A, B, F, Z> Foldl<F, Z> for Input<A, B> | Field<A, B> | Output<A, B>
    where
        F: Closure<(Z, A)>
    {
        type Foldl = OutputT<F, (Z, A)>;

        fn foldl(self, f: F, z: Z) -> Self::Foldl {
            f.call((z, self.0))
        }
    }
}

impl<F, Z> Foldl<F, Z> for ShapeEnd {
    type Foldl = Z;

    fn foldl(self, _: F, z: Z) -> Self::Foldl {
        z
    }
}
