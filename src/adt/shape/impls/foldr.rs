use t_funk::{
    closure::{Closure, OutputT},
    typeclass::foldable::Foldr,
};

use crate::Shape;

impl<A, F, Z> Foldr<F, Z> for Shape<A>
where
    F: Closure<(A, Z)>,
{
    type Foldr = OutputT<F, (A, Z)>;

    fn foldr(self, f: F, z: Z) -> Self::Foldr {
        f.call((self.0, z))
    }
}
