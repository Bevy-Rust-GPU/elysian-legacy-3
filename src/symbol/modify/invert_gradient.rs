use glam::Vec2;
use t_funk::{function::Neg, typeclass::functor::Fmap};

use crate::{Gradient, LiftAdt, Modify, ModifyFunction};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InvertGradient;

impl<F> Fmap<F> for InvertGradient {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl LiftAdt for InvertGradient {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<D> ModifyFunction<D> for InvertGradient {
    type Inputs = Gradient<Vec2>;
    type Moves = ();
    type Function = Neg;

    fn modify_function(self) -> Self::Function {
        Neg
    }
}
