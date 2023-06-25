use std::ops::Sub;

use crate::{EvaluateFunction, EvaluateInputs, LiftAdt, Modify, Position};

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
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<T, D> EvaluateInputs<D> for Translate<T> {
    type Inputs = Position<Vec2>;
    type Moves = ();
}

impl<T, D> EvaluateFunction<D> for Translate<T> {
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
