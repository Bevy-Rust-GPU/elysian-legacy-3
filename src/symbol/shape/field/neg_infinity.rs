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
pub struct NegInfinity;

impl<F> Fmap<F> for NegInfinity {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl IntoMonad for NegInfinity {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl LiftAdt for NegInfinity {
    type LiftAdt = Domains<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Domains(self)
    }
}

impl EvaluateInputs<Distance<f32>> for NegInfinity {
    type Inputs = Position<Vec2>;
    type Moves = Position<Vec2>;
}

impl EvaluateFunction<Distance<f32>> for NegInfinity {
    type Function = Const<Distance<f32>>;

    fn evaluate_function(self) -> Self::Function {
        Const(Distance(f32::NEG_INFINITY))
    }
}

impl EvaluateInputs<Gradient<Vec2>> for NegInfinity {
    type Inputs = Position<Vec2>;
    type Moves = Position<Vec2>;
}

impl EvaluateFunction<Gradient<Vec2>> for NegInfinity {
    type Function = Const<Gradient<Vec2>>;

    fn evaluate_function(self) -> Self::Function {
        Const(Gradient(Vec2::ZERO))
    }
}

