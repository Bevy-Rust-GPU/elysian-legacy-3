use t_funk::{
    closure::{Closure, OutputT},
    macros::impl_adt,
    typeclass::foldable::{Foldr, FoldrT},
};

use crate::{Combine, Sequence, Unit};

impl<T, F, Z> Foldr<F, Z> for Unit<T>
where
    F: Closure<(T, Z)>,
{
    type Foldr = OutputT<F, (T, Z)>;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        f.call((self.0, z))
    }
}

impl_adt! {
    impl<A, B, C, F, Z> Foldr<F, Z> for Sequence<A, B> | Combine<A, B, C>
    where
        A: Foldr<F, Z>,
        B: Foldr<F, FoldrT<A, F, Z>>,
        F: Clone,
    {
        type Foldr = FoldrT<B, F, FoldrT<A, F, Z>>;

        fn foldr(self, f: F, z: Z) -> Self::Foldr {
            self.1.foldr(f.clone(), self.0.foldr(f, z))
        }
    }
}
