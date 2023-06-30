use core::ops::Mul;

use crate::{
    Distance, Domains, EvaluateFunction, EvaluateInputs, Gradient, IntoMonad, IntoTuple,
    IntoTupleT, LiftAdt,
};
use crate::glam::Vec2;
use rust_gpu_bridge::Sign;
use t_funk::{
    function::Abs,
    macros::lift,
    typeclass::{
        functor::Fmap,
        semigroup::{Mappend, MappendT}, monad::Identity,
    },
};

pub trait Manifold {
    type Manifold;

    fn manifold(self) -> Self::Manifold;
}

impl<T> Manifold for T
where
    T: IntoTuple,
    ManifoldS: IntoTuple,
    IntoTupleT<T>: Mappend<IntoTupleT<ManifoldS>>,
{
    type Manifold = MappendT<IntoTupleT<T>, IntoTupleT<ManifoldS>>;

    fn manifold(self) -> Self::Manifold {
        self.into_tuple().mappend(ManifoldS.into_tuple())
    }
}

// Manifold output modifier symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ManifoldS;

impl IntoMonad for ManifoldS {
    type IntoMonad = Identity<Self>;

    fn into_monad(self) -> Self::IntoMonad {
        Identity(self)
    }
}

impl<F> Fmap<F> for ManifoldS {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl LiftAdt for ManifoldS {
    type LiftAdt = Domains<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Domains(self)
    }
}

impl EvaluateInputs<Distance<f32>> for ManifoldS {
    type Inputs = Distance<f32>;
    type Moves = ();
}

impl EvaluateFunction<Distance<f32>> for ManifoldS {
    type Function = ManifoldDistance;

    fn evaluate_function(self) -> Self::Function {
        ManifoldDistance
    }
}

impl EvaluateInputs<Gradient<Vec2>> for ManifoldS {
    type Inputs = (Distance<f32>, Gradient<Vec2>);
    type Moves = ();
}

impl EvaluateFunction<Gradient<Vec2>> for ManifoldS {
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
    let s = d.sign();
    Gradient(g * s)
}
