use crate::{Alias, Elongate, ExpandAlias, IntoMonad, LiftAdt, Point};

use crate::glam::Vec2;
use t_funk::{
    closure::{Closure, OutputT},
    typeclass::{functor::Fmap, monad::Identity},
};

// Ring field symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect<T>(pub T);

pub fn rect2() -> Rect<Vec2> {
    Rect(Vec2::ONE)
}

impl<T> Rect<T> {
    pub fn extent<U>(self, u: U) -> Rect<U> {
        Rect(u)
    }
}

impl<T, F> Fmap<F> for Rect<T>
where
    F: Clone + Closure<T>,
{
    type Fmap = Rect<OutputT<F, T>>;

    fn fmap(self, f: F) -> Self::Fmap {
        Rect(f.clone().call(self.0))
    }
}

impl<T> IntoMonad for Rect<T> {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<T> LiftAdt for Rect<T> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<D> ExpandAlias<D> for Rect<Vec2> {
    type ExpandAlias = (Elongate<Vec2>, Elongate<Vec2>, Point);

    fn expand_alias(self) -> Self::ExpandAlias {
        (
            Elongate(Vec2::X * self.0.x, true),
            Elongate(Vec2::Y * self.0.y, true),
            Point,
        )
    }
}
