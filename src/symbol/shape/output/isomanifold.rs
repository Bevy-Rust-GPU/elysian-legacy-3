use crate::{Alias, ExpandAlias, IntoMonad, IntoTuple, IntoTupleT, IsosurfaceS, LiftAdt, ManifoldS};

use t_funk::{
    macros::{applicative::Applicative, functor::Functor, monad::Monad},
    typeclass::{
        monad::Identity,
        semigroup::{Mappend, MappendT},
    },
};

pub trait Isomanifold<T> {
    type Isomanifold;

    fn isomanifold(self, t: T) -> Self::Isomanifold;
}

impl<T, U> Isomanifold<U> for T
where
    T: IntoTuple,
    IsomanifoldS<U>: IntoTuple,
    IntoTupleT<T>: Mappend<IntoTupleT<IsomanifoldS<U>>>,
{
    type Isomanifold = MappendT<IntoTupleT<T>, IntoTupleT<IsomanifoldS<U>>>;

    fn isomanifold(self, t: U) -> Self::Isomanifold {
        self.into_tuple().mappend(IsomanifoldS(t).into_tuple())
    }
}

// Circle field symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IsomanifoldS<T>(pub T);

impl<T> IntoMonad for IsomanifoldS<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for IsomanifoldS<T> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<T, D> ExpandAlias<D> for IsomanifoldS<T> {
    type ExpandAlias = (ManifoldS, IsosurfaceS<T>);

    fn expand_alias(self) -> Self::ExpandAlias {
        (ManifoldS, IsosurfaceS(self.0))
    }
}
