use glam::Vec2;
use t_funk::{macros::lift, typeclass::functor::Fmap};

use crate::{Distance, LiftAdt, Modify, ModifyFunction, Position};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PositionToDistance;

impl<F> Fmap<F> for PositionToDistance {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl LiftAdt for PositionToDistance {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<D> ModifyFunction<D> for PositionToDistance {
    type Inputs = Position<Vec2>;
    type Moves = Position<Vec2>;
    type Function = PositionToDistanceF;

    fn modify_function(self) -> Self::Function {
        PositionToDistanceF
    }
}

#[lift]
pub fn position_to_distance_f(_t: Position<Vec2>) -> Distance<f32> {
    Distance(0.0)
}

