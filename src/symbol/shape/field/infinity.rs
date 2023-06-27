use crate::{
    Distance, Domains, EvaluateFunction, EvaluateInputs, Gradient, IntoMonad, LiftAdt, Position,
};

use glam::Vec2;
use t_funk::{
    closure::Const,
    typeclass::{functor::Fmap, monad::Identity},
};

// Point field symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Infinity;

impl<F> Fmap<F> for Infinity {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for Infinity {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for Infinity {
    type LiftAdt = Domains<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Domains(self)
    }
}

impl EvaluateInputs<Distance<f32>> for Infinity {
    type Inputs = Position<Vec2>;
    type Moves = Position<Vec2>;
}

impl EvaluateFunction<Distance<f32>> for Infinity {
    type Function = Const<Distance<f32>>;

    fn evaluate_function(self) -> Self::Function {
        Const(Distance(f32::INFINITY))
    }
}

impl EvaluateInputs<Gradient<Vec2>> for Infinity {
    type Inputs = Position<Vec2>;
    type Moves = Position<Vec2>;
}

impl EvaluateFunction<Gradient<Vec2>> for Infinity {
    type Function = Const<Gradient<Vec2>>;

    fn evaluate_function(self) -> Self::Function {
        Const(Gradient(Vec2::ZERO))
    }
}

