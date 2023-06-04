use t_funk::{
    closure::{Closure, OutputT},
    typeclass::foldable::{Foldr, FoldrT},
};

use crate::{Combine, Field, Input, Modify, Output, Sequence};

impl<T, F, Z> Foldr<F, Z> for Input<T>
where
    F: Closure<(T, Z)>,
{
    type Foldr = OutputT<F, (T, Z)>;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        f.call((self.0, z))
    }
}

impl<T, F, Z> Foldr<F, Z> for Field<T>
where
    F: Closure<(T, Z)>,
{
    type Foldr = OutputT<F, (T, Z)>;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        f.call((self.0, z))
    }
}

impl<T, F, Z> Foldr<F, Z> for Output<T>
where
    F: Closure<(T, Z)>,
{
    type Foldr = OutputT<F, (T, Z)>;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        f.call((self.0, z))
    }
}

impl<T, F, Z> Foldr<F, Z> for Modify<T>
where
    F: Closure<(T, Z)>,
{
    type Foldr = OutputT<F, (T, Z)>;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        f.call((self.0, z))
    }
}

impl<T, N, F, Z> Foldr<F, Z> for Sequence<T, N>
where
    T: Foldr<F, Z>,
    N: Foldr<F, FoldrT<T, F, Z>>,
    F: Clone,
{
    type Foldr = FoldrT<N, F, FoldrT<T, F, Z>>;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        self.1.foldr(f.clone(), self.0.foldr(f, z))
    }
}

impl<A, B, F, FF, Z> Foldr<FF, Z> for Combine<A, B, F>
where
    A: Foldr<FF, Z>,
    B: Foldr<FF, FoldrT<A, FF, Z>>,
    FF: Clone,
{
    type Foldr = FoldrT<B, FF, FoldrT<A, FF, Z>>;

    fn foldr(self, f: FF, z: Z) -> Self::Foldr {
        self.1.foldr(f.clone(), self.0.foldr(f, z))
    }
}
