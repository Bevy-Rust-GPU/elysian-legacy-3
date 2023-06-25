use std::ops::Mul;

use crate::{Distance, Domains, EvaluateFunction, EvaluateInputs, Gradient, LiftAdt};
use glam::Vec2;
use t_funk::{function::Abs, macros::lift, typeclass::functor::Fmap};

// Manifold output modifier symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Manifold;

impl<F> Fmap<F> for Manifold {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl LiftAdt for Manifold {
    type LiftAdt = Domains<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Domains(self)
    }
}

impl EvaluateInputs<Distance<f32>> for Manifold {
    type Inputs = Distance<f32>;
    type Moves = ();
}

impl EvaluateFunction<Distance<f32>> for Manifold {
    type Function = ManifoldDistance;

    fn evaluate_function(self) -> Self::Function {
        ManifoldDistance
    }
}

impl EvaluateInputs<Gradient<f32>> for Manifold {
    type Inputs = (Distance<f32>, Gradient<Vec2>);
    type Moves = ();
}

impl EvaluateFunction<Gradient<Vec2>> for Manifold {
    type Function = ManifoldGradient;

    fn evaluate_function(self) -> Self::Function {
        ManifoldGradient
    }
}

#[lift]
pub fn manifold_distance(input: Distance<f32>) -> Distance<f32> {
    input.fmap(Abs)
}

#[lift]
pub fn manifold_gradient<G>((Distance(d), Gradient(g)): (Distance<f32>, Gradient<G>)) -> Gradient<G>
where
    G: Mul<f32, Output = G>,
{
    let s = d.signum();
    Gradient(g * s)
}
