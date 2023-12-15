use crate::{
    Alias, Circle, ExpandAlias, ExpandAliasT, IntoMonad, IsomanifoldS, IsosurfaceS, LiftAdt,
    ManifoldS,
};

use t_funk::{
    closure::{Closure, OutputT},
    typeclass::{
        functor::Fmap,
        monad::Identity,
        semigroup::{Mappend, MappendT},
    }, macros::lift,
};

// Ring field symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ring<T, U>(pub T, pub U);

#[lift]
pub fn make_ring<T, U>(t: T, u: U) -> Ring<T, U> {
    Ring(t, u)
}

pub fn ring() -> Ring<f32, f32> {
    Ring(1.0, 0.2)
}

impl<T, U> Ring<T, U> {
    pub fn radius<V>(self, v: V) -> Ring<V, U> {
        Ring(v, self.1)
    }

    pub fn width<V>(self, v: V) -> Ring<T, V> {
        Ring(self.0, v)
    }
}

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
    ExpandAliasT<Circle<T>, D>: Mappend<(ManifoldS, IsosurfaceS<U>)>,
{
    type ExpandAlias = MappendT<ExpandAliasT<Circle<T>, D>, ExpandAliasT<IsomanifoldS<U>, D>>;

    fn expand_alias(self) -> Self::ExpandAlias {
        ExpandAlias::<D>::expand_alias(Circle(self.0))
            .mappend(ExpandAlias::<D>::expand_alias(IsomanifoldS(self.1)))
    }
}
