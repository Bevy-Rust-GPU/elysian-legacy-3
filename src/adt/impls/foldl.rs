use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::{Foldl, FoldlT},
};

use crate::{Combine, Sequence, Unit};

impl<T, F, Z> Foldl<F, Z> for Unit<T>
where
    F: Closure<(Z, T)>,
{
    type Foldl = OutputT<F, (Z, T)>;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        f.call((z, self.0))
    }
}

impl_adt! {
    impl<A, B, C, F, Z> Foldl<F, Z> for Sequence<A, B> | Combine<A, B, C>
    where
        A: Foldl<F, Z>,
        B: Foldl<F, FoldlT<A, F, Z>>,
        F: Clone,
    {
        type Foldl = FoldlT<B, F, FoldlT<A, F, Z>>;

        fn foldl(self, f: F, z: Z) -> Self::Foldl {
            self.1.foldl(f.clone(), self.0.foldl(f, z))
        }
    }
}
