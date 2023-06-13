use glam::Vec2;
use t_funk::{macros::lift, typeclass::functor::Fmap};

use crate::{Distance, LiftAdt, EvaluateFunction, Position, Evaluable, LiftModify, Run};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PositionToDistance;

impl<F> Fmap<F> for PositionToDistance {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl LiftAdt for PositionToDistance {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl Evaluable for PositionToDistance {
    type Lift = LiftModify;
}

impl<D> EvaluateFunction<D> for PositionToDistance {
    type Inputs = Position<Vec2>;
    type Moves = Position<Vec2>;
    type Function = PositionToDistanceF;

    fn evaluate_function(self) -> Self::Function {
        PositionToDistanceF
    }
}

#[lift]
pub fn position_to_distance_f(_t: Position<Vec2>) -> Distance<f32> {
    Distance(0.0)
}

