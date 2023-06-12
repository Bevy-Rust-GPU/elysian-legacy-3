use std::ops::Div;

use crate::{Distance, DomainFunction, Gradient, Input, LiftAdt, Position, ShapeEnd};

use glam::Vec2;
use t_funk::{
    closure::{Curry2, Curry2B},
    macros::{applicative::Applicative, functor::Functor, lift, monad::Monad},
};

// Scale input modifier symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Scale<S>(pub S);

impl<S> LiftAdt for Scale<S> {
    type LiftAdt = Input<Self, ShapeEnd>;

    fn lift_adt(self) -> Self::LiftAdt {
        Input(self, ShapeEnd)
    }
}

impl<S> DomainFunction<Distance<f32>> for Scale<S> {
    type Inputs = Position<Vec2>;
    type Moves = ();
    type Function = Curry2B<ScaleF, S>;

    fn domain(self) -> Self::Function {
        ScaleF.suffix2(self.0)
    }
}

impl<S> DomainFunction<Gradient<Vec2>> for Scale<S> {
    type Inputs = Position<Vec2>;
    type Moves = ();
    type Function = Curry2B<ScaleF, S>;

    fn domain(self) -> Self::Function {
        ScaleF.suffix2(self.0)
    }
}

#[lift]
pub fn scale_f<P, S>(Position(p): Position<P>, scale: S) -> Position<P::Output>
where
    P: Div<S>,
{
    Position(p / scale)
}
