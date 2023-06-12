use crate::{AdtEnd, Shape, Isosurface, LiftAdt, Point, Then};

use t_funk::macros::{applicative::Applicative, functor::Functor, monad::Monad};

// Point field symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle<T>(pub T);

impl<T> LiftAdt for Circle<T> {
    type LiftAdt = Then<Shape<Point>, Then<Shape<Isosurface<T>>, AdtEnd>>;

    fn lift_adt(self) -> Self::LiftAdt {
        Then(Shape(Point), Then(Shape(Isosurface(self.0)), AdtEnd))
    }
}
