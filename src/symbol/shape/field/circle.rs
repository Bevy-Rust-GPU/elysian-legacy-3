use crate::{Field, Isosurface, LiftAdt, Output, Point, ShapeEnd};

use t_funk::macros::{applicative::Applicative, functor::Functor, monad::Monad};

// Point field symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle<T>(pub T);

impl<T> LiftAdt for Circle<T> {
    type LiftAdt = Field<Point, Output<Isosurface<T>, ShapeEnd>>;

    fn lift_adt(self) -> Self::LiftAdt {
        Field(Point, Output(Isosurface(self.0), ShapeEnd))
    }
}
