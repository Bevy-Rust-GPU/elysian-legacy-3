use t_funk::{
    closure::{Closure, OutputT},
    typeclass::foldable::{Foldl, FoldlT},
};

use crate::{Combine, Field, Input, Modify, Output, Sequence};

impl<T, F, Z> Foldl<F, Z> for Input<T>
where
    F: Closure<(Z, T)>,
{
    type Foldl = OutputT<F, (Z, T)>;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        f.call((z, self.0))
    }
}

impl<T, F, Z> Foldl<F, Z> for Field<T>
where
    F: Closure<(Z, T)>,
{
    type Foldl = OutputT<F, (Z, T)>;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        f.call((z, self.0))
    }
}

impl<T, F, Z> Foldl<F, Z> for Output<T>
where
    F: Closure<(Z, T)>,
{
    type Foldl = OutputT<F, (Z, T)>;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        f.call((z, self.0))
    }
}

impl<T, F, Z> Foldl<F, Z> for Modify<T>
where
    F: Closure<(Z, T)>,
{
    type Foldl = OutputT<F, (Z, T)>;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        f.call((z, self.0))
    }
}

impl<T, N, F, Z> Foldl<F, Z> for Sequence<T, N>
where
    T: Foldl<F, Z>,
    N: Foldl<F, FoldlT<T, F, Z>>,
    F: Clone,
{
    type Foldl = FoldlT<N, F, FoldlT<T, F, Z>>;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        self.1.foldl(f.clone(), self.0.foldl(f, z))
    }
}

impl<A, B, F, FF, Z> Foldl<FF, Z> for Combine<A, B, F>
where
    A: Foldl<FF, Z>,
    B: Foldl<FF, FoldlT<A, FF, Z>>,
    FF: Clone,
{
    type Foldl = FoldlT<B, FF, FoldlT<A, FF, Z>>;

    fn foldl(self, f: FF, z: Z) -> Self::Foldl {
        self.1.foldl(f.clone(), self.0.foldl(f, z))
    }
}
