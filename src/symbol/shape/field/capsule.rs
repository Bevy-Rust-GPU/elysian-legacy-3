use crate::{Alias, ExpandAlias, ExpandAliasT, IntoMonad, Isosurface, LiftAdt, Line};

use t_funk::{
    closure::{Closure, OutputT},
    typeclass::{
        functor::Fmap,
        monad::Identity,
        semigroup::{Mappend, MappendT},
    },
};

// Line field symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Capsule<T, U>(pub T, pub U);

impl<T, U> IntoMonad for Capsule<T, U> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T, U, F> Fmap<F> for Capsule<T, U>
where
    F: Clone + Closure<T> + Closure<U>,
{
    type Fmap = Capsule<OutputT<F, T>, OutputT<F, U>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Capsule(f.clone().call(self.0), f.call(self.1))
    }
}

impl<T, U> LiftAdt for Capsule<T, U> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<T, U, D> ExpandAlias<D> for Capsule<T, U>
where
    ExpandAliasT<Line<T>, D>: Mappend<(Isosurface<U>,)>,
{
    type ExpandAlias = MappendT<ExpandAliasT<Line<T>, D>, (Isosurface<U>,)>;

    fn expand_alias(self) -> Self::ExpandAlias {
        ExpandAlias::<D>::expand_alias(Line(self.0)).mappend((Isosurface(self.1),))
    }
}
