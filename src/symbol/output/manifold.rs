use std::ops::Mul;

use crate::{Distance, DomainFunction, Gradient, LiftAdt, Output};
use glam::Vec2;
use t_funk::{
    closure::{Closure, Curry2},
    function::{Abs, Function},
    macros::{arrow::Arrow, category::Category, Closure},
    typeclass::functor::{Fmap, FmapF},
};

// Isosurface output modifier symbol
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
    type LiftAdt = Output<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Output(self)
    }
}

impl DomainFunction<Distance<f32>> for Manifold {
    type Inputs = Distance<f32>;
    type Function = ManifoldDistance;

    fn domain(self) -> Self::Function {
        ManifoldDistance
    }
}

impl DomainFunction<Gradient<Vec2>> for Manifold {
    type Inputs = (Distance<f32>, Gradient<Vec2>);
    type Function = ManifoldGradient;

    fn domain(self) -> Self::Function {
        ManifoldGradient
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct ManifoldDistance;

impl Function<Distance<f32>> for ManifoldDistance {
    type Output = Distance<f32>;

    fn call(input: Distance<f32>) -> Self::Output {
        FmapF.suffix2(Abs).call(input)
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct ManifoldGradient;

impl<G> Function<(Distance<f32>, Gradient<G>)> for ManifoldGradient
where
    G: Mul<f32, Output = G>,
{
    type Output = Gradient<G>;

    fn call((Distance(d), Gradient(g)): (Distance<f32>, Gradient<G>)) -> Self::Output {
        let s = d.signum();
        Gradient(g * s)
    }
}
