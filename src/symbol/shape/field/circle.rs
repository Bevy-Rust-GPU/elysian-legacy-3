use crate::{Alias, ExpandAlias, Isosurface, LiftAdt, Point};

use t_funk::macros::{applicative::Applicative, functor::Functor, monad::Monad};

// Circle field symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle<T>(pub T);

impl<T> LiftAdt for Circle<T> {
    type LiftAdt = Alias<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Alias(self)
    }
}

impl<T> ExpandAlias for Circle<T> {
    type ExpandAlias = (Point, Isosurface<T>);

    fn expand_alias(self) -> Self::ExpandAlias {
        (Point, Isosurface(self.0))
    }
}
