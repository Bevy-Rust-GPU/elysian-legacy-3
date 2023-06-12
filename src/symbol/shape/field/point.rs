use crate::{Distance, DomainFunction, Field, Gradient, LiftAdt, Position, ShapeEnd};

use glam::Vec2;
use t_funk::{macros::lift, typeclass::functor::Fmap};

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
    type Moves = Position<Vec2>;
    type Function = PointDistance;

    fn domain(self) -> Self::Function {
        PointDistance
    }
}

impl DomainFunction<Gradient<Vec2>> for Point {
    type Inputs = Position<Vec2>;
    type Moves = ();
    type Function = PointGradient;

    fn domain(self) -> Self::Function {
        PointGradient
    }
}

#[lift]
pub fn point_distance(Position(p): Position<Vec2>) -> Distance<f32> {
    Distance(p.length())
}

#[lift]
pub fn point_gradient(Position(p): Position<Vec2>) -> Gradient<Vec2> {
    Gradient(p.normalize())
}
