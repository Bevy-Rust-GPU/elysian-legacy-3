use crate::{Distance, Domains, EvaluateFunction, EvaluateInputs, Gradient, LiftAdt, Position, IntoMonad};

use glam::Vec2;
use t_funk::{macros::lift, typeclass::{functor::Fmap, monad::Identity}};

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

impl IntoMonad for Point {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for Point {
    type LiftAdt = Domains<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Domains(self)
    }
}

impl EvaluateInputs<Distance<f32>> for Point {
    type Inputs = Position<Vec2>;
    type Moves = Position<Vec2>;
}

impl EvaluateFunction<Distance<f32>> for Point {
    type Function = PointDistance;

    fn evaluate_function(self) -> Self::Function {
        PointDistance
    }
}

impl EvaluateInputs<Gradient<Vec2>> for Point {
    type Inputs = Position<Vec2>;
    type Moves = Position<Vec2>;
}

impl EvaluateFunction<Gradient<Vec2>> for Point {
    type Function = PointGradient;

    fn evaluate_function(self) -> Self::Function {
        PointGradient
    }
}

#[lift]
pub fn point_distance(Position(p): Position<Vec2>) -> Distance<f32> {
    let d = p.length();
    assert!(!d.is_nan());
    Distance(d)
}

#[lift]
pub fn point_gradient(Position(p): Position<Vec2>) -> Gradient<Vec2> {
    let g = p.normalize_or_zero();
    assert!(!g.is_nan());
    Gradient(g)
}
