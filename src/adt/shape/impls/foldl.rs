use t_funk::{
    closure::{Closure, OutputT},
    typeclass::foldable::Foldl,
};

use crate::Shape;

impl<A, F, Z> Foldl<F, Z> for Shape<A>
where
    F: Closure<(Z, A)>,
{
    type Foldl = OutputT<F, (Z, A)>;

    fn foldl(self, f: F, z: Z) -> Self::Foldl {
        f.call((z, self.0))
    }
}
