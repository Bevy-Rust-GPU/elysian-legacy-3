use crate::{Alias, Elongate, ExpandAlias, LiftAdt, Point, IntoMonad};

use t_funk::{macros::{applicative::Applicative, functor::Functor, monad::Monad}, typeclass::monad::Identity};

// Line field symbol
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Functor,
    Applicative,
    Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Line<T>(pub T);

impl<T> IntoMonad for Line<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for Line<T> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<T, D> ExpandAlias<D> for Line<T> {
    type ExpandAlias = (Elongate<T>, Point);

    fn expand_alias(self) -> Self::ExpandAlias {
        (Elongate(self.0, true), Point)
    }
}

