use crate::{Distance, DistanceF32, DomainFunction, Gradient, GradientF32, LiftShape, Nil, Output};
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

impl LiftShape for Manifold {
    type LiftShape = Output<Self, Nil>;

    fn lift_shape(self) -> Self::LiftShape {
        Output(self, Nil)
    }
}

impl DomainFunction<DistanceF32> for Manifold {
    type Inputs = DistanceF32;
    type Function = ManifoldDistance;

    fn domain(self) -> Self::Function {
        ManifoldDistance
    }
}

impl DomainFunction<GradientF32> for Manifold {
    type Inputs = (DistanceF32, GradientF32);
    type Function = ManifoldGradient;

    fn domain(self) -> Self::Function {
        ManifoldGradient
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct ManifoldDistance;

impl Function<DistanceF32> for ManifoldDistance {
    type Output = DistanceF32;

    fn call(input: DistanceF32) -> Self::Output {
        FmapF.suffix2(Abs).call(input)
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct ManifoldGradient;

impl Function<(DistanceF32, GradientF32)> for ManifoldGradient {
    type Output = GradientF32;

    fn call((Distance(d), Gradient(x, y)): (DistanceF32, GradientF32)) -> Self::Output {
        let s = d.signum();
        Gradient(x * s, y * s)
    }
}
