use std::ops::{Div, Mul};

use crate::{
    AdtEnd, Distance, Evaluable, EvaluateFunction, LiftAdt, LiftAdtT, LiftModify, Position, Run,
    Then,
};

use glam::Vec2;
use t_funk::{
    closure::{Curry2, Curry2B},
    macros::{applicative::Applicative, functor::Functor, lift, monad::Monad},
};

// Wrapper to pre-scale a Position, then post-inverse-scale a resulting Distance
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scale<S, T>(pub S, pub T);

impl<S, T> LiftAdt for Scale<S, T>
where
    S: Clone,
    T: LiftAdt,
{
    type LiftAdt =
        Then<Run<ScalePosition<S>>, Then<LiftAdtT<T>, Then<Run<InverseScaleDistance<S>>, AdtEnd>>>;

    fn lift_adt(self) -> Self::LiftAdt {
        Then(
            Run(ScalePosition(self.0.clone())),
            Then(
                self.1.lift_adt(),
                Then(Run(InverseScaleDistance(self.0)), AdtEnd),
            ),
        )
    }
}

// Position scaler
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ScalePosition<S>(pub S);

impl<S> LiftAdt for ScalePosition<S> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<S> Evaluable for ScalePosition<S> {
    type Lift = LiftModify;
}

impl<S, D> EvaluateFunction<D> for ScalePosition<S> {
    type Inputs = Position<Vec2>;
    type Moves = ();
    type Function = Curry2B<ScalePositionF, S>;

    fn evaluate_function(self) -> Self::Function {
        ScalePositionF.suffix2(self.0)
    }
}

#[lift]
pub fn scale_position_f<P, S>(Position(p): Position<P>, scale: S) -> Position<P::Output>
where
    P: Div<S>,
{
    Position(p / scale)
}

// Distance inverse scaler
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InverseScaleDistance<S>(pub S);

impl<S> LiftAdt for InverseScaleDistance<S> {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl<S> Evaluable for InverseScaleDistance<S> {
    type Lift = LiftModify;
}

impl<S, D> EvaluateFunction<D> for InverseScaleDistance<S> {
    type Inputs = Distance<f32>;
    type Moves = ();
    type Function = Curry2B<InverseScaleDistanceF, S>;

    fn evaluate_function(self) -> Self::Function {
        InverseScaleDistanceF.suffix2(self.0)
    }
}

#[lift]
pub fn inverse_scale_distance_f<D, S>(Distance(d): Distance<D>, scale: S) -> Distance<D::Output>
where
    D: Mul<S>,
{
    Distance(d * scale)
}
