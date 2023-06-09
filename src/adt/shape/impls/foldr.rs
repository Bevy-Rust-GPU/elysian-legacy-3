use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::Foldr,
};

use crate::{Field, Input, Output, ShapeEnd};

impl_adt! {
    impl<A, B, F, Z> Foldr<F, Z> for Input<A, B> | Field<A, B> | Output<A, B>
    where
        F: Closure<(A, Z)>
    {
        type Foldr = OutputT<F, (A, Z)>;

        fn foldr(self, f: F, z: Z) -> Self::Foldr {
            f.call((self.0, z))
        }
    }
}

impl<F, Z> Foldr<F, Z> for ShapeEnd {
    type Foldr = Z;

    fn foldr(self, _: F, z: Z) -> Self::Foldr {
        z
    }
}
