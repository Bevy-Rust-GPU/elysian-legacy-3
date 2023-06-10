use t_funk::{
    closure::{Closure, OutputT},
    typeclass::foldable::Foldl,
};

use crate::Modify;

impl<T, F, Z> Foldl<F, Z> for Modify<T>
where
    F: Closure<(Z, T)>,
{
    type Foldl = OutputT<F, (Z, T)>;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        f.call((z, self.0))
    }
}

