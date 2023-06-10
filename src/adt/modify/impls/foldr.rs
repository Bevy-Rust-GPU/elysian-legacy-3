use t_funk::{
    closure::{Closure, OutputT},
    typeclass::foldable::Foldr,
};

use crate::Modify;

impl<T, F, Z> Foldr<F, Z> for Modify<T>
where
    F: Closure<(T, Z)>,
{
    type Foldr = OutputT<F, (T, Z)>;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        f.call((self.0, z))
    }
}

