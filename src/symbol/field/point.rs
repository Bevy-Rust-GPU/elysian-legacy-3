use crate::{
    Distance, DistanceF32, DomainFunction, Field, Gradient, GradientF32, LiftShape, Nil,
    Position, PositionF32,
};

use t_funk::{
    function::Function,
    macros::{arrow::Arrow, category::Category, Closure},
    typeclass::functor::Fmap,
};

// Point field symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point;

impl<F> Fmap<F> for Point {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl LiftShape for Point {
    type LiftShape = Field<Self, Nil>;

    fn lift_shape(self) -> Self::LiftShape {
        Field(self, Nil)
    }
}

impl DomainFunction<DistanceF32> for Point {
    type Inputs = PositionF32;
    type Function = PointDistance;

    fn domain(self) -> Self::Function {
        PointDistance
    }
}

impl DomainFunction<GradientF32> for Point {
    type Inputs = PositionF32;
    type Function = PointGradient;

    fn domain(self) -> Self::Function {
        PointGradient
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct PointDistance;

impl Function<PositionF32> for PointDistance {
    type Output = DistanceF32;

    fn call(Position(x, y): PositionF32) -> Self::Output {
        Distance((x * x + y * y).sqrt())
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct PointGradient;

impl Function<PositionF32> for PointGradient {
    type Output = GradientF32;

    fn call(Position(x, y): Position<f32>) -> Self::Output {
        let l = (x * x + y * y).sqrt();
        Gradient(x / l, y / l)
    }
}
