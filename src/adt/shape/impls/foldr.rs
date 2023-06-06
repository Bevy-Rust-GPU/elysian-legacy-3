use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::{Foldr, FoldrT},
};

use crate::{Field, Input, Nil, Output};

impl_adt! {
    impl<A, B, F, Z> Foldr<F, Z> for Input<A, B> | Field<A, B> | Output<A, B>
    where
        B: Foldr<F, OutputT<F, (A, Z)>>,
        F: Clone + Closure<(A, Z)>,
    {
        type Foldr = FoldrT<B, F, OutputT<F, (A, Z)>>;

        fn foldr(self, f: F, z: Z) -> Self::Foldr {
            self.1.foldr(f.clone(), f.call((self.0, z)))
        }
    }
}

impl<F, Z> Foldr<F, Z> for Nil {
    type Foldr = Self;

    fn foldr(self, _f: F, _z: Z) -> Self::Foldr {
        self
    }
}
