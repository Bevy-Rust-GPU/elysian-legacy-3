use std::ops::Sub;

use crate::{Distance, EvaluateFunction, Shape, Gradient, LiftAdt, Position};

use glam::Vec2;
use t_funk::{
    closure::{Curry2, Curry2B},
    macros::{applicative::Applicative, functor::Functor, lift, monad::Monad},
};

// Translation input modifier symbol
#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Functor, Applicative, Monad,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Translate<T>(pub T);

impl<T> LiftAdt for Translate<T> {
    type LiftAdt = Shape<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Shape(self)
    }
}

impl<T> EvaluateFunction<Distance<f32>> for Translate<T> {
    type Inputs = Position<Vec2>;
    type Moves = ();
    type Function = Curry2B<TranslateF, T>;

    fn evaluate_function(self) -> Self::Function {
        TranslateF.suffix2(self.0)
    }
}

impl<T> EvaluateFunction<Gradient<Vec2>> for Translate<T> {
    type Inputs = Position<Vec2>;
    type Moves = ();
    type Function = Curry2B<TranslateF, T>;

    fn evaluate_function(self) -> Self::Function {
        TranslateF.suffix2(self.0)
    }
}

#[lift]
pub fn translate_f<P>(Position(p): Position<P>, translation: P) -> Position<P::Output>
where
    P: Sub<P>,
{
    Position(p - translation)
}
