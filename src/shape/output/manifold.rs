use crate::{Distance, DistanceF32, Domain, Gradient, GradientF32};

use type_fields::{
    macros::{arrow::Arrow, category::Category, Closure},
    t_funk::{Abs, Curry2, Curry2B, FmapF, Function},
};

// Isosurface output modifier symbol
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Manifold;

impl Domain<DistanceF32> for Manifold {
    type Domain = Curry2B<FmapF, Abs>;

    fn domain(self) -> Self::Domain {
        FmapF.suffix2(Abs)
    }
}

impl Domain<GradientF32> for Manifold {
    type Domain = ManifoldGradient;

    fn domain(self) -> Self::Domain {
        ManifoldGradient
    }
}

#[derive(
    Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Closure, Category, Arrow,
)]
pub struct ManifoldGradient;

impl Function<(DistanceF32, GradientF32)> for ManifoldGradient {
    type Output = (DistanceF32, GradientF32);

    fn call((Distance(d), Gradient(x, y)): (DistanceF32, GradientF32)) -> Self::Output {
        let s = d.signum();
        (Distance(d), Gradient(x * s, y * s))
    }
}
