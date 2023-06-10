use crate::{Distance, DomainFunction, Field, Gradient, LiftAdt, Position, ShapeEnd};

use glam::Vec2;
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

impl LiftAdt for Point {
    type LiftAdt = Field<Self, ShapeEnd>;

    fn lift_adt(self) -> Self::LiftAdt {
        Field(self, ShapeEnd)
    }
}

impl DomainFunction<Distance<f32>> for Point {
    type Inputs = Position<Vec2>;
    type Function = PointDistance;

    fn domain(self) -> Self::Function {
        PointDistance
    }
}

impl DomainFunction<Gradient<Vec2>> for Point {
    type Inputs = Position<Vec2>;
    type Function = PointGradient;

    fn domain(self) -> Self::Function {
        PointGradient
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct PointDistance;

impl Function<Position<Vec2>> for PointDistance {
    type Output = Distance<f32>;

    fn call(Position(p): Position<Vec2>) -> Self::Output {
        Distance(p.length())
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct PointGradient;

impl Function<Position<Vec2>> for PointGradient {
    type Output = Gradient<Vec2>;

    fn call(Position(p): Position<Vec2>) -> Self::Output {
        Gradient(p.normalize())
    }
}
