use crate::{
    Distance, DistanceF, DistanceT, DomainF, DomainFunction, Field, FunctionT, Gradient,
    Isosurface, LiftAdt, Point, Position, ShapeEnd,
};

use glam::Vec2;
use t_funk::{
    closure::{Closure, Compose, Composed},
    macros::{applicative::Applicative, functor::Functor, monad::Monad},
};

// Point field symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle<T>(pub T);

impl<T> LiftAdt for Circle<T> {
    type LiftAdt = Field<Self, ShapeEnd>;

    fn lift_adt(self) -> Self::LiftAdt {
        Field(self, ShapeEnd)
    }
}

impl<T> DomainFunction<Distance<f32>> for Circle<T> {
    type Inputs = Position<Vec2>;
    type Function = Composed<DistanceT<Isosurface<T>>, DistanceT<Point>>;

    fn domain(self) -> Self::Function {
        DistanceF::default()
            .call(Point)
            .compose_l(DistanceF::default().call(Isosurface(self.0)))
    }
}

impl<T> DomainFunction<Gradient<Vec2>> for Circle<T> {
    type Inputs = Position<Vec2>;
    type Function = FunctionT<Point, Gradient<Vec2>>;

    fn domain(self) -> Self::Function {
        DomainF::<Gradient<Vec2>>::default().call(Point)
    }
}
