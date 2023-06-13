use std::ops::Mul;

use crate::{Distance, Evaluable, EvaluateFunction, Gradient, LiftAdt, LiftDomains, Run};
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
    type LiftAdt = Run<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Run(self)
    }
}

impl Evaluable for Manifold {
    type Lift = LiftDomains;
}

impl EvaluateFunction<Distance<f32>> for Manifold {
    type Inputs = Distance<f32>;
    type Moves = ();
    type Function = ManifoldDistance;

    fn evaluate_function(self) -> Self::Function {
        ManifoldDistance
    }
}

impl EvaluateFunction<Gradient<Vec2>> for Manifold {
    type Inputs = (Distance<f32>, Gradient<Vec2>);
    type Moves = ();
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
