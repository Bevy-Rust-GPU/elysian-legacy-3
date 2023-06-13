use glam::Vec2;
use t_funk::{function::Neg, typeclass::functor::Fmap};

use crate::{EvaluateFunction, Gradient, LiftAdt, Evaluable, LiftModify, Run};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvertGradient;

impl<F> Fmap<F> for InvertGradient {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl LiftAdt for InvertGradient {
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl Evaluable for InvertGradient {
    type Lift = LiftModify;
}

impl<D> EvaluateFunction<D> for InvertGradient {
    type Inputs = Gradient<Vec2>;
    type Moves = ();
    type Function = Neg;

    fn evaluate_function(self) -> Self::Function {
        Neg
    }
}
