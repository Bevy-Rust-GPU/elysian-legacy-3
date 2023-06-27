use crate::{Alias, Circle, ExpandAlias, ExpandAliasT, Isosurface, LiftAdt, Manifold, IntoMonad};

use t_funk::{
    closure::{Closure, OutputT},
    typeclass::{
        functor::Fmap,
        semigroup::{Mappend, MappendT}, monad::Identity,
    },
};

// Ring field symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ring<T, U>(pub T, pub U);

impl<T, U, F> Fmap<F> for Ring<T, U>
where
    F: Clone + Closure<T> + Closure<U>,
{
    type Fmap = Ring<OutputT<F, T>, OutputT<F, U>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Ring(f.clone().call(self.0), f.call(self.1))
    }
}

impl<T, U> IntoMonad for Ring<T, U> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T, U> LiftAdt for Ring<T, U> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<T, U, D> ExpandAlias<D> for Ring<T, U>
where
    ExpandAliasT<Circle<T>, D>: Mappend<(Manifold, Isosurface<U>)>,
{
    type ExpandAlias = MappendT<ExpandAliasT<Circle<T>, D>, (Manifold, Isosurface<U>)>;

    fn expand_alias(self) -> Self::ExpandAlias {
        ExpandAlias::<D>::expand_alias(Circle(self.0)).mappend((Manifold, Isosurface(self.1)))
    }
}
