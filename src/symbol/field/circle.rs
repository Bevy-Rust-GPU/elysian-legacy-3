use crate::{
    DistanceF, DistanceF32, DistanceT, DomainF, DomainFunction, Field, FunctionT, GradientF32,
    Isosurface, LiftShape, Point, PositionF32,
};

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

impl<T> LiftShape for Circle<T> {
    type LiftShape = Field<Self>;

    fn lift_shape(self) -> Self::LiftShape {
        Field(self)
    }
}

impl<T> DomainFunction<DistanceF32> for Circle<T> {
    type Inputs = PositionF32;
    type Function = Composed<DistanceT<Isosurface<T>>, DistanceT<Point>>;

    fn domain(self) -> Self::Function {
        DistanceF::default()
            .call(Point)
            .compose_l(DistanceF::default().call(Isosurface(self.0)))
    }
}

impl<T> DomainFunction<GradientF32> for Circle<T> {
    type Inputs = PositionF32;
    type Function = FunctionT<Point, GradientF32>;

    fn domain(self) -> Self::Function {
        DomainF::<GradientF32>::default().call(Point)
    }
}
